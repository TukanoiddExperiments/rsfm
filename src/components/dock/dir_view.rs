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

impl<'a> DirView<'a> {
    fn select_button(&mut self, i: usize) {
        let DirViewState {
            buttons,
            current_selected_button,
            ..
        } = self.state;

        buttons[i].select();
        *current_selected_button = Some(i);
    }

    fn deselect_button(&mut self, i: usize) {
        let DirViewState {
            buttons,
            current_selected_button,
            ..
        } = self.state;

        buttons[i].deselect();

        if &Some(i) == current_selected_button {
            *current_selected_button = None;
        }
    }
}

impl<'a> Widget for DirView<'a> {
    fn ui(mut self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                // TODO
            });
            ui.separator();

            ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    const MARGIN: f32 = 10.0;
                    const SPACING: f32 = 20.0;

                    let available_rect = ui.available_rect_before_wrap();
                    let available_size = available_rect.size();

                    let grid_size = available_size - vec2(MARGIN, MARGIN);
                    let icon_size_vec2: Vec2 = self.state.icon_size.into();
                    let button_size = icon_size_vec2 + vec2(20.0, 35.0);
                    let num_per_row: usize =
                        (grid_size.x + SPACING).div(button_size.x + SPACING).floor() as usize;
                    let rows =
                        (self.state.buttons.len() as f32 / num_per_row as f32).ceil() as usize;

                    let mut new_file_name: Option<PathBuf> = None;

                    let mut new_selected_button = None;

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
                            self.state.buttons.iter_mut().enumerate().for_each(
                                |(i, button_state)| {
                                    ui.cell(|ui| {
                                        let response = ui.add(DirViewButton::new(
                                            button_state,
                                            self.theme,
                                            self.state.icon_size,
                                            button_size,
                                        ));

                                        if response.clicked() {
                                            new_selected_button = Some(i);
                                        }

                                        if button_state.rsfm.double_clicked() {
                                            // TODO
                                            new_file_name =
                                                Some(button_state.file_data.path().clone());
                                        }

                                        if let Some(text) = button_state.rsfm.text().clone() {
                                            if &text != button_state.file_data.name() {
                                                button_state.file_data.rename(text);
                                            }
                                        }
                                    });
                                },
                            );
                        });

                    if new_selected_button != self.state.current_selected_button {
                        match (self.state.current_selected_button, new_selected_button) {
                            (None, None) | (Some(_), None) => {}
                            (None, Some(new_button)) => self.select_button(new_button),
                            (Some(old_button), Some(new_button)) => {
                                self.deselect_button(old_button);
                                self.select_button(new_button);
                            }
                        }
                    }
                })
        })
        .response
    }
}

pub struct DirViewState {
    file_data: FileData,
    buttons: Vec<DirViewButtonState>,
    current_selected_button: Option<usize>,
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
            current_selected_button: None,
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
        let icon_size = self.icon_size;

        let button_size = self.button_size;

        self.to_rsfm_but()
            .with_icon_size(icon_size)
            .with_icon(icon)
            .with_desired_size(button_size)
            .with_font_size(12.0)
            .with_text_halign(Align::Center)
            .with_text_selected_rows(3)
            .ui(ui)
    }
}
