use std::path::PathBuf;

use catppuccin_egui::Theme;
use eframe::egui::{RichText, Widget};

use crate::{button_newtype, utils::icons::PhosphorIcon};

pub struct Sidebar<'a> {
    state: &'a mut SidebarState,
    theme: &'a Theme,
}

impl<'a> Sidebar<'a> {
    pub fn new(state: &'a mut SidebarState, theme: &'a Theme) -> Self {
        Self { state, theme }
    }
}

impl<'a> Widget for Sidebar<'a> {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        ui.vertical(|ui| {
            ui.collapsing(
                RichText::new(format!("{} My Computer", PhosphorIcon::Desktop.symbol())),
                |ui| {
                    self.state.my_comp_buttons.iter_mut().for_each(|comp_but| {
                        ui.add(SidebarButton::new(comp_but, self.theme));
                    })
                },
            )
        })
        .response
    }
}

#[derive(Default)]
pub struct SidebarState {
    my_comp_buttons: Vec<SidebarButtonState>,
}

impl SidebarState {
    pub fn with_my_comp_buttons(mut self, my_comp_buttons: Vec<SidebarButtonState>) -> Self {
        self.my_comp_buttons = my_comp_buttons;
        self
    }

    pub fn append_comp_button(&mut self, comp_button: SidebarButtonState) {
        self.my_comp_buttons.push(comp_button);
    }
}

button_newtype!(SidebarButton {
    wimpl: default;
    custom_state: SidebarButtonState {
        path: PathBuf,
    };
});
