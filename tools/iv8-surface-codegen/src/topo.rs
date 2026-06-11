//! Topological sort — Kahn algorithm for inheritance chain ordering.

use crate::ir::Definition;
use std::collections::{HashMap, HashSet, VecDeque};

pub type DomainMap = HashMap<String, String>;

pub struct TopoResult {
    pub sorted: Vec<String>,
    pub cycles: Vec<String>,
    pub missing_parents: Vec<(String, String)>,
}

pub fn merge_and_sort(
    definitions: &[Definition],
) -> (Vec<Definition>, TopoResult) {
    // Step 1: Group by name
    let mut groups: HashMap<String, Vec<Definition>> = HashMap::new();
    let mut includes: Vec<(String, String)> = Vec::new();

    for def in definitions {
        if def.kind == "includes" {
            if let (Some(t), Some(i)) = (&def.target, &def.includes) {
                includes.push((t.clone(), i.clone()));
            }
            continue;
        }
        if let Some(name) = &def.name {
            groups.entry(name.clone()).or_default().push(def.clone());
        }
    }

    // Step 2: Merge partials
    let mut merged: HashMap<String, Definition> = HashMap::new();
    for (name, defs) in &groups {
        if defs.is_empty() { continue; }
        let mut primary = defs[0].clone();
        for extra in &defs[1..] {
            primary.members.extend(extra.members.clone());
        }
        merged.insert(name.clone(), primary);
    }

    // Step 3: Build includes map and expand mixins
    let mut includes_map: HashMap<String, Vec<String>> = HashMap::new();
    for (target, mixin) in &includes {
        includes_map.entry(target.clone()).or_default().push(mixin.clone());
    }

    let mut final_defs: Vec<Definition> = Vec::new();
    for (name, mut def) in merged {
        if def.kind == "interface_mixin" { continue; }

        if let Some(mixin_names) = includes_map.get(&name) {
            let own_names: HashSet<Option<String>> = def.members.iter()
                .map(|m| m.name.clone()).collect();

            for mixin_name in mixin_names {
                if let Some(mixin_def) = groups.get(mixin_name).and_then(|g| g.first()) {
                    for m in &mixin_def.members {
                        if m.name.is_some() && own_names.contains(&m.name) {
                            continue;
                        }
                        def.members.push(m.clone());
                    }
                }
            }
        }
        final_defs.push(def);
    }

    // Step 4: Kahn topological sort
    let all_names: HashSet<String> = final_defs.iter()
        .filter_map(|d| d.name.clone()).collect();

    let mut indegree: HashMap<String, usize> = HashMap::new();
    let mut children: HashMap<String, Vec<String>> = HashMap::new();

    for name in &all_names {
        indegree.insert(name.clone(), 0);
        children.insert(name.clone(), Vec::new());
    }

    for def in &final_defs {
        if let (Some(name), Some(parent)) = (&def.name, &def.inheritance) {
            if all_names.contains(parent) {
                children.entry(parent.clone()).or_default().push(name.clone());
                *indegree.get_mut(name).unwrap() += 1;
            }
        }
    }

    let mut queue_vec: Vec<String> = indegree.iter()
        .filter(|(_, &deg)| deg == 0)
        .map(|(n, _)| n.clone())
        .collect();
    queue_vec.sort();
    let mut queue: VecDeque<String> = queue_vec.into();

    let mut sorted: Vec<String> = Vec::new();
    while let Some(current) = queue.pop_front() {
        sorted.push(current.clone());
        if let Some(kids) = children.get(&current) {
            let mut kids_sorted = kids.clone();
            kids_sorted.sort();
            for child in &kids_sorted {
                let deg = indegree.get_mut(child).unwrap();
                *deg -= 1;
                if *deg == 0 { queue.push_back(child.clone()); }
            }
        }
    }

    let cycles: Vec<String> = indegree.iter()
        .filter(|(_, &deg)| deg > 0)
        .map(|(n, _)| n.clone())
        .collect();

    let mut missing = Vec::new();
    for def in &final_defs {
        if let (Some(name), Some(parent)) = (&def.name, &def.inheritance) {
            if !all_names.contains(parent) {
                missing.push((name.clone(), parent.clone()));
            }
        }
    }

    (final_defs, TopoResult { sorted, cycles, missing_parents: missing })
}

pub fn classify_domain(name: &str) -> &'static str {
    // SVG — ~95 interfaces
    if name.starts_with("SVG") { return "svg"; }
    // WebXR — ~49 interfaces
    if name.starts_with("XR") || name.starts_with("WebXR") { return "webxr"; }
    // HTML Elements — ~120 interfaces (must be AFTER SVG check)
    if name.starts_with("HTML") || name == "HTMLElement" { return "html_elements"; }
    // DOM Core — complex inheritance chain
    if name == "Element" || name == "Node" || name == "EventTarget" || name == "Document"
        || name == "Attr" || name == "CharacterData" || name == "Text" || name == "Comment"
        || name == "DocumentType" || name == "DocumentFragment" || name == "ShadowRoot"
        || name == "DOMImplementation" || name == "NodeList" || name == "HTMLCollection"
        || name == "DOMTokenList" || name == "NamedNodeMap" || name == "Range"
        || name == "TreeWalker" || name == "NodeIterator" || name == "MutationObserver"
        || name == "MutationRecord" || name == "AbstractRange" || name == "StaticRange"
        || name == "AbortController" || name == "AbortSignal" || name == "DOMRect"
        || name == "DOMRectReadOnly" || name == "DOMPoint" || name == "DOMPointReadOnly"
        || name == "DOMMatrix" || name == "DOMMatrixReadOnly" || name == "DOMQuad"
    { return "dom_core"; }
    // Events — ~130 interfaces
    if name.starts_with("Event") || name.ends_with("Event") || name == "CustomEvent"
        || name == "EventListener" || name == "EventTarget"
    { return "events"; }
    // CSS OM
    if name.starts_with("CSS") || name.contains("StyleSheet") || name == "MediaList"
        || name.contains("StyleDeclaration") || name == "StyleSheetList"
        || name == "Screen" || name == "VisualViewport"
    { return "css_om"; }
    // WebGL
    if name.starts_with("WebGL") || name.starts_with("WebGL2") { return "webgl"; }
    // Web Audio
    if name.starts_with("Audio") || name.contains("AudioNode") || name == "OfflineAudioContext"
        || name.contains("Oscillator") || name.contains("BiquadFilter")
        || name.contains("Delay") || name.contains("Gain") || name == "AudioContext"
        || name == "BaseAudioContext"
    { return "web_audio"; }
    // Crypto
    if name.starts_with("Crypto") || name == "SubtleCrypto" { return "crypto"; }
    // Fetch
    if name == "Request" || name == "Response" || name == "Headers" || name == "FormData"
    { return "fetch"; }
    // Workers
    if name.starts_with("Worker") || name.starts_with("ServiceWorker")
        || name == "MessagePort" || name == "MessageChannel"
    { return "workers"; }
    // Streams
    if name.starts_with("Readable") || name.starts_with("Writable")
        || name.starts_with("Transform") || name == "ByteLengthQueuingStrategy"
        || name == "CountQueuingStrategy"
    { return "streams"; }
    // Bluetooth
    if name.starts_with("Bluetooth") { return "bluetooth"; }
    // Sensors
    if name.starts_with("Sensor") || name.ends_with("Sensor")
        || name == "Accelerometer" || name == "Gyroscope" || name == "Magnetometer"
        || name == "AmbientLightSensor" || name == "AbsoluteOrientationSensor"
        || name == "RelativeOrientationSensor"
    { return "sensors"; }
    // Chrome extensions
    if name.starts_with("Chrome") || name.starts_with("chrome") || name.starts_with("webkit")
        || name == "External"
    { return "chrome_extensions"; }
    // Media
    if name.starts_with("Media") || name.contains("MediaStream") || name.contains("MediaDevice")
        || name == "Permissions" || name == "PermissionStatus"
    { return "media_apis"; }
    // Observers
    if name.ends_with("Observer") || name.starts_with("Intersection") || name.starts_with("Resize")
    { return "observers"; }
    // IndexedDB
    if name.starts_with("IDB") || name.starts_with("DOMStringList") { return "idb"; }
    // WebRTC
    if name.starts_with("RTC") || name == "RTCError" || name.starts_with("RTCPeer")
        || name.starts_with("RTCRtp") || name.starts_with("RTCData")
        || name.starts_with("RTCDtls") || name.starts_with("RTCIce")
        || name.starts_with("RTCSctp") || name == "RTCStatsReport"
    { return "webrtc"; }
    // Gamepad
    if name.starts_with("Gamepad") { return "gamepad"; }
    // GPU (WebGPU)
    if name.starts_with("GPU") || name == "GPUSupportedFeatures" { return "gpu"; }
    // USB
    if name.starts_with("USB") || name == "USBPermissionDescriptor" { return "usb"; }
    // HID
    if name.starts_with("HID") { return "hid"; }
    // MIDI
    if name.starts_with("MIDI") { return "midi"; }
    // Encoding
    if name.starts_with("TextEncoder") || name.starts_with("TextDecoder") { return "encoding"; }
    // URL
    if name == "URL" || name == "URLSearchParams" { return "url"; }
    // Payment
    if name.starts_with("Payment") || name.starts_with("Merchant")
        || name.starts_with("SecurePayment") || name == "PaymentAddress"
    { return "payment"; }
    // Presentation
    if name.starts_with("Presentation") { return "presentation"; }
    // Credential Management / WebAuthn
    if name.starts_with("Credential") || name.starts_with("PublicKeyCredential")
        || name.starts_with("Authenticator") || name == "CredentialsContainer"
    { return "credentials"; }
    // Cache
    if name.starts_with("Cache") || name == "CacheStorage" { return "cache_api"; }
    // Remaining web APIs
    if name.starts_with("Web") || name.starts_with("Navigator") || name == "Window"
        || name == "Location" || name == "History" || name == "Storage"
        || name.starts_with("XML") || name == "URL" || name == "URLSearchParams"
        || name.starts_with("WebTransport") || name.starts_with("WebSocket")
        || name.starts_with("Cache") || name == "CacheStorage"
        || name.starts_with("BackgroundFetch")
        || name.starts_with("WakeLock") || name == "BarcodeDetector"
        || name == "FaceDetector" || name.starts_with("NDEF")
        || name.starts_with("Clipboard") || name == "ClipboardItem"
        || name.starts_with("FileSystem") || name.starts_with("FileSystemWritable")
        || name.starts_with("Launch") || name.starts_with("EyeDropper")
        || name.starts_with("CookieStore") || name.starts_with("Task")
        || name.starts_with("Scheduler") || name.starts_with("Trusted")
        || name.starts_with("Sanitizer") || name.starts_with("Highlight")
        || name.starts_with("Scheduling") || name.starts_with("SharedStorage")
        || name.starts_with("Navigation") || name.starts_with("ViewTransition")
        || name.starts_with("Scroll") || name.starts_with("Speculation")
        || name.starts_with("Fence") || name.starts_with("FencedFrame")
        || name.starts_with("AdAuction") || name.starts_with("ProtectedAudience")
        || name.starts_with("Bidding") || name.starts_with("Attribution")
        || name.starts_with("InterestGroup") || name.starts_with("PrivateAggregation")
        || name.starts_with("StorageAccess") || name.starts_with("FedCM")
        || name.starts_with("Identity") && !name.starts_with("IdentityCredential")
        || name.starts_with("Digital") || name.starts_with("Multi")
        || name == "BroadcastChannel" || name == "CloseWatcher"
        || name.starts_with("Toggle") || name.starts_with("Popover")
        || name.starts_with("Command") || name.starts_with("Invoker")
        || name.starts_with("Intl") || name.starts_with("Temporal")
        || name.starts_with("WebAssembly") || name.starts_with("Wasm")
    { return "web_apis"; }
    // Final catch-all — should ideally be empty
    "web_apis"
}
