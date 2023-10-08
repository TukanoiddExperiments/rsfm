use catppuccin_egui::Theme;
use eframe::{
    egui::{self, vec2, Align, Frame, InnerResponse, Layout, Margin, Rounding, Sense, Ui, Vec2},
    epaint::Stroke,
};
use serde::{Deserialize, Serialize};

use crate::utils::{icons::PhosphorIcon, theme::ThemeExt};

pub struct RSFMButton<'a> {
    state: &'a mut RSFMButtonState,
    theme: &'a Theme,

    editable: bool,
    selectable: bool,

    layout: Layout,
    desired_size: Vec2,
    icon: Option<PhosphorIcon>,
    icon_size: Vec2,
    spacing: Option<f32>,
    padding: Margin,
    rounding: Rounding,
    border_width: Option<f32>,
}

impl<'a> RSFMButton<'a> {
    pub fn new(state: &'a mut RSFMButtonState, theme: &'a Theme) -> Self {
        Self {
            state,
            theme,

            editable: false,
            selectable: false,

            layout: Layout::left_to_right(Align::Center),
            desired_size: vec2(150.0, 15.0),
            icon: None,
            icon_size: vec2(50.0, 50.0),
            spacing: Some(2.0),
            padding: Margin::symmetric(5.0, 5.0),
            rounding: Rounding::none(),
            border_width: None,
        }
    }
}

impl<'theme> RSFMButton<'theme> {
    pub fn set_editable(mut self, editable: bool) -> Self {
        self.editable = editable;
        self
    }

    pub fn with_selectable(mut self, selectable: bool) -> Self {
        self.selectable = selectable;
        self
    }

    pub fn with_layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    pub fn with_icon(mut self, icon: PhosphorIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn with_icon_size(mut self, icon_size: impl Into<Vec2>) -> Self {
        self.icon_size = icon_size.into();
        self
    }

    pub fn with_spacing(mut self, spacing: Option<f32>) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn with_padding(mut self, padding: impl Into<Margin>) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn with_rounding(mut self, rounding: Rounding) -> Self {
        self.rounding = rounding;
        self
    }

    pub fn with_border_width(mut self, border_width: f32) -> Self {
        self.border_width = Some(border_width);
        self
    }
}

macro_rules! fill_rect {
    ($self:ident, $painter:ident, $response:ident, $state:ident) => {
        paste::paste!(
            $painter.rect_filled(
                $response.rect,
                $self.rounding,
                $self.theme.[< button_ $state _color >](),
            );
        )
    };
}

impl<'theme> egui::Widget for RSFMButton<'theme> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let InnerResponse {
            inner: (icon_double_clicked, text_double_clicked),
            response,
        } = Frame::none()
            .inner_margin(self.padding)
            .fill(self.theme.button_fill_color())
            .rounding(self.rounding)
            .show(ui, |ui| {
                let mut icon_double_clicked = false;
                let mut text_double_clicked = false;

                ui.allocate_ui_with_layout(self.desired_size, self.layout, |ui| {
                    if let Some(icon) = &self.icon {
                        let response = ui.add(icon.image_widget(ui.ctx(), self.icon_size));
                        icon_double_clicked = response.double_clicked();

                        if response.clicked() {
                            self.state.editing_text = false;
                        }
                    }

                    if let Some(text) = &mut self.state.text {
                        if let Some(spacing) = self.spacing {
                            ui.add_space(spacing);
                        }

                        match self.state.editing_text {
                            true => {
                                let response = ui.text_edit_singleline(text);

                                if response.lost_focus()
                                    && ui.input(|i| i.key_pressed(egui::Key::Enter))
                                {
                                    self.state.editing_text = false;
                                }
                            }
                            false => {
                                let response = ui.label(text.clone());

                                match self.editable {
                                    true => text_double_clicked = response.double_clicked(),
                                    false => icon_double_clicked = response.double_clicked(),
                                }
                            }
                        }
                    }
                });

                (icon_double_clicked, text_double_clicked)
            });

        let response = ui.interact(
            response.rect,
            response.id,
            Sense::click().union(Sense::hover()),
        );

        let painter = ui.painter();

        if let Some(border_width) = self.border_width {
            painter.rect_stroke(
                response.rect,
                self.rounding,
                Stroke::new(border_width, self.theme.button_border_color()),
            );
        }

        if icon_double_clicked {
            self.state.double_clicked = true;
        } else if text_double_clicked && self.editable {
            self.state.editing_text = true;
        } else if response.clicked() {
            self.state.selected = self.selectable;

            fill_rect!(self, painter, response, clicked);
        } else if response.is_pointer_button_down_on() {
            fill_rect!(self, painter, response, clicked);
        } else if response.hovered() {
            fill_rect!(self, painter, response, hover);
        } else if self.state.selected {
            fill_rect!(self, painter, response, selected);
        }

        response
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct RSFMButtonState {
    selected: bool,
    double_clicked: bool,

    editing_text: bool,
    text: Option<String>,
}

impl RSFMButtonState {
    pub fn with_selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn with_double_clicked(mut self, double_clicked: bool) -> Self {
        self.double_clicked = double_clicked;
        self
    }

    pub fn with_editing_text(mut self, editing_text: bool) -> Self {
        self.editing_text = editing_text;
        self
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
}

pub trait RSFMButtonDerived {
    fn to_rsfm_but(&mut self) -> RSFMButton;
}

#[macro_export]
macro_rules! button_newtype {
    ($name:ident {
        wimpl: $wimpl_ty:tt;
        $(custom_state: $cs_name:ident {
            $($cs_field_name:ident: $cs_field_ty:path),+
        };)*
        $(modifiers: [$($mod:ident{$val:expr}),+])*
    }) => {
        pub struct $name<'a> {
            state: &'a mut button_newtype!(state_ty; $($cs_name)*),
            theme: &'a catppuccin_egui::Theme,
        }

        impl<'a> $name<'a> {
            pub fn new(state: &'a mut button_newtype!(state_ty; $($cs_name)*), theme: &'a catppuccin_egui::Theme) -> Self {
                Self {
                    state,
                    theme
                }
            }
        }

        impl<'a> $crate::components::button::RSFMButtonDerived for $name<'a> {
            fn to_rsfm_but(&mut self) -> $crate::components::button::RSFMButton {
                paste::paste! {
                    $crate::components::button::RSFMButton::new(&mut self.state, &self.theme)$($(.[< with_ $mod >]($val))+)*
                }
            }
        }

        $(
            pub struct $cs_name {
                rsfm: $crate::components::button::RSFMButtonState,
                $($cs_field_name: $cs_field_ty),+
            }

            impl $cs_name {
                pub fn new(rsfm: $crate::components::button::RSFMButtonState, $($cs_field_name: impl Into<$cs_field_ty>),+) -> Self {
                    Self {
                        rsfm,
                        $($cs_field_name: $cs_field_name.into()),+
                    }
                }
            }

            button_newtype!(wimpl; $wimpl_ty $name);

            impl std::ops::Deref for $cs_name {
                type Target = RSFMButtonState;

                fn deref(&self) -> &Self::Target {
                    &self.rsfm
                }
            }

            impl std::ops::DerefMut for $cs_name {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.rsfm
                }
            }
        )*
    };

    (state_ty;) => {
        $crate::components::button::RSFMButtonState
    };
    (state_ty; $cs_name:ident) => {
        $cs_name
    };

    (wimpl; custom $name:ident) => {};
    (wimpl; default $name:ident) => {
        use $crate::components::button::RSFMButtonDerived;

        impl<'a> eframe::egui::Widget for $name<'a> {
            fn ui(mut self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
                self.to_rsfm_but().ui(ui)
            }
        }
    };
}
