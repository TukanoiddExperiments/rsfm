use catppuccin_egui::Theme;
use eframe::egui::{Ui, Widget, WidgetText};
use egui_dock::TabViewer;
use egui_tracing::EventCollector;

use super::sidebar::{Sidebar, SidebarState};

pub enum DockTab {
    Sidebar(SidebarState),
    InfoSidebar,
    DirView,
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
            DockTab::Sidebar(_) => "Sidebar",
            DockTab::InfoSidebar => "Info",
            DockTab::DirView => "DirView",
            DockTab::Log => "Log",
            DockTab::Terminal => "Terminal",
        }
        .into()
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        match tab {
            DockTab::Sidebar(sidebar_state) => Sidebar::new(sidebar_state, self.theme).ui(ui),
            DockTab::InfoSidebar => ui.label("Info"),
            DockTab::DirView => ui.label("Dir View"),
            DockTab::Log => ui.label("Log"),
            DockTab::Terminal => ui.label("Terminal"),
        };
    }
}
