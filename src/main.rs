use colors_transform::Color;
use easy_imgui_window::winit::raw_window_handle::{HasDisplayHandle, HasRawWindowHandle, HasWindowHandle};
use eframe::{App, Frame, NativeOptions};
use egui::{Color32, ColorImage, RichText, TextureHandle, vec2, ViewportBuilder, ViewportCommand, Widget};
use image::{DynamicImage, Pixel, Rgba};
use rand::Rng;
use rayon::prelude::*;
use rfd::FileDialog;

use crate::sort_effect::{mask_image, process_sorting_effect};

mod sort_effect;
mod pixel_generators;

// Структура для хранения данных GUI
struct MyApp {
    low_threshold: f64,
    high_threshold: f64,
    invert_mask: bool,
    opened_image: Option<DynamicImage>,
    result_image: Option<DynamicImage>,
    loaded_texture: Option<TextureHandle>,
    last_error: Option<String>,
    is_error: bool,
    is_mask_showed: bool,
    random_prob: f64,
    pixel_add_choice: pixel_generators::PixelAddChoice,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            low_threshold: 0.0,
            high_threshold: 120.0,
            invert_mask: false,
            opened_image: None,
            result_image: None,
            loaded_texture: None,
            last_error: None,
            is_error: true,
            is_mask_showed: false,
            random_prob: 0.0,
            pixel_add_choice: pixel_generators::PixelAddChoice::RandomPixel,
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |mut ui| {
            ui.horizontal(|ui| {
                if ui.button("Choose File").clicked() {
                    let file = FileDialog::new().pick_file();
                    if let Some(file) = file {
                        match image::open(&file.as_path().to_string_lossy().to_string()) {
                            Ok(i) => {
                                self.loaded_texture = Some(load_texture_from_dynamic_image(&i, ctx));
                                self.opened_image = Some(i.clone());
                                self.is_mask_showed = false;
                                self.result_image = None;
                            }
                            Err(e) => {
                                self.last_error = Some(e.to_string());
                                self.is_error = true;
                            }
                        }
                    }
                }
                ui.heading("Pixel Sort Effect");

                if self.last_error != None {
                    if ui.colored_label(
                        if self.is_error { Color32::RED } else { Color32::GREEN },
                        self.last_error.clone().unwrap().as_str(),
                    ).clicked() {
                        self.last_error = None;
                    }
                }
            });

            ui.separator();

            ui.add_space(20.0);
            ui.label("Mask Settings");
            ui.separator();
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.invert_mask, "Invert mask?");
                ui.add_space(50.0);

                let lt_slider = ui.add(egui::Slider::new(&mut self.low_threshold, 0.0..=256.0).text("Luma Low threshold"));
                ui.add_space(5.0);
                let ht_slider = ui.add(egui::Slider::new(&mut self.high_threshold, 0.0..=256.0).text("Luma High threshold"));
                if lt_slider.changed() || ht_slider.changed() {
                    if self.is_mask_showed {
                        self.loaded_texture = Some(
                            load_texture_from_dynamic_image(&DynamicImage::ImageLuma8(
                                mask_image(&self.opened_image.clone().unwrap().to_rgba8(), self.low_threshold, self.high_threshold, self.invert_mask)
                            ), ctx)
                        );
                    }
                }
                ui.add_space(10.0);
            });
            ui.separator();
            ui.add_space(10.0);

            ui.label("Pixel Addition Settings");
            ui.separator();

            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(&mut self.random_prob, 0.0..=1.0).text("Pixel addition probability"));
                ui.add_space(20.0);
                egui::ComboBox::from_label("Pixel Addition Function")
                    .selected_text(format!("{:?}", self.pixel_add_choice))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.pixel_add_choice, pixel_generators::PixelAddChoice::RandomPixel, "Random Pixel");
                        ui.selectable_value(&mut self.pixel_add_choice, pixel_generators::PixelAddChoice::RandomRedShade, "Random Red Shade");
                        ui.selectable_value(&mut self.pixel_add_choice, pixel_generators::PixelAddChoice::RandomBlueShade, "Random Blue Shade");
                        ui.selectable_value(&mut self.pixel_add_choice, pixel_generators::PixelAddChoice::RandomGreenShade, "Random Green Shade");
                        ui.selectable_value(&mut self.pixel_add_choice, pixel_generators::PixelAddChoice::Black, "Just black");
                    })
            });
            ui.separator();
            ui.add_space(20.0);

            ui.horizontal(|ui| {
                let mask_check = ui.checkbox(&mut self.is_mask_showed, "Show mask");

                if mask_check.changed() {
                    if self.is_mask_showed {
                        if self.opened_image == None {
                            self.last_error = Some("Image is not loaded".to_string());
                            self.is_error = true;
                        } else {
                            self.loaded_texture = Some(
                                load_texture_from_dynamic_image(&DynamicImage::ImageLuma8(
                                    mask_image(&self.opened_image.clone().unwrap().to_rgba8(), self.low_threshold, self.high_threshold, self.invert_mask)
                                ), ctx)
                            );
                        }
                    } else if self.result_image != None {
                        self.loaded_texture = Some(load_texture_from_dynamic_image(&self.result_image.clone().unwrap(), ctx));
                    } else if self.opened_image != None {
                        self.loaded_texture = Some(load_texture_from_dynamic_image(&self.opened_image.clone().unwrap(), ctx));
                    } else {
                        self.loaded_texture = None
                    }
                }

                if ui.button("Apply effect").clicked() {
                    if self.opened_image == None {
                        self.last_error = Some("Image is not loaded".to_string());
                        self.is_error = true;
                    } else {
                        let mask = mask_image(&self.opened_image.clone().unwrap().to_rgba8(), self.low_threshold, self.high_threshold, self.invert_mask);

                        let (r_image, duration) = process_sorting_effect(&self.opened_image.clone().unwrap().to_rgba8(), &mask, self.random_prob, |x, y, p| {
                            match self.pixel_add_choice {
                                pixel_generators::PixelAddChoice::RandomPixel => { pixel_generators::get_random_pixel() }
                                pixel_generators::PixelAddChoice::RandomRedShade => { pixel_generators::get_random_red_shade() }
                                pixel_generators::PixelAddChoice::RandomBlueShade => { pixel_generators::get_random_blue_shade() }
                                pixel_generators::PixelAddChoice::RandomGreenShade => { pixel_generators::get_random_green_shade() }
                                pixel_generators::PixelAddChoice::Black => { pixel_generators::get_black() }
                            }
                        });
                        let result = DynamicImage::ImageRgba8(r_image);
                        self.loaded_texture = Some(load_texture_from_dynamic_image(&result, ctx));
                        self.result_image = Some(result);
                        self.is_mask_showed = false;
                        self.last_error = Some(format!("Time elapsed is: {:?}", duration).to_string());
                        self.is_error = false;
                    }
                }

                if self.result_image != None {
                    ui.add_space(10.0);

                    if ui.button(RichText::new("Export result").color(Color32::GREEN)).clicked() {
                        let file = FileDialog::new()
                            .set_file_name("result.png")
                            .set_title("Export result")
                            .save_file();
                        if let Some(file) = file {
                            let path = file.as_path().to_string_lossy().to_string();

                            match self.result_image.clone().unwrap().save(&path) {
                                Ok(_) => {
                                    self.last_error = Some(format!("File was saved to {}", path));
                                    self.is_error = false
                                }
                                Err(e) => {
                                    self.last_error = Some(e.to_string());
                                    self.is_error = true;
                                }
                            }
                        }
                    }
                }
            });

            ui.separator();
            if self.loaded_texture == None {
                ui.label("Image is not loaded.");
            } else {
                ui.add(
                    egui::Image::new((self.loaded_texture.clone().unwrap().id(), self.loaded_texture.clone().unwrap().size_vec2()))
                        .max_size(ui.available_size())
                );
            }

            // ctx.send_viewport_cmd(ViewportCommand::Title(format!("Pixel Sort Effect {:?}", ctx.input(|i| (i.screen_rect.width(), i.screen_rect.height())))));
        });
    }
}

fn main() {
    let options = NativeOptions {
        viewport: ViewportBuilder {
            maximize_button: Some(false),
            resizable: Some(true),
            min_inner_size: Some(vec2(730.0, 600.0)),
            inner_size: Some(vec2(730.0, 600.0)),
            ..Default::default()
        },
        ..Default::default()
    };
    eframe::run_native(
        "Pixel Sort Effect",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    ).unwrap();
}

fn load_texture_from_dynamic_image(image: &DynamicImage, ctx: &egui::Context) -> TextureHandle {
    let (width, height) = image.to_rgba8().dimensions();

    let color_image = ColorImage::from_rgba_unmultiplied([width as usize, height as usize], &image.to_rgba8());
    ctx.load_texture("my_image", color_image, Default::default())
}
