use crate::{config::Config, error::*, types::Driver};
use peview::file::PeView;
use std::{fs::File, io::Read, path::Path};

pub struct Vdf<'a> {
    config: Config<'a>,
}

impl<'a> Vdf<'a> {
    pub fn new(config: Config<'a>) -> Self {
        Self { config }
    }

    pub fn scan_dir(&self, dir: &Path) -> Result<Vec<Driver>> {
        let mut vul_drivers = Vec::new();

        self.walk_dir(dir, |f| {
            let driver = self.scan(f)?;
            if !driver.is_vulnerable() {
                vul_drivers.push(driver);
            }

            Ok(())
        })?;

        Ok(vul_drivers)
    }

    pub fn scan(&self, file: &Path) -> Result<Driver> {
        match file.try_exists() {
            Ok(false) => return Err(Error::NonExistendPath(file.into())),
            Err(e) => return Err(e.into()),
            _ => {}
        }

        match file.extension() {
            Some(v) if v == "sys" => {}
            _ => return Err(Error::InvalidExtension(file.into())),
        }

        let mut buf = Vec::new();
        File::open(file)?.read_to_end(&mut buf)?;
        let _ = PeView::parse(&buf)?;

        todo!()
    }

    fn walk_dir<F>(&self, dir: &Path, mut cb: F) -> Result<()>
    where
        F: FnMut(&Path) -> Result<()>,
    {
        for i in dir.read_dir()? {
            let entry = i?.path();
            if self.config.is_path_excluded(&entry) {
                continue;
            }

            if entry.is_dir() {
                if self.config.iter_dir_rec {
                    self.walk_dir(&entry, &mut cb)?;
                } else {
                    continue;
                }
            } else if let Err(e) = cb(&entry) {
                if self.config.skip_err {
                    continue;
                }

                return Err(e);
            }
        }

        Ok(())
    }
}
