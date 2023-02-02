use crate::types::Exclude;
use std::path::Path;

pub struct Config<'a> {
    pub excludes: Vec<Exclude<'a>>,
    pub skip_err: bool,
    pub iter_dir_rec: bool,
}

impl<'a> Config<'a> {
    pub fn is_path_excluded(&self, path: &Path) -> bool {
        self.excludes
            .iter()
            .any(|i| matches!(i, Exclude::Path(f) if path == *f))
    }
}
