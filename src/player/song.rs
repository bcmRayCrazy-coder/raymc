use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaySong {
    name: String,
    path: PathBuf,
    pub cover: Option<PathBuf>,
}

impl PlaySong {
    pub fn new(name: String, path: PathBuf) -> Self {
        Self {
            name,
            path,
            cover: None,
        }
    }

    pub fn from_path(path: PathBuf) -> Self {
        Self::new(
            path.file_name()
                .expect(format!("{} is not a valid file.", path.to_string_lossy()).as_ref())
                .to_string_lossy()
                .to_string(),
            path,
        )
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}
