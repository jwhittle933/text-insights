use walkdir::{DirEntry, WalkDir};

pub fn read_entries(data_dir: &str) -> anyhow::Result<Vec<String>> {
    let entries = std::fs::read_dir(data_dir)?;
    Ok(entries
        .into_iter()
        .filter_map(Result::ok)
        .map(|d| d.path().to_string_lossy().to_string())
        .filter(|d| !d.contains(".DS_Store"))
        .collect::<Vec<_>>())
}

pub fn read_recursive(data_dir: &str) -> anyhow::Result<Vec<String>> {
    Ok(WalkDir::new(data_dir)
        .into_iter()
        .filter_entry(is_not_hidden)
        .filter_map(Result::ok)
        .filter(is_not_dir)
        .map(to_path)
        .collect())
}

fn is_not_dir(d: &DirEntry) -> bool {
    !d.metadata().unwrap().is_dir()
}

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| entry.depth() == 0 || !s.starts_with("."))
        .unwrap_or(false)
}

fn to_path(d: DirEntry) -> String {
    d.path().to_string_lossy().to_string()
}
