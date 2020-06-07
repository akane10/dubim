mod signature;
use std::error::Error;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
  entry
    .file_name()
    .to_str()
    .map(|s| s.starts_with(".") || s == "node_modules")
    .unwrap_or(false)
}

pub fn walking(root: &str) -> Result<Vec<(String, String)>, Box<dyn Error>> {
  let mut vec: Vec<(String, String)> = Vec::new();

  let walker = WalkDir::new(root)
    .into_iter()
    .filter_entry(|e| !is_hidden(e));

  for entry in walker.filter_map(|e| e.ok()) {
    let path = Path::new(entry.path());

    let get_pic_path = |ext: &str| {
      if ext == "jpg" || ext == "jpeg" || ext == "png" {
        path.to_str()
      } else {
        None
      }
    };

    let get_pic_sig = |y: &str| {
      let p = y.to_string();
      let s = signature::get_sig(y.to_string());

      match s {
        Ok(sig) => Some((p, sig)),
        _ => None,
      }
    };

    if path.is_file() {
      let res: Option<(String, String)> = path
        .extension()
        .and_then(|x| x.to_str())
        .and_then(get_pic_path)
        .and_then(get_pic_sig);

      match res {
        Some(val) => vec.push(val),
        None => continue,
      }
    }

    println!("path : {:?}", path);
  }
  Ok(vec)
}
