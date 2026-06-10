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
    if name.starts_with("HTML") || name == "Element" || name == "Node" || name == "EventTarget" || name == "Document" || name == "Attr" || name == "CharacterData" || name == "Text" || name == "Comment" || name == "DocumentType" || name == "DocumentFragment" || name == "ShadowRoot" || name == "DOMImplementation" || name == "NodeList" || name == "HTMLCollection" || name == "DOMTokenList" || name == "NamedNodeMap" || name == "Range" || name == "TreeWalker" || name == "NodeIterator" || name == "MutationObserver" || name == "MutationRecord" || name == "AbstractRange" || name == "StaticRange" {
        return "dom_core";
    }
    if name.starts_with("HTML") { return "html_elements"; }
    if name.starts_with("CSS") || name.contains("Style") || name == "StyleSheet" || name == "StyleSheetList" || name == "MediaList" || name.contains("StyleDeclaration") {
        return "css_om";
    }
    if name.starts_with("Event") || name.ends_with("Event") || name == "CustomEvent" { return "events"; }
    if name.starts_with("WebGL") || name == "WebGLRenderingContext" || name == "WebGL2RenderingContext" { return "webgl"; }
    if name.starts_with("Audio") || name.contains("AudioNode") || name == "AudioContext" || name == "OfflineAudioContext" { return "web_audio"; }
    if name.starts_with("Crypto") || name == "SubtleCrypto" { return "crypto"; }
    if name == "Request" || name == "Response" || name == "Headers" || name == "FetchEvent" || name == "FormData" { return "fetch"; }
    if name.starts_with("Worker") || name.starts_with("ServiceWorker") || name == "MessagePort" || name == "MessageChannel" { return "workers"; }
    if name.starts_with("Chrome") || name.starts_with("chrome") || name.starts_with("webkit") || name == "External" { return "chrome_extensions"; }
    if name.starts_with("Media") || name.contains("MediaStream") || name.contains("MediaDevice") || name == "Permissions" { return "media_apis"; }
    if name.ends_with("Observer") || name.starts_with("Intersection") || name.starts_with("Resize") { return "observers"; }
    "web_apis"
}
