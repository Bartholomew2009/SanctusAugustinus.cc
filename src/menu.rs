use bevy::{log::tracing_subscriber::fmt::format, prelude::*};
use bevy_egui::{egui::{self, style::ScrollStyle, CollapsingHeader, Color32, Rangef, RichText}, EguiContexts,};
use crate::egui::Frame;
use crate::egui::Margin;
use crate::about_window;
use crate::search_menu;

pub struct Images {
    pub data_icon: Handle<Image>,
    pub burger_icon: Handle<Image>
}

impl FromWorld for Images {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        Self {
            data_icon: asset_server.load("Cloud.png"),
            burger_icon: asset_server.load("burger-menu.png"),
        }
    }
}

pub fn spawn_system(mut contexts: EguiContexts,images: Local<Images>,keys: Res<ButtonInput<KeyCode>>,) -> Result {
    
    let data_icon = contexts.add_image(images.data_icon.clone_weak());
    let burger_icon = contexts.add_image(images.burger_icon.clone_weak());
    let ctx = contexts.ctx_mut()?;

    let header = egui::TopBottomPanel::top("header").default_height(30.).resizable(false).frame(Frame {
        fill: Color32::from_hex("#9c9c86ff").expect("Failure to Detect Colour"),
        inner_margin: Margin {
            left: 8,
            right: Default::default(),
            top: 5,
            bottom: Default::default(),
        },
        stroke: Default::default(),
        corner_radius: Default::default(),
        outer_margin: Default::default(),
        shadow: Default::default(),
    }).show_separator_line(false);
     
     let ctrl =  keys.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);
    if ctrl && keys.just_pressed(KeyCode::KeyG) && *search_menu::VISIBLE_SEARCH.lock().unwrap() == false{
        *search_menu::VISIBLE_SEARCH.lock().unwrap() = true;
    } else if ctrl && keys.just_pressed(KeyCode::KeyG) && *search_menu::VISIBLE_SEARCH.lock().unwrap() == true {
        *search_menu::VISIBLE_SEARCH.lock().unwrap() = false;
    }
     
     header.show(ctx, |ui| {
            ui.vertical(|ui| {                
                egui::menu::bar(ui, |ui| {
                    let menu_button = ui.menu_image_button(egui::widgets::Image::new(egui::load::SizedTexture::new(burger_icon,[30.0, 24.0])), |ui| {
                        let left_column =  ui.menu_button(RichText::new("Left Column").color(Color32::WHITE), |ui| {
                            let latin_button = ui.button(RichText::new("Latin").color(Color32::from_rgb(180, 85,72)));
                            let english_button = ui.button(RichText::new("English").color(Color32::from_rgb(180, 85,72)));
                            latin_button.on_hover_cursor(egui::CursorIcon::PointingHand);
                            english_button.on_hover_cursor(egui::CursorIcon::PointingHand);
                        });

                       let right_column =  ui.menu_button(RichText::new("Right Column").color(Color32::WHITE), |ui| {
                            let latin_button = ui.button(RichText::new("Latin").color(Color32::from_rgb(180, 85,72)));
                            let english_button = ui.button(RichText::new("English").color(Color32::from_rgb(180, 85,72)));

                            latin_button.on_hover_cursor(egui::CursorIcon::PointingHand);
                            english_button.on_hover_cursor(egui::CursorIcon::PointingHand);
                        });

                        ui.label(RichText::new("Options").color(Color32::LIGHT_GRAY).size(12.));
                        let go_to_button = ui.button(RichText::new("Go to Row    (CTRL+G)").color(Color32::WHITE));
                        if go_to_button.clicked() && *search_menu::VISIBLE_SEARCH.lock().unwrap() == false {
                            *search_menu::VISIBLE_SEARCH.lock().unwrap() = true;
                        } else if go_to_button.clicked() && *search_menu::VISIBLE_SEARCH.lock().unwrap() == true {
                            *search_menu::VISIBLE_SEARCH.lock().unwrap() = false;
                        }
                        ui.horizontal(|ui| {
                            ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(data_icon,[20.0, 20.0])));
                            ui.label(RichText::new("Data").color(Color32::LIGHT_GRAY).size(12.));
                        });

                        let about_button = ui.button(RichText::new("About").color(Color32::WHITE));
                        if about_button.clicked() && *about_window::VISIBLE.lock().unwrap() == false{
                            *about_window::VISIBLE.lock().unwrap() = true;
                        } else if about_button.clicked() && *about_window::VISIBLE.lock().unwrap() == true {
                            *about_window::VISIBLE.lock().unwrap() = false;
                        }
                        about_button.on_hover_cursor(egui::CursorIcon::PointingHand);
                        go_to_button.on_hover_cursor(egui::CursorIcon::PointingHand);
                        left_column.response.on_hover_cursor(egui::CursorIcon::PointingHand);
                        right_column.response.on_hover_cursor(egui::CursorIcon::PointingHand);
                    });


                    menu_button.response.on_hover_cursor(egui::CursorIcon::PointingHand);
                });
            });
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        });

    let book_list_gui = egui::SidePanel::left("left_panel").default_width(415.).resizable(false).width_range(Rangef {
        max: 400.,
        min: 400.,
    }).frame(Frame {
        fill: Color32::from_hex("#c9c9b4").expect("Failure to Detect Colour"),
        inner_margin: Margin {
            left: 8,
            right: Default::default(),
            top: 5,
            bottom: Default::default(),
        },
        stroke: Default::default(),
        corner_radius: Default::default(),
        outer_margin: Default::default(),
        shadow: Default::default(),
    }).show_separator_line(false);
     
     
     
     book_list_gui.show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                
                let auto_biographical_works_header = CollapsingHeader::new(RichText::new("Autobiographical Works").strong())
                .default_open(false).show_background(true);
                auto_biographical_works_header.show(ui, |ui| {
                let confessions_header = CollapsingHeader::new(RichText::new("Confessions").strong())
                .default_open(false).show_background(true).show(ui, |ui| {
                    let _ = ui.button(RichText::new("Retractiones II").strong());
                    let blessed_confession_titles = ["Infancy and boyhood up to age 14","Fall amongst bad companions, which leads to thievery and lust"
                    ,"Studies at Carthage, conversion to Manichaeism, and continued indulgence in lust (age 16-19)","Loss of a friend, studies in Aristotle, the fit and the fair (age 20-29)"
                    ,"Moving away from Manichaeism under the influence of St. Ambrose in Milan (age 29)","Moving towards Catholicism under the influence of St. Ambrose (age 30)",
                    "Moving towards a greater understanding of God (age 31)","Conversion to Christianity and instruction by Simplicianus on how to convert others (age 32)",
                    "Baptism, the death of his mother Monica, the death of his friends Nebridius and Vecundus, and his abandonment of his studies of rhetoric (age 33)",
                    "Continued reflections on the values of confessions and on the workings of memory, as related to the 5 senses",
                    "Reflections on Genesis and searching for the meaning of time","Continued reflections on the book of Genesis","Exploration of the meaning of Genesis and the Trinity"];

                        for (index, title) in blessed_confession_titles.iter().enumerate() {
                            let formatted_title = format!("Bk. {} - {}", index + 1, title);
                            let wrapped_title = wrap_text(&formatted_title, 50);
                            CollapsingHeader::new(RichText::new(wrapped_title).strong())
                                .default_open(false)
                                .show_background(true)
                                .show(ui, |ui| {
                                   match index + 1 {
                                    1 => for num in 1..=20 {
                                        ui.button(RichText::new(format!("Chapter {}", num)).strong());
                                    },
                                    2 => for num in 1..=10 {
                                        ui.button(RichText::new(format!("Chapter {}", num)).strong());
                                    },
                                    3 => for num in 1..=12 {
                                        ui.button(RichText::new(format!("Chapter {}", num)).strong());
                                    },
                                    4 => for num in 1..=16 {
                                        ui.button(RichText::new(format!("Chapter {}", num)).strong());
                                    },
                                    5 => for num in 1..=14 {
                                        ui.button(RichText::new(format!("Chapter {}", num)).strong());
                                    },
                                    6 => for num in 1..=16 {
                                        ui.button(RichText::new(format!("Chapter {}", num)).strong());
                                    },
                                    7 => for num in 1..=21 {
                                        ui.button(RichText::new(format!("Chapter {}", num)).strong());
                                    },
                                    8 => for num in 1..=12 {
                                        ui.button(RichText::new(format!("Chapter {}", num)).strong());
                                    },
                                    9 => for num in 1..=13 {
                                        ui.button(RichText::new(format!("Chapter {}", num)).strong());
                                    },
                                    10 => for num in 1..=43 {
                                        ui.button(RichText::new(format!("Chapter {}", num)).strong());
                                    },
                                    11 => for num in 1..=31 {
                                        ui.button(RichText::new(format!("Chapter {}", num)).strong());
                                    },
                                    12 => for num in 1..=32 {
                                        ui.button(RichText::new(format!("Chapter {}", num)).strong());
                                    },
                                    13 => for num in 1..=38 {
                                        ui.button(RichText::new(format!("Chapter {}", num)).strong());
                                    },
                                    _ => panic!()
                                   }
                                });
                        }
                            
                });
            });
            
            
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        });
        });
    Ok(())

}


fn wrap_text(text: &str, max_line_length: usize) -> String {
    let mut wrapped = String::new();
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut line = String::new();

    for word in words {
        if line.len() + word.len() + 1 > max_line_length {
            wrapped.push_str(&line);
            wrapped.push_str("\n");
            line = word.to_string();
        } else {
            if !line.is_empty() {
                line.push(' ');
            }
            line.push_str(word);
        }
    }

    // Add the last line
    if !line.is_empty() {
        wrapped.push_str(&line);
    }

    wrapped
}