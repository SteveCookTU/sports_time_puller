use eframe::egui::Ui;

pub mod app;
mod mlb;
mod time_zone;

pub trait Pane {
    fn side_panel(&mut self, ui: &mut Ui);
    fn central_panel(&mut self, ui: &mut Ui);
}
