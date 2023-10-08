use std::{cmp::Ordering, fs::FileType, ops::Div, path::PathBuf};

use catppuccin_egui::Theme;
use eframe::{
    egui::{Layout, Margin, ScrollArea, Widget},
    emath::Align,
    epaint::{vec2, Vec2},
};
use egui_extras::Size;
use egui_grid::GridBuilder;

use crate::{button_newtype, utils::fs::FileData};

pub struct DirView<'a> {
    state: &'a mut DirViewState,
    theme: &'a Theme,
}

impl<'a> DirView<'a> {
    pub fn new(state: &'a mut DirViewState, theme: &'a Theme) -> Self {
        Self { state, theme }
    }
}

impl<'a> Widget for DirView<'a> {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                // TODO
            });
            ui.separator();

            const MARGIN: f32 = 10.0;
            const SPACING: f32 = 20.0;

            ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    let available_rect = ui.available_rect_before_wrap();
                    let available_size = available_rect.size();

                    let grid_size = available_size - vec2(MARGIN, MARGIN);
                    let icon_size_vec2: Vec2 = self.state.icon_size.into();
                    let button_size = icon_size_vec2 + vec2(20.0, 35.0);
                    let num_per_row: usize = (grid_size.x + SPACING)
                        .div(icon_size_vec2.x + SPACING)
                        .floor() as usize;
                    let rows =
                        (self.state.buttons.len() as f32 / num_per_row as f32).ceil() as usize;

                    tracing::warn!("Grid Size: {grid_size:?}");
                    tracing::warn!("Icon Size: {icon_size_vec2:?}");
                    tracing::warn!("Button Size: {button_size:?}");
                    tracing::warn!(
                        "num_per_row: (gsz + s) / (is + s) = ({} + {}) / ({} + {}) = {}\nrows = {}",
                        grid_size.x,
                        SPACING,
                        icon_size_vec2.x,
                        SPACING,
                        (grid_size.x + SPACING) / (icon_size_vec2.x + SPACING),
                        self.state.buttons.len() as f32
                            / ((grid_size.x + SPACING) / (icon_size_vec2.x + SPACING))
                    );
                    tracing::warn!(
                        "Num: {}, Rows: {rows}, Columns: {num_per_row}",
                        self.state.buttons.len(),
                    );

                    let mut new_file_name: Option<PathBuf> = None;

                    (0..rows)
                        .fold(
                            GridBuilder::new()
                                .with_margin(Margin::same(MARGIN))
                                .spacing(SPACING, SPACING),
                            |grid, _row| {
                                grid.new_row(Size::Absolute {
                                    initial: button_size.y,
                                    range: (button_size.y, button_size.y),
                                })
                                .cells(
                                    Size::Absolute {
                                        initial: button_size.x,
                                        range: (button_size.x, button_size.x),
                                    },
                                    num_per_row as i32,
                                )
                            },
                        )
                        .show(ui, |mut ui| {
                            self.state.buttons.iter_mut().for_each(|button_state| {
                                ui.cell(|ui| {
                                    let _response = ui.add(DirViewButton::new(
                                        button_state,
                                        self.theme,
                                        self.state.icon_size,
                                        button_size,
                                    ));

                                    if button_state.rsfm.double_clicked() {
                                        // TODO
                                        new_file_name = Some(button_state.file_data.path().clone());
                                    }

                                    if let Some(text) = button_state.rsfm.text().clone() {
                                        if &text != button_state.file_data.name() {
                                            button_state.file_data.rename(text);
                                        }
                                    }
                                });
                            });
                        })
                })
        })
        .response
    }
}

pub struct DirViewState {
    file_data: FileData,
    buttons: Vec<DirViewButtonState>,
    icon_size: DirViewIconSize,
}

impl DirViewState {
    pub fn new(file_data: FileData) -> Self {
        let mut files = walkdir::WalkDir::new(file_data.path())
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .flat_map(|e| e.map(|e| FileData::new(e.path())))
            .collect::<Vec<_>>();
        files.sort_by(|f1, f2| {
            let f1_t: Option<FileType> = f1.file_ty();
            let f2_t: Option<FileType> = f2.file_ty();

            let cmp_nonsym_fts = |f1_t: FileType, f2_t: FileType| {
                match (f1_t.is_dir(), f2_t.is_dir()) {
                    (true, true) // isdir & isdir
                    | (false, false) // isfile & isfile
                    => f1.name().cmp(f2.name()),
                    // isdir & isfile
                    (true, false) => Ordering::Less,
                    // isfile & isdir
                    (false, true) => Ordering::Greater,
                }
            };

            match (f1_t, f2_t) {
                (None, None) => f1.name().cmp(f2.name()),
                (Some(_f1_t), None) => Ordering::Less,
                (None, Some(_f2_t)) => Ordering::Greater,
                (Some(f1_t), Some(f2_t)) => {
                    match (f1_t.is_symlink(), f2_t.is_symlink()) {
                        // sym & sym
                        (true, true) => match (f1.rec_linked_file_ty(), f2.rec_linked_file_ty()) {
                            (None, None) => f1.name().cmp(f2.name()),
                            (None, Some(_f2_t)) => Ordering::Greater,
                            (Some(_f1_t), None) => Ordering::Less,
                            (Some(f1_t), Some(f2_t)) => cmp_nonsym_fts(f1_t, f2_t),
                        },
                        // sym & nonsym
                        (true, false) => {
                            match f1.rec_linked_file_ty() {
                                Some(f1_t) => match (f1_t.is_dir(), f2_t.is_dir()) {
                                    (true, true) // symdir & isdir
                                    | (false, true) // symfile & isdir
                                    => Ordering::Greater,

                                    (true, false) // symdir & isfile
                                    | (false, false) // symfile & isfile
                                    => Ordering::Less,
                                },
                                None => Ordering::Greater,
                            }
                        }
                        // nonsym & sym
                        (false, true) => {
                            match f2.rec_linked_file_ty() {
                                Some(f2_t) => match (f1_t.is_dir(), f2_t.is_dir()) {
                                    (true, true) // symdir & isdir
                                    | (false, true) // symfile & isdir
                                    => Ordering::Less,

                                    (true, false) // symdir & isfile
                                    | (false, false) // symfile & isfile
                                    => Ordering::Greater,
                                },
                                None => Ordering::Less,
                            }
                        }
                        // nonsym & nonsym
                        (false, false) => cmp_nonsym_fts(f1_t, f2_t),
                    }
                }
            }
        });

        let buttons = files
            .into_iter()
            .map(|fd| DirViewButtonState::new(RSFMButtonState::default().with_text(fd.name()), fd))
            .collect();

        Self {
            file_data,
            buttons,
            icon_size: DirViewIconSize::Small,
        }
    }

    pub fn file_data(&self) -> &FileData {
        &self.file_data
    }
}

#[derive(Clone, Copy)]
pub enum DirViewIconSize {
    Small,
    Medium,
    Large,
}

impl DirViewIconSize {
    fn as_f32(&self) -> f32 {
        match self {
            DirViewIconSize::Small => 50.0,
            DirViewIconSize::Medium => 75.0,
            DirViewIconSize::Large => 100.0,
        }
    }
}

impl From<DirViewIconSize> for Size {
    fn from(value: DirViewIconSize) -> Self {
        let val: f32 = value.as_f32();

        Size::Absolute {
            initial: val,
            range: (val, val),
        }
    }
}

impl From<DirViewIconSize> for Vec2 {
    fn from(value: DirViewIconSize) -> Self {
        let val: f32 = value.as_f32();

        Self::new(val, val)
    }
}

button_newtype!(DirViewButton {
    wimpl: custom;
    custom_fields: [
        icon_size: DirViewIconSize,
        button_size: Vec2,
    ];
    custom_state: DirViewButtonState {
        file_data: FileData,
    };
    modifiers: [
        editable{true},
        selectable{true},

        layout{Layout::top_down(Align::Center)}
    ]
});

impl<'a> Widget for DirViewButton<'a> {
    fn ui(mut self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        let icon = self.state.file_data.icon();
        let button_size = self.button_size;

        self.to_rsfm_but()
            .with_desired_size(button_size)
            .with_icon(icon)
            .ui(ui)
    }
}
