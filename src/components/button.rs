use catppuccin_egui::Theme;

use eframe::{
    egui::{
        self, vec2, Align, Frame, InnerResponse, Layout, Margin, Rounding, Sense, TextFormat, Ui,
        Vec2, WidgetInfo,
    },
    epaint::{
        text::{LayoutJob, TextWrapping},
        FontId, Stroke,
    },
};

use serde::{Deserialize, Serialize};

use crate::{
    struct_with_funcs, struct_with_into_funcs, struct_with_some_funcs,
    utils::{icons::PhosphorIcon, theme::ThemeExt},
};

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

    font_size: f32,
    text_valign: Align,
    text_halign: Align,
    text_unselected_rows: usize,
    text_selected_rows: usize,
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
            spacing: None,
            padding: Margin::symmetric(5.0, 5.0),
            rounding: Rounding::ZERO,
            border_width: None,

            font_size: 14.0,
            text_valign: Align::Center,
            text_halign: Align::Min,
            text_unselected_rows: 1,
            text_selected_rows: 1,
        }
    }
}

impl<'theme> RSFMButton<'theme> {
    struct_with_funcs![
        editable: bool,
        selectable: bool,
        layout: Layout,
        spacing: Option<f32>,
        rounding: Rounding,
        font_size: f32,
        text_valign: Align,
        text_halign: Align,
        text_unselected_rows: usize,
        text_selected_rows: usize
    ];
    struct_with_into_funcs![
        desired_size: Vec2,
        icon_size: Vec2,
        padding: Margin
    ];
    struct_with_some_funcs![icon: PhosphorIcon, border_width: f32];
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
                        let response =
                            ui.add(icon.image_widget(self.icon_size).sense(Sense::click()));
                        icon_double_clicked = response.double_clicked();

                        if icon_double_clicked {
                            tracing::warn!("Icon double clicked");
                        }

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
                                let response = ui.text_edit_multiline(text);

                                if response.lost_focus()
                                    && ui.input(|i| i.key_pressed(egui::Key::Enter))
                                {
                                    self.state.editing_text = false;
                                }
                            }
                            false => {
                                let response = ui.label({
                                    let mut layout_job = LayoutJob {
                                        halign: self.text_halign,
                                        wrap: TextWrapping {
                                            max_rows: match self.state.selected {
                                                true => self.text_selected_rows,
                                                false => self.text_unselected_rows,
                                            },
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    };
                                    layout_job.append(
                                        text,
                                        0.0,
                                        TextFormat {
                                            font_id: FontId::new(
                                                self.font_size,
                                                egui::FontFamily::Proportional,
                                            ),
                                            color: self.theme.text,
                                            valign: self.text_valign,
                                            ..Default::default()
                                        },
                                    );
                                    layout_job
                                });

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

        if !icon_double_clicked {
            self.state.double_clicked = false;
        }

        response.widget_info(|| {
            WidgetInfo::labeled(
                egui::WidgetType::Button,
                self.state.text.clone().unwrap_or("Unknown".into()),
            )
        });

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
    struct_with_funcs![
        selected: bool,
        double_clicked: bool,
        editing_text: bool
    ];

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn double_clicked(&self) -> bool {
        self.double_clicked
    }

    pub fn text(&self) -> &Option<String> {
        &self.text
    }

    pub fn selected(&self) -> bool {
        self.selected
    }

    pub fn select(&mut self) {
        self.selected = true;
    }

    pub fn deselect(&mut self) {
        self.selected = false;
    }
}

pub trait RSFMButtonDerived {
    fn to_rsfm_but(&mut self) -> RSFMButton;
}

#[macro_export]
macro_rules! button_newtype {
    ($name:ident {
        wimpl: $wimpl_ty:tt;
        $(custom_fields: [
            $(& $custom_field_name_ref:ident: $custom_field_ty_ref:path,)*
            $(&mut $custom_field_name_ref_mut:ident: $custom_field_ty_ref_mut:path,)*
            $($custom_field_name:ident: $custom_field_ty:path,)*
        ];)*
        $(custom_state: $cs_name:ident {
            $($cs_field_name:ident: $cs_field_ty:path,)*
        };)*
        $(modifiers: [$($mod:ident{$val:expr}),+])*
    }) => {
        use $crate::components::button::{RSFMButtonDerived, RSFMButton, RSFMButtonState};

        pub struct $name<'a> {
            state: &'a mut button_newtype!(state_ty; $($cs_name)*),
            theme: &'a catppuccin_egui::Theme,
            $(
                $($custom_field_name: $custom_field_ty,)*
                $($custom_field_name_ref: &'a $custom_field_ty_ref,)*
                $($custom_field_name_ref_mut: &'a mut $custom_field_ty_ref_mut,)*
            )*
        }

        impl<'a> $name<'a> {
            pub fn new(
                state: &'a mut button_newtype!(state_ty; $($cs_name)*),
                theme: &'a catppuccin_egui::Theme,
                $(
                    $($custom_field_name: $custom_field_ty,)*
                    $($custom_field_name_ref: &'a $custom_field_ty_ref,)*
                    $($custom_field_name_ref_mut: &'a mut $custom_field_ty_ref_mut,)*
                )*
            ) -> Self {
                Self {
                    state,
                    theme,
                    $(
                        $($custom_field_name,)*
                        $($custom_field_name_ref,)*
                        $($custom_field_name_ref_mut,)*
                    )*
                }
            }
        }

        impl<'a> RSFMButtonDerived for $name<'a> {
            fn to_rsfm_but(&mut self) -> RSFMButton {
                paste::paste! {
                    RSFMButton::new(&mut self.state, &self.theme)$($(.[< with_ $mod >]($val))+)*
                }
            }
        }

        $(
            pub struct $cs_name {
                rsfm: RSFMButtonState,
                $($cs_field_name: $cs_field_ty),+
            }

            impl $cs_name {
                pub fn new(rsfm: RSFMButtonState, $($cs_field_name: impl Into<$cs_field_ty>),+) -> Self {
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
        impl<'a> eframe::egui::Widget for $name<'a> {
            fn ui(mut self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
                self.to_rsfm_but().ui(ui)
            }
        }
    };
}
