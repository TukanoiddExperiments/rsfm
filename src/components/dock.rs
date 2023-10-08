pub mod dir_view;
pub mod sidebar;

use catppuccin_egui::Theme;
use eframe::egui::{Ui, Widget, WidgetText};
use egui_dock::TabViewer;
use egui_tracing::{ui::Logs, EventCollector};

use dir_view::{DirView, DirViewState};
use sidebar::{Sidebar, SidebarState};

pub enum DockTab {
    Sidebar(SidebarState),
    InfoSidebar,
    DirView(DirViewState),
    Log,
    Terminal,
}

pub struct DockTabViewer<'a> {
    theme: &'a Theme,
    log_event_collector: &'a EventCollector,
}

impl<'a> DockTabViewer<'a> {
    pub fn new(theme: &'a Theme, log_event_collector: &'a EventCollector) -> Self {
        Self {
            theme,
            log_event_collector,
        }
    }
}

impl<'a> TabViewer for DockTabViewer<'a> {
    type Tab = DockTab;

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        match tab {
            DockTab::Sidebar(_) => "Sidebar".into(),
            DockTab::InfoSidebar => "Info".into(),
            DockTab::DirView(dir_view_state) => dir_view_state.file_data().name().into(),
            DockTab::Log => "Log".into(),
            DockTab::Terminal => "Terminal".into(),
        }
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        match tab {
            DockTab::Sidebar(sidebar_state) => Sidebar::new(sidebar_state, self.theme).ui(ui),
            DockTab::InfoSidebar => ui.label("Info"),
            DockTab::DirView(dir_view_state) => DirView::new(dir_view_state, self.theme).ui(ui),
            DockTab::Log => Logs::new(self.log_event_collector.clone()).ui(ui),
            DockTab::Terminal => ui.label("Terminal"),
        };
    }
}
