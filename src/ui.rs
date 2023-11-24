use std::path::Path;

use egui_macroquad::{*, egui::{Context, TopBottomPanel, RichText, Color32, menu, Align2, Label, Slider, Window, TextureHandle, ColorImage, Ui}};
use egui_macroquad::egui::Vec2 as UIVec2;
use macroquad::time::{get_frame_time, get_fps}; 
use crate::globals::*;

pub struct UI {
    pointer_over: bool,
    monitor_win: bool,
    settings_win: bool,
    about_win: bool,
    logo: Option<egui_macroquad::egui::TextureHandle>,
    big_logo: Option<egui_macroquad::egui::TextureHandle>,
}


impl UI {

    pub fn new() -> Self {
        //let img =  Self::load_image(Path::new("assets/img/atom.png")).unwrap();
        Self {
            pointer_over: false,
            monitor_win: false,
            settings_win: false,
            about_win: false,
            logo: Self::load_textures("atom"),
            big_logo: Self::load_textures("atom_med"),
        }
    }

    fn load_image(path: &Path) -> Result<egui_macroquad::egui::ColorImage, image::ImageError> {
        let image = image::io::Reader::open(path)?.decode()?;
        let size = [image.width() as _, image.height() as _];
        let image_buffer = image.to_rgba8();
        let pixels = image_buffer.as_flat_samples();
        Ok(egui_macroquad::egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()))
    }
    
    pub fn load_textures(name: &str) -> Option<TextureHandle> {
        let mut texture: Option<TextureHandle> = None;
        egui_macroquad::ui(|egui_ctx| {
            let path = format!("assets/img/{}.png", name);
            let img =  Self::load_image(Path::new(&path)).unwrap();
            texture = Some(egui_ctx.load_texture("logo".to_string(), img, Default::default()));
        });
        return texture;
    }

    pub fn process(&mut self, fps: i32, fps2: i32) {
        egui_macroquad::ui(|egui_ctx| {
            self.pointer_over = egui_ctx.is_pointer_over_area();
            self.build_top_menu(egui_ctx);
            self.build_monitor_win(egui_ctx, fps, fps2);
            self.build_settings_win(egui_ctx);
            self.build_about_win(egui_ctx);
        });
    }

    pub fn draw(&self) {
        egui_macroquad::draw();
    }

    fn build_top_menu(&mut self, egui_ctx: &Context) {
        let mut signals = get_signals();
        TopBottomPanel::top("top_panel").default_height(100.0).show(egui_ctx, |ui| {
            if !self.pointer_over {
                self.pointer_over = ui.ui_contains_pointer();
            }
            
            menu::bar(ui, |ui| {
                let logo = self.logo.clone().unwrap();
                ui.image(logo.id(), logo.size_vec2());
                ui.separator();
                ui.label(RichText::new("MAD LAB").heading().strong().color(Color32::RED));
                ui.separator();
                
                menu::menu_button(ui, RichText::new("SIM").strong(), |ui| {
                    if ui.button(RichText::new("Start New Sim").strong().color(Color32::GREEN)).clicked() {
                        signals.start_new_sim = true;
                        set_global_signals(signals);
                    }
                    if ui.button(RichText::new("Quit").strong().color(Color32::RED)).clicked() {
                        std::process::exit(0);
                    }
                });

                ui.separator();
                menu::menu_button(ui, RichText::new("SETTINGS").strong(), |ui| {
                    if ui.button(RichText::new("Shuffle interactions").strong().color(Color32::GREEN)).clicked() {
                        signals.shuffle_interactions = true;
                        set_global_signals(signals);
                    }
                    if ui.button(RichText::new("Rules").strong().color(Color32::GREEN)).clicked() {
                        self.settings_win = !self.settings_win;
                    }
                });
                
                ui.separator();
                menu::menu_button(ui, RichText::new("VIEW").strong(), |ui| {
                    if ui.button(RichText::new("Monitor").strong().color(Color32::GOLD)).clicked() {
                        self.monitor_win = !self.monitor_win;
                    }
                    if ui.button(RichText::new("Display Filled Elements").strong().color(Color32::GREEN)).clicked() {
                        let mut settings = get_settings();
                        settings.display = DisplayMode::ELEMENTS;
                        set_global_settings(settings);
                    }
                    if ui.button(RichText::new("Display Elements Energy").strong().color(Color32::RED)).clicked() {
                        let mut settings = get_settings();
                        settings.display = DisplayMode::ENERGY;
                        set_global_settings(settings);
                    }
                    if ui.button(RichText::new("Display Stroke Elements").strong().color(Color32::BLUE)).clicked() {
                        let mut settings = get_settings();
                        settings.display = DisplayMode::STROKE;
                        set_global_settings(settings);
                    }
                });

                ui.separator();
                menu::menu_button(ui, RichText::new("INFO").strong(), |ui| {
                    if ui.button(RichText::new("About").strong().color(Color32::GREEN)).clicked() {
                        self.about_win = !self.about_win;
                    }
                });

            })
        });
    }

    fn build_monitor_win(&mut self, egui_ctx: &Context, fps: i32, fps2: i32) {
        if self.monitor_win {
            egui::Window::new("Monitor").default_height(80.0).anchor(Align2::RIGHT_TOP, [0.0, 0.0]).show(egui_ctx, |win| {
                win.vertical(|ui| {
                    let dt = (get_frame_time()*100.0).round()/100.0;
                    //let fps = get_fps();
                    let txt = format!("dT: {} | FPS: {}({})", dt, fps, fps2);
                    ui.add(Label::new(RichText::new(txt).color(Color32::GREEN).strong()));
                })
            });
        }
    }

    fn build_settings_win(&mut self, egui_ctx: &Context) {
        if !self.settings_win {
            return;
        }
        let mut settings = get_settings();
        egui::Window::new("SETTINGS").id("settings_win".into()).default_pos((SCREEN_WIDTH/2., 0.0)).fixed_size([380., 400.])
        .title_bar(true).show(egui_ctx, |ui| {
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut particles_num = settings.particles_num;
                column[0].label(RichText::new("PARTICLES NUMBER").color(Color32::YELLOW).strong());
                if column[1].add(Slider::new(&mut particles_num, 0..=20000).step_by(100.0)).changed() {
                    settings.particles_num = particles_num;
                    set_global_settings(settings);
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut particle_types = settings.particle_types;
                column[0].label(RichText::new("PARTICLE TYPES").color(Color32::LIGHT_BLUE).strong());
                if column[1].add(Slider::new(&mut particle_types, 1..=19)).changed() {
                    settings.particle_types = particle_types;
                    set_global_settings(settings);
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut field_radius = settings.field;
                column[0].label(RichText::new("FIELD RADIUS").color(Color32::BLUE).strong());
                if column[1].add(Slider::new(&mut field_radius, 0.0..=400.0).step_by(5.0)).changed() {
                    settings.field = field_radius;
                    set_global_settings(settings);
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut force = settings.force;
                column[0].label(RichText::new("FORCE").color(Color32::GREEN).strong());
                if column[1].add(Slider::new(&mut force, 0.0..=500000.0).step_by(5000.0)).changed() {
                    settings.force = force;
                    set_global_settings(settings);
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut repel = settings.repel;
                column[0].label(RichText::new("REPEL RELATIVE DISTANCE").color(Color32::RED).strong());
                if column[1].add(Slider::new(&mut repel, 0.0..=1.0).step_by(0.05)).changed() {
                    settings.repel = repel;
                    set_global_settings(settings);
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut damping = settings.damping;
                column[0].label(RichText::new("DAMPING").color(Color32::DARK_BLUE).strong());
                if column[1].add(Slider::new(&mut damping, 0.0..=2.0).step_by(0.05)).changed() {
                    settings.damping = damping;
                    set_global_settings(settings);
                    let mut signals = get_signals();
                    signals.particles_new_settings = true;
                    set_global_signals(signals);
                }
            });
        });
    }

    fn build_about_win(&mut self, egui_ctx: &Context) {
        if self.about_win {
            Window::new("ABOUT").resizable(false).default_pos((SCREEN_WIDTH/2.-150., SCREEN_HEIGHT/6.)).min_height(680.).min_width(300.)
            .title_bar(true).show(egui_ctx, |ui| {
                let big_logo = self.big_logo.clone().unwrap();
                ui.vertical_centered(|pic| {
                    pic.image(big_logo.id(), big_logo.size_vec2());
                });
                ui.add_space(2.0);
                ui.vertical_centered(|title| {
                    title.heading(RichText::new("MAD LAB").color(Color32::RED).strong());
                });
                ui.vertical_centered(|author| {
                    author.label(RichText::new("Artur Gwoździowski 2019-2023").color(Color32::BLUE).strong());
                });
                ui.add_space(2.0);
                ui.vertical_centered(|author| {
                    author.label(RichText::new(format!("version {}", env!("CARGO_PKG_VERSION"))).color(Color32::YELLOW).italics());
                });
                ui.add_space(2.0);
                ui.vertical_centered(|closer| {
                    if closer.button(RichText::new("CLOSE").color(Color32::LIGHT_BLUE).strong()).clicked() {
                        self.about_win = false;
                    }
                });
            });
        }
    }

}

struct LogoImage {
    texture: Option<TextureHandle>,
}

impl LogoImage {
    fn ui(&mut self, ui: &mut Ui) {
        let texture: &TextureHandle = self.texture.get_or_insert_with(|| {
            ui.ctx().load_texture("atom", ColorImage::example(), Default::default())
        });
        ui.add(egui_macroquad::egui::Image::new(texture, texture.size_vec2()));
        ui.image(texture, texture.size_vec2());
    }
}