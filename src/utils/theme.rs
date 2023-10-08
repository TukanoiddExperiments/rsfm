use eframe::epaint::Color32;

pub trait ThemeExt {
    fn button_fill_color(&self) -> Color32;
    fn button_border_color(&self) -> Color32;
    fn button_clicked_color(&self) -> Color32;
    fn button_hover_color(&self) -> Color32;
    fn button_selected_color(&self) -> Color32;
}

impl ThemeExt for catppuccin_egui::Theme {
    fn button_fill_color(&self) -> Color32 {
        self.surface1
    }

    fn button_border_color(&self) -> Color32 {
        self.crust
    }

    fn button_clicked_color(&self) -> Color32 {
        let [r, g, b, _a] = self.overlay2.to_array();
        Color32::from_rgba_unmultiplied(r, g, b, 100)
    }

    fn button_hover_color(&self) -> Color32 {
        let [r, g, b, _a] = self.overlay1.to_array();
        Color32::from_rgba_unmultiplied(r, g, b, 100)
    }

    fn button_selected_color(&self) -> Color32 {
        let [r, g, b, _a] = self.overlay0.to_array();
        Color32::from_rgba_unmultiplied(r, g, b, 100)
    }
}
