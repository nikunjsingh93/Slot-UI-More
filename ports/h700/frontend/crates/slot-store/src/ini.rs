use std::collections::HashMap;
use std::path::Path;

pub fn read(root: &Path, file: &str) -> HashMap<String, String> {
    std::fs::read_to_string(root.join(file))
        .map(|text| parse(&text))
        .unwrap_or_default()
}

pub fn parse(text: &str) -> HashMap<String, String> {
    let mut out = HashMap::new();
    for line in text.lines() {
        let Some((key, value)) = entry(line) else {
            continue;
        };
        out.insert(key.to_string(), value.to_string());
    }
    out
}

fn entry(line: &str) -> Option<(&str, &str)> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') || line.starts_with(';') || line.starts_with('[') {
        return None;
    }
    let (key, value) = line.split_once('=')?;
    let key = key.trim();
    (!key.is_empty()).then_some((key, value.trim()))
}

pub fn value(root: &Path, file: &str, key: &str) -> Option<String> {
    read(root, file).remove(key)
}

pub fn set(text: &str, key: &str, value: Option<&str>) -> std::io::Result<String> {
    let probe = format!("{key} = {}", value.unwrap_or("x"));
    if probe.lines().count() != 1 || entry(&probe) != Some((key, value.unwrap_or("x"))) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("cannot hold the entry {probe:?}"),
        ));
    }
    let entry_line = value.map(|v| format!("{key} = {v}"));
    let mut out = String::with_capacity(text.len() + probe.len() + 1);
    let mut replaced = false;
    for line in text.lines() {
        let is_this_key = entry(line).is_some_and(|(k, _)| k == key);
        if !is_this_key {
            out.push_str(line);
            out.push('\n');
        } else if let (false, Some(new)) = (replaced, &entry_line) {
            out.push_str(new);
            out.push('\n');
            replaced = true;
        }
    }
    if let (false, Some(new)) = (replaced, &entry_line) {
        out.push_str(new);
        out.push('\n');
    }
    Ok(out)
}

pub fn write(root: &Path, file: &str, key: &str, value: &str) -> std::io::Result<()> {
    let path = root.join(file);
    let existing = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(e) => return Err(e),
    };
    let out = set(&existing, key, Some(value))
        .map_err(|e| std::io::Error::new(e.kind(), format!("{file} {e}")))?;
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    crate::atomic::atomic_write(&path, out.as_bytes())
}
