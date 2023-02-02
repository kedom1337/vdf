use std::path::Path;

pub enum Exclude<'a> {
    Vendor(&'a str),
    Path(&'a Path),
}

pub struct Driver<'a> {
    pub file: &'a Path,
    pub hits: Vec<Vulnerability<'a>>,
}

impl<'a> Driver<'a> {
    pub fn is_vulnerable(&self) -> bool {
        !self.hits.is_empty()
    }
}
