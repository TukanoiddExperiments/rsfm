use std::path::PathBuf;

use crate::config::PathsOverrides;

macro_rules! paths_struct {
    ($($name:ident: $ty:ty),+) => {
        pub struct Paths {
            $($name: $ty),+
        }

        impl Paths {
            $(
                pub fn $name(&self) -> &$ty {
                    &self.$name
                }
            )+
        }
    };
}

paths_struct! {
    home_dir: PathBuf,
    downloads_dir: Option<PathBuf>,
    desktop_dir: Option<PathBuf>,
    documents_dir: Option<PathBuf>
}

impl Paths {
    pub fn load(overrides: &PathsOverrides) -> Self {
        let home_dir = dirs::home_dir().expect("Failed to find home dir");
        let downloads_dir = overrides.downloads().clone().or(dirs::download_dir());
        let desktop_dir = dirs::desktop_dir();
        let documents_dir = dirs::document_dir();

        Self {
            home_dir,
            downloads_dir,
            desktop_dir,
            documents_dir,
        }
    }
}
