use std::path::{PathBuf};

pub struct Explorer {
    pub current_dir: PathBuf,
}

impl Explorer {
    pub fn new() -> Self {
        Explorer {
            current_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"))
        }
    }
}
