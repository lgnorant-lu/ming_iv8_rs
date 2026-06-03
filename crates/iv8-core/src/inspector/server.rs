//! WebSocket server for CDP (Chrome DevTools Protocol).
//!
//! Runs in a background thread, accepts one DevTools client connection,
//! and exchanges CDP messages via the shared ChannelState.

use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::inspector::channel::{ChannelState, SharedChannelState, lock_channel_state};

/// Start the CDP WebSocket server on the given port.
/// Returns the shared channel state and the server URL.
pub fn start_server(port: u16) -> (SharedChannelState, String) {
    let state = Arc::new(Mutex::new(ChannelState::new()));
    let state_clone = state.clone();

    let url = format!("ws://127.0.0.1:{}/", port);
    let devtools_url = format!(
        "devtools://devtools/bundled/js_app.html?experiments=true&v8only=true&ws=127.0.0.1:{}/",
        port
    );

    thread::spawn(move || {
        run_server(port, state_clone);
    });

    tracing::info!("V8 Inspector listening on {}", url);
    tracing::info!("Open in Chrome: {}", devtools_url);
    println!("V8 Inspector: {}", devtools_url);

    (state, devtools_url)
}

fn run_server(port: u16, state: SharedChannelState) {
    let listener = match TcpListener::bind(format!("127.0.0.1:{}", port)) {
        Ok(l) => l,
        Err(e) => {
            tracing::error!("Failed to bind inspector port {}: {}", port, e);
            return;
        }
    };

    tracing::info!("Inspector server bound to port {}", port);

    // Accept one connection at a time
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let state_clone = state.clone();
                // Handle the WebSocket connection
                handle_connection(stream, state_clone);
            }
            Err(e) => {
                tracing::warn!("Inspector accept error: {}", e);
            }
        }
    }
}

fn handle_connection(stream: std::net::TcpStream, state: SharedChannelState) {
    // Simple WebSocket handshake + message loop
    // We use a minimal WebSocket implementation to avoid adding dependencies
    use std::io::{Read, Write};

    let mut stream = stream;
    stream.set_read_timeout(Some(Duration::from_millis(10))).ok();

    // Read HTTP upgrade request
    let mut buf = [0u8; 4096];
    let n = match stream.read(&mut buf) {
        Ok(n) => n,
        Err(_) => return,
    };

    let request = String::from_utf8_lossy(&buf[..n]);

    // Extract WebSocket key
    let ws_key = request.lines()
        .find(|l| l.to_lowercase().starts_with("sec-websocket-key:"))
        .and_then(|l| l.split_once(':').map(|x| x.1))
        .map(|s| s.trim().to_string());

    let ws_key = match ws_key {
        Some(k) => k,
        None => return,
    };

    // Compute accept key
    let accept_key = compute_ws_accept(&ws_key);

    // Send upgrade response
    let response = format!(
        "HTTP/1.1 101 Switching Protocols\r\n\
         Upgrade: websocket\r\n\
         Connection: Upgrade\r\n\
         Sec-WebSocket-Accept: {}\r\n\r\n",
        accept_key
    );

    if stream.write_all(response.as_bytes()).is_err() {
        return;
    }

    // Mark as connected
    {
        let mut s = lock_channel_state(&state);
        s.connected = true;
    }

    tracing::info!("DevTools client connected");

    // Message loop
    loop {
        // Send outgoing messages
        let outgoing: Vec<_> = {
            let mut s = lock_channel_state(&state);
            s.outgoing.drain(..).collect()
        };

        for msg in outgoing {
            let text = match msg {
                crate::inspector::channel::InspectorMessage::Response { message, .. } => message,
                crate::inspector::channel::InspectorMessage::Notification { message } => message,
            };
            if send_ws_text(&mut stream, &text).is_err() {
                break;
            }
        }

        // Read incoming messages
        match read_ws_text(&mut stream) {
            Ok(Some(text)) => {
                let mut s = lock_channel_state(&state);
                s.incoming.push(text);
            }
            Ok(None) => {} // timeout, no data
            Err(_) => break, // connection closed
        }

        thread::sleep(Duration::from_millis(1));
    }

    // Mark as disconnected
    {
        let mut s = lock_channel_state(&state);
        s.connected = false;
    }

    tracing::info!("DevTools client disconnected");
}

/// Compute WebSocket accept key from client key.
fn compute_ws_accept(key: &str) -> String {
    
    let magic = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";
    let combined = format!("{}{}", key, magic);

    // SHA1 hash
    let hash = sha1_bytes(combined.as_bytes());

    // Base64 encode
    base64_encode(&hash)
}

fn sha1_bytes(data: &[u8]) -> [u8; 20] {
    // Simple SHA1 implementation
    let mut h: [u32; 5] = [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0];

    let mut msg = data.to_vec();
    let orig_len = data.len() as u64 * 8;
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&orig_len.to_be_bytes());

    for chunk in msg.chunks(64) {
        let mut w = [0u32; 80];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([chunk[i*4], chunk[i*4+1], chunk[i*4+2], chunk[i*4+3]]);
        }
        for i in 16..80 {
            w[i] = (w[i-3] ^ w[i-8] ^ w[i-14] ^ w[i-16]).rotate_left(1);
        }

        let (mut a, mut b, mut c, mut d, mut e) = (h[0], h[1], h[2], h[3], h[4]);

        for (i, word) in w.iter().enumerate() {
            let (f, k) = match i {
                0..=19  => ((b & c) | (!b & d), 0x5A827999u32),
                20..=39 => (b ^ c ^ d, 0x6ED9EBA1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC),
                _       => (b ^ c ^ d, 0xCA62C1D6),
            };
            let temp = a.rotate_left(5).wrapping_add(f).wrapping_add(e).wrapping_add(k).wrapping_add(*word);
            e = d; d = c; c = b.rotate_left(30); b = a; a = temp;
        }

        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
    }

    let mut result = [0u8; 20];
    for i in 0..5 {
        result[i*4..i*4+4].copy_from_slice(&h[i].to_be_bytes());
    }
    result
}

fn base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    let mut i = 0;
    while i < data.len() {
        let b0 = data[i] as u32;
        let b1 = if i+1 < data.len() { data[i+1] as u32 } else { 0 };
        let b2 = if i+2 < data.len() { data[i+2] as u32 } else { 0 };
        let n = (b0 << 16) | (b1 << 8) | b2;
        result.push(CHARS[((n >> 18) & 63) as usize] as char);
        result.push(CHARS[((n >> 12) & 63) as usize] as char);
        result.push(if i+1 < data.len() { CHARS[((n >> 6) & 63) as usize] as char } else { '=' });
        result.push(if i+2 < data.len() { CHARS[(n & 63) as usize] as char } else { '=' });
        i += 3;
    }
    result
}

fn send_ws_text(stream: &mut std::net::TcpStream, text: &str) -> std::io::Result<()> {
    use std::io::Write;
    let data = text.as_bytes();
    let len = data.len();

    let mut frame = Vec::new();
    frame.push(0x81); // FIN + text opcode

    if len < 126 {
        frame.push(len as u8);
    } else if len < 65536 {
        frame.push(126);
        frame.push((len >> 8) as u8);
        frame.push((len & 0xFF) as u8);
    } else {
        frame.push(127);
        for i in (0..8).rev() {
            frame.push(((len >> (i * 8)) & 0xFF) as u8);
        }
    }

    frame.extend_from_slice(data);
    stream.write_all(&frame)
}

fn read_ws_text(stream: &mut std::net::TcpStream) -> std::io::Result<Option<String>> {
    use std::io::Read;

    let mut header = [0u8; 2];
    match stream.read_exact(&mut header) {
        Ok(_) => {}
        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock ||
                  e.kind() == std::io::ErrorKind::TimedOut => return Ok(None),
        Err(e) => return Err(e),
    }

    let opcode = header[0] & 0x0F;
    if opcode == 8 { // close
        return Err(std::io::Error::new(std::io::ErrorKind::ConnectionAborted, "close"));
    }

    let masked = (header[1] & 0x80) != 0;
    let mut payload_len = (header[1] & 0x7F) as usize;

    if payload_len == 126 {
        let mut ext = [0u8; 2];
        stream.read_exact(&mut ext)?;
        payload_len = ((ext[0] as usize) << 8) | ext[1] as usize;
    } else if payload_len == 127 {
        let mut ext = [0u8; 8];
        stream.read_exact(&mut ext)?;
        payload_len = usize::from_be_bytes(ext);
    }

    let mask = if masked {
        let mut m = [0u8; 4];
        stream.read_exact(&mut m)?;
        Some(m)
    } else {
        None
    };

    let mut payload = vec![0u8; payload_len];
    stream.read_exact(&mut payload)?;

    if let Some(mask) = mask {
        for (i, b) in payload.iter_mut().enumerate() {
            *b ^= mask[i % 4];
        }
    }

    Ok(Some(String::from_utf8_lossy(&payload).to_string()))
}
