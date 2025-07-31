use bevy::prelude::*;
use bevy_egui::{egui::{self, Align2, RichText}, EguiContexts,};
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static VISIBLE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
const VERSION: &str = "1.0"; 
pub fn about_window(mut contexts: EguiContexts) -> Result {
    
    let ctx = contexts.ctx_mut()?;
    let screen_center = ctx.input(|reader| {
        reader.screen_rect.center()
    });
    
    let mut visible = VISIBLE.lock().unwrap();
    let mut window_open = *visible;
    let mut should_close = false;
    let glorifying_text = "A Website to Glorify the Lord Jesus Christ and His Humble Servant! The Glorious Theologian named Aurelius Augustinus Whom was Born in Thagaste in Modern Day Algeria, Was Born to Saint Monica in 354AD and Had his Bodily Death in 430AD, When He was 75 Years Old, May He Pray for Us";
    let window = egui::Window::new(RichText::new("Sanctus Augustinus, Ora Pro Nobis").strong())
        .pivot(Align2::CENTER_CENTER)
        .default_pos(screen_center)
        .resizable(true)
        .open(&mut window_open);
    
    if *visible {
    window.show(&ctx, |ui| {
            ui.label(RichText::new("SanctusAugustinus.cc").size(20.));
            ui.add_space(15.);
            ui.label(RichText::new(glorifying_text).size(15.));
            ui.add_space(250.);
            ui.horizontal_wrapped(|ui| {

            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.label(format!("Version: {}", VERSION));
            });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let close_button = ui.button(RichText::new("Close").size(20.));
                if close_button.clicked() {
                        should_close = true;
                }
                close_button.on_hover_cursor(egui::CursorIcon::PointingHand);
            });
            });

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        });
    } 
    
    // Apply changes after the egui window logic
    *visible = window_open && !should_close;
    
    Ok(())
}
