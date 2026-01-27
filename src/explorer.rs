use std::path::{PathBuf};
use std::{fs, io};

pub struct Explorer {
    pub current_dir: PathBuf,
}

impl Explorer {
    pub fn new() -> Self {
        Explorer {
            current_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"))
        }
    }

    pub fn list_directory(&self) -> io::Result<()> {
        println!("Listing contents of: {}", self.current_dir.display());
        let entries = fs::read_dir(&self.current_dir)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            let file_name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown");

            let metadata = entry.metadata()?;
            let prefix = if metadata.is_dir() { "[DIR] " } else { "[FILE]" };
            let size = metadata.len();

            println!("{}{}{}", prefix, file_name, size);
        }
        Ok(())
    }

    pub fn change_directory(&mut self, target: &str) {
        let new_path = if target == ".." {
            self.current_dir.parent().map(|p| p.to_path_buf())
        } else {
            let path = self.current_dir.join(target);
            if path.is_dir() {
                Some(path)
            } else {
                println!("Error: '{}' is not a directory or does not exist.", target);
                None
            }
        };

        if let Some(path) = new_path {
            if let Ok(canonical) = fs::canonicalize(&path) {
                self.current_dir = canonical;
            } else {
                self.current_dir = path;
            }
            println!("Current directory: {}", self.current_dir.display());
        }
    }
}
