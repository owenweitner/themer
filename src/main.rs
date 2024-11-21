#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use eframe::egui::{self, popup_below_widget, Id, PopupCloseBehavior};
use eframe::App;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;
use std::io::{BufReader, ErrorKind, Write};
use rfd::FileDialog;

mod structs;
use crate::structs::{AppData, Theme};
mod generator;
use crate::generator::generate_colors;
mod apply;
use crate::apply::apply_theme;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    

    //Load from memory eventually
    let data = load();

    eframe::run_native(
        "WalThemer",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            
            Ok(Box::new(data))
        }),
    )
}


impl eframe::App for AppData {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui|{
                ui.heading("WalThemer");
                ui.horizontal(|ui| {
                    ui.label(format!("Theme: {}", self.entries[self.selected_index]));
                    let response = ui.button("Rename");
                    let popup_id = Id::new("popup_id");

                    if response.clicked() {
                        ui.memory_mut(|mem| mem.toggle_popup(popup_id));
                    }

                    popup_below_widget(
                        ui,
                        popup_id,
                        &response,
                        PopupCloseBehavior::CloseOnClickOutside,
                        |ui| {
                            ui.set_min_width(300.0);
                            ui.label("What do you want to name your Theme?");
                            ui.text_edit_singleline(&mut self.tempname);
                            if ui.button("Submit").clicked() {
                                rename_theme(self);
                            }
                        },
                    );
                });
                
                let imageuri: String = self.themes[self.selected_index].imagepath.clone();
                ui.add(
                    egui::Image::new(format!("file://{imageuri}"))
                );

                if ui.button("Open Image").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.temppath = Some(path.display().to_string());
                    }

                    self.themes[self.selected_index].imagepath = self.temppath.clone().expect("Image Selection Failed");
                }
                
                ui.horizontal(|ui| {
                    ui.vertical(|ui|{
                        ui.label("color 0");
                        ui.color_edit_button_rgb(&mut self.themes[self.selected_index].color0);
                        ui.label("color 8");
                        ui.color_edit_button_rgb(&mut self.themes[self.selected_index].color8);
                    });
                    ui.vertical(|ui|{
                        ui.label("color 1");
                        ui.color_edit_button_rgb(&mut self.themes[self.selected_index].color1);
                        ui.label("color 9");
                        ui.color_edit_button_rgb(&mut self.themes[self.selected_index].color9);
                    });
                    ui.vertical(|ui|{
                        ui.label("color 2");
                        ui.color_edit_button_rgb(&mut self.themes[self.selected_index].color2);
                        ui.label("color 10");
                        ui.color_edit_button_rgb(&mut self.themes[self.selected_index].color10);
                    });
                    ui.vertical(|ui|{
                        ui.label("color 3");
                        ui.color_edit_button_rgb(&mut self.themes[self.selected_index].color3);
                        ui.label("color 11");
                        ui.color_edit_button_rgb(&mut self.themes[self.selected_index].color11);
                    });
                    ui.vertical(|ui|{
                        ui.label("color 4");
                        ui.color_edit_button_rgb(&mut self.themes[self.selected_index].color4);
                        ui.label("color 12");
                        ui.color_edit_button_rgb(&mut self.themes[self.selected_index].color12);
                    });
                    ui.vertical(|ui|{
                        ui.label("color 5");
                        ui.color_edit_button_rgb(&mut self.themes[self.selected_index].color5);
                        ui.label("color 13");
                        ui.color_edit_button_rgb(&mut self.themes[self.selected_index].color13);
                    });
                    ui.vertical(|ui|{
                        ui.label("color 6");
                        ui.color_edit_button_rgb(&mut self.themes[self.selected_index].color6);
                        ui.label("color 14");
                        ui.color_edit_button_rgb(&mut self.themes[self.selected_index].color14);
                    });
                    ui.vertical(|ui|{
                        ui.label("color 7");
                        ui.color_edit_button_rgb(&mut self.themes[self.selected_index].color7);
                        ui.label("color 15");
                        ui.color_edit_button_rgb(&mut self.themes[self.selected_index].color15);
                    });
                });
                ui.horizontal(|ui| {
                    ui.label("Foreground");
                    ui.color_edit_button_rgb(&mut self.themes[self.selected_index].foreground);
                    ui.label("Background");
                    ui.color_edit_button_rgb(&mut self.themes[self.selected_index].background);
                });
                ui.horizontal(|ui| {
                    ui.label("polybar:");
                    ui.label("Foreground");
                    ui.color_edit_button_rgb(&mut self.themes[self.selected_index].polybar_foreground);
                    ui.label("Background");
                    ui.color_edit_button_rgb(&mut self.themes[self.selected_index].polybar_background);
                });
                
                egui::ComboBox::from_label("Options")
                    .selected_text(&self.entries[self.selected_index]) // Show the selected entry's text
                    .show_ui(ui, |ui| {
                        for (index, entry) in self.entries.iter().enumerate() {
                            // Add an entry for each option in the entries Vec
                            ui.selectable_value(&mut self.selected_index, index, entry);
                        }
                    });

                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        save(self);
                    }
                    if ui.button("Apply").clicked() {
                        apply_theme(self);
                    }
                    if ui.button("New").clicked() {
                        let empty: Theme = new_theme("New".to_string());
                        self.themes.push(empty.clone());
                        self.entries.push(empty.name.to_string());
                        self.selected_index = self.entries.iter().position(|n| *n == empty.name.to_string()).unwrap();
                    }
                    if ui.button("Generate Colors").clicked() {
                        generate_colors(self);
                    }


                });


                

            });
        }     
        );
    }
}


fn rgb_u8_to_f32(rgb: [u8; 3]) -> [f32; 3] {
    println!("{},{},{}", rgb[0], rgb[1], rgb[2]);
    [
        (rgb[0] as f32 / 255.0).powf(2.2),
        (rgb[1] as f32 / 255.0).powf(2.2),
        (rgb[2] as f32 / 255.0).powf(2.2),
    ]
}


fn rgb_f32_to_u8(rgb: [f32; 3]) -> [u8; 3] {
    [
        (rgb[0].powf(1.0/2.2) * 255.0).round() as u8,
        (rgb[1].powf(1.0/2.2) * 255.0).round() as u8,
        (rgb[2].powf(1.0/2.2) * 255.0).round() as u8,
    ]
}

    
fn load() -> AppData{
    match load_from_file(){
        Ok(n) => return n,
        Err(err) => new_appdata(),
    }
    
}

fn load_from_file() -> Result<AppData, serde_json::Error>{
    let file1 = File::open("/home/.config/themer/appdata.txt");
    let file2 = match file1 {
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("/home/.config/themer/appdata.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("Error Opening AppData.txt")
            },
            other_error => {
                panic!("Error Opening AppData.txt")
            }
        },
    };
    let reader = BufReader::new(file2);
    
    
    return serde_json::from_reader(reader);
}

fn new_appdata() -> AppData{
    let themes: Vec<Theme> = vec![new_theme("".to_string())];
    let entries: Vec<String> = vec!["".to_string()];
    let data: AppData = AppData {
        themes: themes,
        entries: entries,
        selected_index: 0,
        tempname: "".to_string(),
        temppath: None,
        current_theme: "".to_string(),
    };
    return data
}


fn rename_theme(data: &mut AppData){
    data.entries[data.selected_index] = data.tempname.clone();
    data.themes[data.selected_index].name = data.tempname.clone();
}

fn new_theme(name: String) -> Theme{
    let mut empty = Theme{
        name: "Test".to_string(),
        color0:  [0.0, 0.0, 0.0],
        color1:  [0.0, 0.0, 0.0],
        color2:  [0.0, 0.0, 0.0],
        color3:  [0.0, 0.0, 0.0],
        color4:  [0.0, 0.0, 0.0],
        color5:  [0.0, 0.0, 0.0],
        color6:  [0.0, 0.0, 0.0],
        color7:  [0.0, 0.0, 0.0],
        color8:  [0.0, 0.0, 0.0],
        color9: [0.0, 0.0, 0.0],
        color10: [0.0, 0.0, 0.0],
        color11: [0.0, 0.0, 0.0],
        color12: [0.0, 0.0, 0.0],
        color13: [0.0, 0.0, 0.0],
        color14: [0.0, 0.0, 0.0],
        color15: [0.0, 0.0, 0.0],
        foreground: [0.0, 0.0, 0.0],
        background: [0.0, 0.0, 0.0],
        polybar_background: [0.0, 0.0, 0.0],
        polybar_foreground: [0.0, 0.0, 0.0],
        imagepath: "".to_string(),
    };
    empty.name = name;
    return empty
}

fn save(data: &AppData){
    let save = serde_json::to_string(data).unwrap();
    let path = Path::new("/home/.config/themer/appdata.txt");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(save.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
    print!("Saved")
}