use std::{io::Write, path::PathBuf};

use serde::{Deserialize, Serialize};

pub struct Config {
    dir_path: PathBuf,
    file_path: PathBuf,
    file: ConfigFile,
}

macro_rules! ccf_getters {
    ($($name:ident: $ty:ty),+) => {
        $(
            pub fn $name(&self) -> &$ty {
                &self.file.$name
            }
        )+
    };
}

impl Config {
    pub fn load(override_path: Option<impl Into<PathBuf>>) -> Self {
        let dir_path = override_path.map(Into::into).unwrap_or(
            dirs::config_dir()
                .map(|config_dir| config_dir.join("rsfm"))
                .expect("Failed to find a config dir"),
        );

        if !dir_path.exists() {
            std::fs::create_dir_all(&dir_path).expect("Failed to create config dir");
        }

        let file_path = dir_path.join("config.toml");

        match file_path.exists() {
            true => {
                let file = toml::from_str(
                    &std::fs::read_to_string(&file_path).expect("Failed to read the config file"),
                )
                .expect("Failed to deserialize config file");

                Self {
                    dir_path,
                    file_path,
                    file,
                }
            }
            false => {
                let file = ConfigFile::default();

                let res = Self {
                    dir_path,
                    file_path,
                    file,
                };
                res.save();

                res
            }
        }
    }

    pub fn save(&self) {
        // TODO: backup

        let toml = match toml::to_string_pretty(&self.file) {
            Ok(toml) => toml,
            Err(err) => {
                tracing::error!("Failed to serialize config file: {err}");
                return;
            }
        };

        match std::fs::File::create(&self.file_path) {
            Ok(mut file) => {
                if let Err(err) = file.write(toml.as_bytes()) {
                    tracing::error!("Failed to save config to the file: {err}");
                }
            }
            Err(err) => {
                tracing::error!("Failed to create a config file: {err}")
            }
        }
    }

    pub fn switch_theme(&mut self) {
        self.file.theme.switch();
        self.save()
    }

    ccf_getters!(theme: Theme, overrides: Overrides);
}

#[derive(Default, Serialize, Deserialize)]
pub struct ConfigFile {
    theme: Theme,
    overrides: Overrides,
}

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
pub enum Theme {
    Frappe,
    Latte,
    Macchiato,
    #[default]
    Mocha,
}

impl Theme {
    pub fn switch(&mut self) {
        use Theme::*;

        *self = match self {
            Frappe => Latte,
            Latte => Macchiato,
            Macchiato => Mocha,
            Mocha => Frappe,
        }
    }
}

impl From<Theme> for catppuccin_egui::Theme {
    fn from(value: Theme) -> Self {
        match value {
            Theme::Frappe => catppuccin_egui::FRAPPE,
            Theme::Latte => catppuccin_egui::LATTE,
            Theme::Macchiato => catppuccin_egui::MACCHIATO,
            Theme::Mocha => catppuccin_egui::MOCHA,
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Overrides {
    paths: PathsOverrides,
}

impl Overrides {
    pub fn paths(&self) -> &PathsOverrides {
        &self.paths
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct PathsOverrides {
    downloads: Option<PathBuf>,
    desktop: Option<PathBuf>,
    documents: Option<PathBuf>,
}

macro_rules! po_getters {
    ($($fname:ident),+) => {
        $(
            pub fn $fname(&self) -> &Option<PathBuf> {
                &self.$fname
            }
        )+
    };
}

impl PathsOverrides {
    po_getters!(downloads, desktop, documents);
}
