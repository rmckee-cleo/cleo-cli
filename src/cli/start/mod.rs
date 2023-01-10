use walkdir::{DirEntry, WalkDir};

pub fn run() {
  let folders = folders();

  // From here execute docker compose up in each folder
  println!("{:?}", folders);
}

fn is_hidden(entry: &DirEntry) -> bool {
  entry
    .file_name()
    .to_str()
    .map(|s| s.starts_with(".") && entry.depth() > 0)
    .unwrap_or(false)
}

// Fn to filter out folders that do not contain a docker-compose.yml

fn is_docker_compose(entry: &DirEntry) -> bool {
  entry
    .file_name()
    .to_str()
    .map(|s| s == "docker-compose.yml")
    .unwrap_or(false)
}

fn folders() -> Vec<String> {
  let mut folders = Vec::new();

  // Gets a list of all folders in the current directory
  for entry in WalkDir::new(".")
    .into_iter()
    // Filter out hidden directories
    .filter_entry(|e| !is_hidden(&e) && is_docker_compose(&e))
    // Filter out directories that do not contain a docker-compose.yml
    .filter_map(|v| v.ok())
  {
    if entry.file_type().is_dir() {
      folders.push(entry.path().to_str().unwrap().to_string());
      println!("{}", entry.path().display());
    }
  }
  folders
}
