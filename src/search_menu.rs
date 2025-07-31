use bevy::prelude::*;
use bevy_egui::{egui::{self, Align2, RichText}, EguiContexts,};
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static VISIBLE_SEARCH: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

#[derive(Default, Resource)]
pub struct UiState {
    label: String,
}

pub fn search_menu_window(mut contexts: EguiContexts,mut ui_state: ResMut<UiState>,keys: Res<ButtonInput<KeyCode>>,) -> Result {
    let ctx = contexts.ctx_mut()?;
    let screen_center = ctx.input(|reader| {
        reader.screen_rect.center()
    });
    
    let mut visible = VISIBLE_SEARCH.lock().unwrap();
    let mut window_open = *visible;
    let mut should_close = false;
    let window = egui::Window::new(RichText::new("Go to Row").strong())
        .pivot(Align2::CENTER_CENTER)
        .default_pos(screen_center)
        .resizable(true)
        .open(&mut window_open)
        .default_width(700.)
        .default_height(300.);
    
    if *visible {
    window.show(&ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                 egui::TextEdit::multiline(&mut ui_state.label)
                    .hint_text("Enter Row Number")
                    .show(ui);
            });

            ui.add_space(300.);

            ui.horizontal_wrapped(|ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let close_button = ui.button(RichText::new("Cancel").size(20.));
                if close_button.clicked() {
                        should_close = true;
                }
                close_button.on_hover_cursor(egui::CursorIcon::PointingHand);
                let go_button = ui.button(RichText::new("Go").size(20.));
                let my_integer: Result<i32, _> = ui_state.label.trim().parse();
                if ui_state.label.is_empty() == false && go_button.clicked() || ui_state.label.is_empty() == false && keys.just_pressed(KeyCode::Enter){
                match my_integer {
                    Ok(number) => println!("Converted integer: {}", number),
                    Err(_) => println!("Failed to parse string to integer"),
                }
            }
                go_button.on_hover_cursor(egui::CursorIcon::PointingHand);

            });
            });

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        });
    } 
    
    // Apply changes after the egui window logic
    *visible = window_open && !should_close;

    Ok(())
}
