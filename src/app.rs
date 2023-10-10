use std::path::PathBuf;

use catppuccin_egui::{set_theme, Theme};
use eframe::{
    egui::{CentralPanel, Context, FontDefinitions, RichText, TopBottomPanel},
    CreationContext,
};
use egui_dock::{DockArea, DockState, NodeIndex};
use egui_phosphor::{add_to_fonts, Variant};
use egui_tracing::EventCollector;

use crate::{
    components::{
        button::RSFMButtonState,
        dock::{
            dir_view::DirViewState,
            sidebar::{SidebarButtonState, SidebarState},
            DockTab, DockTabViewer,
        },
    },
    config::Config,
    utils::{fs::FileData, icons::PhosphorIcon, paths::Paths},
};

pub struct App {
    log_event_collector: EventCollector,

    config: Config,
    paths: Paths,
    dock_state: DockState<DockTab>,
}

impl App {
    pub fn new(
        cc: &CreationContext,
        override_config: Option<PathBuf>,
        log_event_collector: EventCollector,
    ) -> Self {
        let mut fonts = FontDefinitions::default();
        add_to_fonts(&mut fonts, Variant::Regular);

        cc.egui_ctx.set_fonts(fonts);

        let config = Config::load(override_config);
        let paths = Paths::load(config.overrides().paths());

        set_theme(&cc.egui_ctx, (*config.theme()).into());

        let mut dock_state = DockState::new(vec![DockTab::Sidebar(
            SidebarState::default().with_my_comp_buttons(vec![SidebarButtonState::new(
                RSFMButtonState::default().with_text("Home"),
                paths.home_dir(),
            )]),
        )]);
        let surface = dock_state.main_surface_mut();

        let [_sidebar, dir_view] = surface.split_right(
            NodeIndex::root(),
            0.15,
            vec![DockTab::DirView(DirViewState::new(FileData::new(
                paths.home_dir(),
            )))],
        );
        let [dir_view, _terminal_log] =
            surface.split_below(dir_view, 0.8, vec![DockTab::Terminal, DockTab::Log]);
        let [_dir_view, _info] = surface.split_right(dir_view, 0.8, vec![DockTab::InfoSidebar]);

        Self {
            log_event_collector,

            paths,
            config,
            dock_state,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .button(RichText::new(PhosphorIcon::Palette.symbol()))
                    .clicked()
                {
                    self.config.switch_theme();
                    set_theme(ui.ctx(), (*self.config.theme()).into());
                }
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            DockArea::new(&mut self.dock_state).show_inside(
                ui,
                &mut DockTabViewer::new(
                    &Into::<Theme>::into(*self.config.theme()),
                    &self.log_event_collector,
                ),
            )
        });
    }
}
