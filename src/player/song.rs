use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaySong {
    name: String,
    path: PathBuf,
    pub cover: Option<PathBuf>,
}

const SUPPORTED_COVER_EXT: &[&str] = &["png", "jpg", "jpeg"];
fn is_supported_cover_type(ext: &str) -> bool {
    SUPPORTED_COVER_EXT.contains(&&ext.to_lowercase().as_str())
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
            path.file_prefix()
                .expect(format!("{} is not a valid file.", path.to_string_lossy()).as_ref())
                .to_string_lossy()
                .to_string(),
            path,
        )
    }

    pub fn detect_cover(mut self) -> Self {
        let mut path = self.path.clone();
        for ext in SUPPORTED_COVER_EXT.iter() {
            if path.set_extension(ext) && path.exists() {
                self.cover = Some(path.clone());
                break;
            }
        }

        self
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}
