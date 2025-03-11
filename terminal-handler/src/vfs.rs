use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

// Virtual filesystem structure
pub struct VirtualFS {
    pub current_dir: PathBuf,
    pub root_dir: PathBuf,
    pub files: HashMap<String, Vec<String>>,
}

impl VirtualFS {
    fn normalize_path(&self, path: &str) -> String {
        match path {
            path if path.starts_with("/") => path.to_string(),
            path if path.starts_with("./") => self
                .current_dir
                .join(path.strip_prefix("./").unwrap())
                .to_str()
                .unwrap()
                .to_string(),
            path if path.starts_with("../") => self
                .current_dir
                .parent()
                .unwrap()
                .join(path.strip_prefix("../").unwrap())
                .to_str()
                .unwrap()
                .to_string(),
            path if path.starts_with("~/") => self
                .current_dir
                .join(path.strip_prefix("~/").unwrap())
                .to_str()
                .unwrap()
                .to_string(),
            path => self.current_dir.join(path).to_str().unwrap().to_string(),
        }
    }

    fn is_dir(&self, path: &str) -> bool {
        let output = self.files.keys().find(|f| f.starts_with(path));
        if let Some(output) = output {
            output != path
        } else {
            false
        }
    }

    pub fn list_dir(&self, path: &str) -> Vec<String> {
        let path = self.normalize_path(path);

        let mut output = self
            .files
            .keys()
            .filter(|f| f.starts_with(&path))
            .map(|f| f.replace(&path, ""))
            .map(|f| {
                let mut splits = f.split('/');
                let entry = splits.next().unwrap_or("");
                if splits.next().is_some() {
                    format!("{}/", entry)
                } else {
                    entry.to_string()
                }
            })
            .collect::<HashSet<String>>()
            .into_iter()
            .collect::<Vec<_>>();

        output.sort();

        output
    }

    pub fn change_dir(&mut self, path: &str) {
        let path = self.normalize_path(path);

        if !self.is_dir(&path) {
            return;
        }

        if self.files.keys().any(|f| f.starts_with(&path)) {
            self.current_dir = PathBuf::from(path);
        }
    }

    pub fn cat_file(&self, path: &str) -> Option<impl Iterator<Item = &String>> {
        let path = self.normalize_path(path);

        self.files.get(&path).map(|content| content.iter())
    }
}
