use std::{
    fs::{self, File, FileType, Metadata, OpenOptions},
    path::PathBuf,
};

use super::icons::PhosphorIcon;

pub struct FileData {
    path: PathBuf,
    name: String,
    ext: Option<String>,
    meta: Option<Metadata>,
}

impl FileData {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path: PathBuf = path.into();
        let name = path
            .file_name()
            .unwrap_or(path.as_os_str())
            .to_string_lossy()
            .to_string();
        let ext = path
            .extension()
            .map(|ext| ext.to_string_lossy().to_string());
        let meta = match path.metadata() {
            Ok(meta) => Some(meta),
            Err(err) => {
                tracing::error!("Failed to get file {path:?} metadata: {err}");
                None
            }
        };

        Self {
            path,
            name,
            ext,
            meta,
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn rename(&mut self, new_name: impl AsRef<str>) {
        let new_name = new_name.as_ref();

        let mut new_path = self.path.clone();
        new_path.set_file_name(new_name);

        match fs::rename(&self.path, &new_path) {
            Ok(_) => {
                self.path = new_path;
            }
            Err(err) => {
                // TODO: Toast
                tracing::error!("Failed to rename the file: {err}");
            }
        }
    }

    pub fn open(&self, options: &OpenOptions) -> std::io::Result<File> {
        options.open(&self.path)
    }

    pub fn linked(&self) -> Option<FileData> {
        match self.file_ty()?.is_symlink() {
            true => match fs::read_link(&self.path) {
                Ok(path) => Some(Self::new(path)),
                Err(err) => {
                    tracing::error!("Failed to get linked FileData for {:?}: {err}", &self.path);
                    None
                }
            },
            _ => None,
        }
    }

    pub fn rec_linked(&self) -> Option<FileData> {
        self.linked().map(|fd| fd.linked().unwrap_or(fd))
    }

    pub fn icon(&self) -> PhosphorIcon {
        self.file_ty()
            .map(
                |file_ty| match (file_ty.is_symlink(), file_ty.is_file(), file_ty.is_dir()) {
                    (true, false, false) => PhosphorIcon::Link,
                    (false, true, false) => PhosphorIcon::File,
                    (false, false, true) => match self.name().as_str() {
                        "Home" => PhosphorIcon::House,
                        "Desktop" => PhosphorIcon::DotsNine,
                        "Downloads" => PhosphorIcon::Download,
                        _ => PhosphorIcon::Folder,
                    },
                    _ => PhosphorIcon::SealWarning,
                },
            )
            .unwrap_or(PhosphorIcon::SealWarning)
    }

    pub fn file_ty(&self) -> Option<FileType> {
        self.meta.as_ref().map(|m| m.file_type())
    }

    pub fn rec_linked_file_ty(&self) -> Option<FileType> {
        self.rec_linked().and_then(|fd| fd.file_ty())
    }
}
