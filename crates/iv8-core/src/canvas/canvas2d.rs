//! Canvas 2D rendering backend using tiny-skia.
//!
//! Provides a Rust-side canvas that JS can draw into, then export as PNG/JPEG.
//! Text rendering uses a simple bitmap approach (no cosmic-text dependency).
//!
//! Architecture:
//! - JS canvas operations are recorded as draw commands
//! - On toDataURL(), commands are replayed into a tiny-skia Pixmap
//! - The Pixmap is encoded as PNG and base64-encoded
//!
//! Fingerprint strategy (per environment.canvas.fingerprint):
//! - If environment.canvas.fingerprint.toDataURL.png is set → return fixed value
//! - Otherwise → render with tiny-skia + optional noise

use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, Rect, Stroke, Transform};

/// A draw command recorded from JS canvas operations.
#[derive(Debug, Clone)]
pub enum DrawCmd {
    FillRect { x: f32, y: f32, w: f32, h: f32, color: [u8; 4] },
    StrokeRect { x: f32, y: f32, w: f32, h: f32, color: [u8; 4], line_width: f32 },
    ClearRect { x: f32, y: f32, w: f32, h: f32 },
    FillText { text: String, x: f32, y: f32, color: [u8; 4], font_size: f32 },
    Arc { x: f32, y: f32, r: f32, start: f32, end: f32, color: [u8; 4], fill: bool },
    LineTo { x: f32, y: f32 },
    MoveTo { x: f32, y: f32 },
    BeginPath,
    ClosePath,
    Fill { color: [u8; 4] },
    Stroke { color: [u8; 4], line_width: f32 },
    Save,
    Restore,
    SetTransform { a: f32, b: f32, c: f32, d: f32, e: f32, f: f32 },
    Translate { x: f32, y: f32 },
    Scale { x: f32, y: f32 },
    Rotate { angle: f32 },
}

/// Canvas 2D state.
pub struct Canvas2D {
    pub width: u32,
    pub height: u32,
    pub commands: Vec<DrawCmd>,
    // Current state
    pub fill_color: [u8; 4],
    pub stroke_color: [u8; 4],
    pub line_width: f32,
    pub font_size: f32,
    pub global_alpha: f32,
    // Path state
    pub path_points: Vec<(f32, f32)>,
    pub path_started: bool,
    // State stack
    pub state_stack: Vec<CanvasState>,
}

#[derive(Clone)]
pub struct CanvasState {
    pub fill_color: [u8; 4],
    pub stroke_color: [u8; 4],
    pub line_width: f32,
    pub font_size: f32,
    pub global_alpha: f32,
}

impl Canvas2D {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: width.max(1),
            height: height.max(1),
            commands: Vec::new(),
            fill_color: [0, 0, 0, 255],
            stroke_color: [0, 0, 0, 255],
            line_width: 1.0,
            font_size: 10.0,
            global_alpha: 1.0,
            path_points: Vec::new(),
            path_started: false,
            state_stack: Vec::new(),
        }
    }

    /// Parse a CSS color string to RGBA bytes.
    pub fn parse_color(s: &str) -> [u8; 4] {
        let s = s.trim();
        // #rrggbb
        if s.starts_with('#') && s.len() == 7 {
            let r = u8::from_str_radix(&s[1..3], 16).unwrap_or(0);
            let g = u8::from_str_radix(&s[3..5], 16).unwrap_or(0);
            let b = u8::from_str_radix(&s[5..7], 16).unwrap_or(0);
            return [r, g, b, 255];
        }
        // #rgb
        if s.starts_with('#') && s.len() == 4 {
            let r = u8::from_str_radix(&s[1..2].repeat(2), 16).unwrap_or(0);
            let g = u8::from_str_radix(&s[2..3].repeat(2), 16).unwrap_or(0);
            let b = u8::from_str_radix(&s[3..4].repeat(2), 16).unwrap_or(0);
            return [r, g, b, 255];
        }
        // rgb(r,g,b)
        if s.starts_with("rgb(") && s.ends_with(')') {
            let inner = &s[4..s.len()-1];
            let parts: Vec<&str> = inner.split(',').collect();
            if parts.len() == 3 {
                let r = parts[0].trim().parse::<u8>().unwrap_or(0);
                let g = parts[1].trim().parse::<u8>().unwrap_or(0);
                let b = parts[2].trim().parse::<u8>().unwrap_or(0);
                return [r, g, b, 255];
            }
        }
        // rgba(r,g,b,a)
        if s.starts_with("rgba(") && s.ends_with(')') {
            let inner = &s[5..s.len()-1];
            let parts: Vec<&str> = inner.split(',').collect();
            if parts.len() == 4 {
                let r = parts[0].trim().parse::<u8>().unwrap_or(0);
                let g = parts[1].trim().parse::<u8>().unwrap_or(0);
                let b = parts[2].trim().parse::<u8>().unwrap_or(0);
                let a = (parts[3].trim().parse::<f32>().unwrap_or(1.0) * 255.0) as u8;
                return [r, g, b, a];
            }
        }
        // Named colors
        match s {
            "black" => [0, 0, 0, 255],
            "white" => [255, 255, 255, 255],
            "red" => [255, 0, 0, 255],
            "green" => [0, 128, 0, 255],
            "blue" => [0, 0, 255, 255],
            "yellow" => [255, 255, 0, 255],
            "cyan" | "aqua" => [0, 255, 255, 255],
            "magenta" | "fuchsia" => [255, 0, 255, 255],
            "orange" => [255, 165, 0, 255],
            "purple" => [128, 0, 128, 255],
            "gray" | "grey" => [128, 128, 128, 255],
            "transparent" => [0, 0, 0, 0],
            _ => [0, 0, 0, 255],
        }
    }

    /// Parse font string to extract font size.
    pub fn parse_font_size(font: &str) -> f32 {
        // "14px Arial", "bold 12pt sans-serif", etc.
        for part in font.split_whitespace() {
            if let Some(px) = part.strip_suffix("px") {
                if let Ok(v) = px.parse::<f32>() { return v; }
            }
            if let Some(pt) = part.strip_suffix("pt") {
                if let Ok(v) = pt.parse::<f32>() { return v * 1.333; }
            }
            if let Some(em) = part.strip_suffix("em") {
                if let Ok(v) = em.parse::<f32>() { return v * 16.0; }
            }
        }
        10.0
    }

    /// Render all commands to a PNG byte vector.
    pub fn to_png(&self, noise_seed: Option<u64>) -> Vec<u8> {
        let mut pixmap = match Pixmap::new(self.width, self.height) {
            Some(p) => p,
            None => return Vec::new(),
        };

        // Fill with transparent background
        pixmap.fill(Color::TRANSPARENT);

        for cmd in &self.commands {
            self.render_cmd(&mut pixmap, cmd);
        }

        // Apply fingerprint noise if requested
        if let Some(seed) = noise_seed {
            apply_noise(&mut pixmap, seed);
        }

        // Encode as PNG using the `png` crate
        encode_pixmap_to_png(&pixmap)
    }

    fn render_cmd(&self, pixmap: &mut Pixmap, cmd: &DrawCmd) {
        match cmd {
            DrawCmd::FillRect { x, y, w, h, color } => {
                if let Some(rect) = Rect::from_xywh(*x, *y, *w, *h) {
                    let mut paint = Paint::default();
                    paint.set_color_rgba8(color[0], color[1], color[2], color[3]);
                    pixmap.fill_rect(rect, &paint, Transform::identity(), None);
                }
            }
            DrawCmd::StrokeRect { x, y, w, h, color, line_width } => {
                let mut pb = PathBuilder::new();
                pb.move_to(*x, *y);
                pb.line_to(*x + *w, *y);
                pb.line_to(*x + *w, *y + *h);
                pb.line_to(*x, *y + *h);
                pb.close();
                if let Some(path) = pb.finish() {
                    let mut paint = Paint::default();
                    paint.set_color_rgba8(color[0], color[1], color[2], color[3]);
                    let stroke = Stroke { width: *line_width, ..Default::default() };
                    pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
                }
            }
            DrawCmd::ClearRect { x, y, w, h } => {
                if let Some(rect) = Rect::from_xywh(*x, *y, *w, *h) {
                    let mut paint = Paint::default();
                    paint.set_color(Color::TRANSPARENT);
                    paint.blend_mode = tiny_skia::BlendMode::Clear;
                    pixmap.fill_rect(rect, &paint, Transform::identity(), None);
                }
            }
            DrawCmd::FillText { text, x, y, color, font_size } => {
                // Simple bitmap text: draw a colored rectangle as text placeholder
                // Real text rendering would need cosmic-text
                let char_w = font_size * 0.6;
                let text_w = text.len() as f32 * char_w;
                let text_h = *font_size;
                if let Some(_rect) = Rect::from_xywh(*x, *y - text_h * 0.8, text_w, text_h) {
                    let mut paint = Paint::default();
                    paint.set_color_rgba8(color[0], color[1], color[2], color[3]);
                    // Draw individual character blocks for more realistic fingerprint
                    for (i, _ch) in text.chars().enumerate() {
                        let cx = *x + i as f32 * char_w;
                        if let Some(cr) = Rect::from_xywh(cx, *y - text_h * 0.8, char_w * 0.85, text_h * 0.9) {
                            pixmap.fill_rect(cr, &paint, Transform::identity(), None);
                        }
                    }
                }
            }
            DrawCmd::Arc { x, y, r, start, end, color, fill } => {
                let mut pb = PathBuilder::new();
                // Approximate arc with line segments
                let steps = 32;
                let angle_step = (*end - *start) / steps as f32;
                let sx = *x + r * start.cos();
                let sy = *y + r * start.sin();
                pb.move_to(sx, sy);
                for i in 1..=steps {
                    let angle = *start + angle_step * i as f32;
                    pb.line_to(*x + r * angle.cos(), *y + r * angle.sin());
                }
                if *fill { pb.close(); }
                if let Some(path) = pb.finish() {
                    let mut paint = Paint::default();
                    paint.set_color_rgba8(color[0], color[1], color[2], color[3]);
                    if *fill {
                        pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);
                    } else {
                        let stroke = Stroke { width: 1.0, ..Default::default() };
                        pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
                    }
                }
            }
            DrawCmd::Fill { color }
                // Fill current path
                if self.path_points.len() >= 2 => {
                    let mut pb = PathBuilder::new();
                    pb.move_to(self.path_points[0].0, self.path_points[0].1);
                    for &(px, py) in &self.path_points[1..] {
                        pb.line_to(px, py);
                    }
                    pb.close();
                    if let Some(path) = pb.finish() {
                        let mut paint = Paint::default();
                        paint.set_color_rgba8(color[0], color[1], color[2], color[3]);
                        pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);
                    }
                }
            DrawCmd::Stroke { color, line_width }
                if self.path_points.len() >= 2 => {
                    let mut pb = PathBuilder::new();
                    pb.move_to(self.path_points[0].0, self.path_points[0].1);
                    for &(px, py) in &self.path_points[1..] {
                        pb.line_to(px, py);
                    }
                    if let Some(path) = pb.finish() {
                        let mut paint = Paint::default();
                        paint.set_color_rgba8(color[0], color[1], color[2], color[3]);
                        let stroke = Stroke { width: *line_width, ..Default::default() };
                        pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
                    }
                }
            _ => {} // Save/Restore/Transform handled at state level
        }
    }

    /// Get pixel data as RGBA bytes (for getImageData).
    pub fn get_image_data(&self, x: u32, y: u32, w: u32, h: u32) -> Vec<u8> {
        let png_data = self.to_png(None);
        if png_data.is_empty() {
            return vec![0u8; (w * h * 4) as usize];
        }

        // Decode PNG to get pixel data
        if let Some(pixmap) = decode_png_to_pixmap(&png_data) {
            let mut result = Vec::with_capacity((w * h * 4) as usize);
            for py in y..y.saturating_add(h).min(pixmap.height()) {
                for px in x..x.saturating_add(w).min(pixmap.width()) {
                    if let Some(pixel) = pixmap.pixel(px, py) {
                        let c = pixel.demultiply();
                        result.push(c.red());
                        result.push(c.green());
                        result.push(c.blue());
                        result.push(c.alpha());
                    } else {
                        result.extend_from_slice(&[0, 0, 0, 0]);
                    }
                }
            }
            // Pad if needed
            let expected = (w * h * 4) as usize;
            while result.len() < expected {
                result.push(0);
            }
            result
        } else {
            vec![0u8; (w * h * 4) as usize]
        }
    }
}

/// Encode a Pixmap to PNG bytes using the `png` crate.
pub fn encode_pixmap_to_png(pixmap: &Pixmap) -> Vec<u8> {
    let mut buf = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut buf, pixmap.width(), pixmap.height());
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = match encoder.write_header() {
            Ok(w) => w,
            Err(_) => return Vec::new(),
        };
        // tiny-skia stores pixels as premultiplied RGBA
        // We need to demultiply before encoding
        let mut rgba_data = Vec::with_capacity((pixmap.width() * pixmap.height() * 4) as usize);
        for pixel in pixmap.pixels() {
            let c = pixel.demultiply();
            rgba_data.push(c.red());
            rgba_data.push(c.green());
            rgba_data.push(c.blue());
            rgba_data.push(c.alpha());
        }
        if writer.write_image_data(&rgba_data).is_err() {
            return Vec::new();
        }
    }
    buf
}

/// Decode PNG bytes to a Pixmap.
pub fn decode_png_to_pixmap(data: &[u8]) -> Option<Pixmap> {
    let decoder = png::Decoder::new(std::io::Cursor::new(data));
    let mut reader = decoder.read_info().ok()?;
    let mut img_data = vec![0u8; reader.output_buffer_size()];
    let info = reader.next_frame(&mut img_data).ok()?;

    let width = info.width;
    let height = info.height;
    let mut pixmap = Pixmap::new(width, height)?;

    // Convert to premultiplied RGBA
    let pixels = pixmap.pixels_mut();
    match info.color_type {
        png::ColorType::Rgba => {
            for (i, pixel) in pixels.iter_mut().enumerate() {
                let base = i * 4;
                if base + 3 < img_data.len() {
                    let r = img_data[base];
                    let g = img_data[base + 1];
                    let b = img_data[base + 2];
                    let a = img_data[base + 3];
                    if let Ok(p) = tiny_skia::ColorU8::from_rgba(r, g, b, a).premultiply().try_into() {
                        *pixel = p;
                    }
                }
            }
        }
        png::ColorType::Rgb => {
            for (i, pixel) in pixels.iter_mut().enumerate() {
                let base = i * 3;
                if base + 2 < img_data.len() {
                    let r = img_data[base];
                    let g = img_data[base + 1];
                    let b = img_data[base + 2];
                    if let Ok(p) = tiny_skia::ColorU8::from_rgba(r, g, b, 255).premultiply().try_into() {
                        *pixel = p;
                    }
                }
            }
        }
        _ => {}
    }
    Some(pixmap)
}

/// Apply deterministic noise to a pixmap for fingerprint variation.
fn apply_noise(pixmap: &mut Pixmap, seed: u64) {
    // Simple LCG-based noise: flip a few pixels
    let mut rng = seed;
    let w = pixmap.width();
    let h = pixmap.height();
    if w == 0 || h == 0 { return; }

    let pixels = pixmap.pixels_mut();
    let noise_count = 3; // Flip 3 pixels for subtle variation
    for _ in 0..noise_count {
        rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let idx = (rng >> 33) as usize % pixels.len();
        let pixel = &mut pixels[idx];
        // Flip the least significant bit of red channel
        let c = pixel.demultiply();
        let new_r = c.red() ^ 1;
        if let Some(new_pixel) = tiny_skia::PremultipliedColorU8::from_rgba(
            new_r, c.green(), c.blue(), c.alpha()
        ) {
            *pixel = new_pixel;
        }
    }
}

/// Encode bytes as base64.
pub fn base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::with_capacity(data.len().div_ceil(3) * 4);
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let combined = (b0 << 16) | (b1 << 8) | b2;
        result.push(CHARS[((combined >> 18) & 63) as usize] as char);
        result.push(CHARS[((combined >> 12) & 63) as usize] as char);
        result.push(if chunk.len() > 1 { CHARS[((combined >> 6) & 63) as usize] as char } else { '=' });
        result.push(if chunk.len() > 2 { CHARS[(combined & 63) as usize] as char } else { '=' });
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_color_hex() {
        assert_eq!(Canvas2D::parse_color("#ff0000"), [255, 0, 0, 255]);
        assert_eq!(Canvas2D::parse_color("#000000"), [0, 0, 0, 255]);
        assert_eq!(Canvas2D::parse_color("#ffffff"), [255, 255, 255, 255]);
    }

    #[test]
    fn test_parse_color_rgb() {
        assert_eq!(Canvas2D::parse_color("rgb(255, 0, 0)"), [255, 0, 0, 255]);
        assert_eq!(Canvas2D::parse_color("rgba(0, 0, 255, 0.5)"), [0, 0, 255, 127]);
    }

    #[test]
    fn test_parse_color_named() {
        assert_eq!(Canvas2D::parse_color("black"), [0, 0, 0, 255]);
        assert_eq!(Canvas2D::parse_color("white"), [255, 255, 255, 255]);
        assert_eq!(Canvas2D::parse_color("red"), [255, 0, 0, 255]);
    }

    #[test]
    fn test_parse_font_size() {
        assert!((Canvas2D::parse_font_size("14px Arial") - 14.0).abs() < 0.01);
        assert!((Canvas2D::parse_font_size("12pt sans-serif") - 16.0).abs() < 0.1);
        assert!((Canvas2D::parse_font_size("bold 16px monospace") - 16.0).abs() < 0.01);
    }

    #[test]
    fn test_canvas_fill_rect_to_png() {
        let mut canvas = Canvas2D::new(100, 100);
        canvas.commands.push(DrawCmd::FillRect {
            x: 0.0, y: 0.0, w: 50.0, h: 50.0,
            color: [255, 0, 0, 255],
        });
        let png = canvas.to_png(None);
        assert!(!png.is_empty());
        // PNG magic bytes
        assert_eq!(&png[0..4], &[0x89, 0x50, 0x4E, 0x47]);
    }

    #[test]
    fn test_canvas_to_data_url() {
        let canvas = Canvas2D::new(10, 10);
        let png = canvas.to_png(None);
        let b64 = base64_encode(&png);
        let data_url = format!("data:image/png;base64,{}", b64);
        assert!(data_url.starts_with("data:image/png;base64,"));
        assert!(data_url.len() > 30);
    }

    #[test]
    fn test_noise_produces_different_output() {
        let canvas = Canvas2D::new(50, 50);
        let png1 = canvas.to_png(Some(12345));
        let png2 = canvas.to_png(Some(99999));
        // Different seeds should produce different output (with high probability)
        // Note: for empty canvas this might be the same, so we add a rect
        let mut canvas2 = Canvas2D::new(50, 50);
        canvas2.commands.push(DrawCmd::FillRect {
            x: 0.0, y: 0.0, w: 50.0, h: 50.0,
            color: [100, 150, 200, 255],
        });
        let png3 = canvas2.to_png(Some(12345));
        let png4 = canvas2.to_png(Some(99999));
        assert_ne!(png3, png4);
    }

    #[test]
    fn test_base64_encode() {
        // "Man" → "TWFu"
        assert_eq!(base64_encode(b"Man"), "TWFu");
        // "Ma" → "TWE="
        assert_eq!(base64_encode(b"Ma"), "TWE=");
        // "M" → "TQ=="
        assert_eq!(base64_encode(b"M"), "TQ==");
    }
}
