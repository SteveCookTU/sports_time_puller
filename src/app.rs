use crate::mlb::mlb_pane::MlbPane;
use crate::Pane;
use eframe::egui::Context;
use eframe::{egui, App, Frame};

pub struct SportsTimePuller {
    active_pane: Box<dyn Pane>,
}

impl Default for SportsTimePuller {
    fn default() -> Self {
        Self {
            active_pane: Box::new(MlbPane::new()),
        }
    }
}

impl SportsTimePuller {}

impl App for SportsTimePuller {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::SidePanel::left("side_panel")
            .min_width(250.0)
            .resizable(true)
            .show(ctx, |ui| {
                self.active_pane.side_panel(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.active_pane.central_panel(ui);
        });
    }
}
