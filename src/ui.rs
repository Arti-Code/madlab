use std::path::Path;

use egui_macroquad::{*, egui::{menu, Align2, Color32, ColorImage, Context, Label, RichText, Slider, TextureHandle, TopBottomPanel, Ui, Window}};
use egui_macroquad::egui::Vec2 as UIVec2;
use macroquad::time::{get_frame_time, get_fps};
use macroquad::math::clamp; 
use egui_macroquad::egui::vec2;
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
            logo: Self::load_textures("science32"),
            big_logo: Self::load_textures("science128"),
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
        let mut signals = signals();
        let mut settings =  get_settings();
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
                        set_signals(signals);
                    }
                    if ui.button(RichText::new("Quit").strong().color(Color32::RED)).clicked() {
                        std::process::exit(0);
                    }
                });

                ui.separator();
                menu::menu_button(ui, RichText::new("SETTINGS").strong(), |ui| {
                    if ui.button(RichText::new("Settings").strong().color(Color32::GREEN)).clicked() {
                        self.settings_win = !self.settings_win;
                    }
                });

                ui.separator();
                let mut onoff = String::from("Enable Repel");
                let mut color = Color32::GREEN;
                let mut collisions_label = "Enable Collisions".to_string();
                let mut col_collisions = Color32::YELLOW;
                if settings.collisions {
                    collisions_label = "Disable Collisions".to_string();
                    col_collisions = Color32::GREEN;
                }
                if settings.repel_on {
                    color = Color32::YELLOW;
                    onoff = "Disable Repel".to_string();
                }

                menu::menu_button(ui, RichText::new("RULES").strong(), |ui| {
                    if ui.button(RichText::new(onoff).strong().color(color)).clicked() {
                        settings.repel_on = !settings.repel_on;
                        set_settings(settings);
                    }
                    if ui.button(RichText::new(collisions_label).strong().color(col_collisions)).clicked() {
                        settings.collisions = !settings.collisions;
                        set_settings(settings);
                    }
                    if ui.button(RichText::new("Shuffle Particles").strong().color(Color32::GREEN)).clicked() {
                        signals.shuffle_interactions = true;
                        set_signals(signals);
                    }
                });
                
                ui.separator();
                menu::menu_button(ui, RichText::new("VIEW").strong(), |ui| {
                    if ui.button(RichText::new("Monitor").strong().color(Color32::GOLD)).clicked() {
                        self.monitor_win = !self.monitor_win;
                    }
                    if ui.button(RichText::new("Display Filled Elements").strong().color(Color32::GREEN)).clicked() {
                        let mut cfg = get_settings();
                        cfg.display = DisplayMode::ELEMENTS;
                        set_settings(cfg);
                    }
                    if ui.button(RichText::new("Display Elements Energy").strong().color(Color32::RED)).clicked() {
                        let mut cfg = get_settings();
                        cfg.display = DisplayMode::ENERGY;
                        set_settings(cfg);
                    }
                    if ui.button(RichText::new("Display Stroke Elements").strong().color(Color32::BLUE)).clicked() {
                        let mut cfg = get_settings();
                        cfg.display = DisplayMode::STROKE;
                        set_settings(cfg);
                    }
                    if ui.button(RichText::new("Show Field Range").strong().color(Color32::BLUE)).clicked() {
                        let mut cfg = get_settings();
                        cfg.field_range = !cfg.field_range;
                        set_settings(cfg);
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
            let red = 1.0 - clamp(fps2 as f32, 0.0, 60.0) / 60.0;
            let green = clamp(fps2 as f32, 0.0, 60.0) / 60.0;
            //let green = 1.0 - red;
            let i = 1.0/(red+green);
            let r = (red*i * 255.0) as u8;
            let g = (green*i * 255.0) as u8;
            let color = Color32::from_rgb(r, g, 0);
            egui::Window::new("Monitor").default_height(200.0).anchor(Align2::RIGHT_TOP, [0.0, 0.0]).show(egui_ctx, |win| {
                win.vertical(|ui| {
                    let dt = (get_frame_time()*100.0).round()/100.0;
                    //let fps = get_fps();
                    let txt = format!("dT: {} | FPS: {}({})", dt, fps, fps2);
                    ui.add(Label::new(RichText::new(txt).color(color).strong()));
                })
            });
        }
    }

    fn build_settings_win(&mut self, egui_ctx: &Context) {
        if !self.settings_win {
            return;
        }
        let mut settings = get_settings();
        let w = 200.0; let h = 400.0;
        egui::Window::new("SETTINGS").id("settings_win".into()).default_pos((SCREEN_WIDTH/2.-w/2., 0.0)).default_size(vec2(w, h))
        .title_bar(true).show(egui_ctx, |ui| {
            //ui.set_height_range(300.0..=600.0);
            //ui.set_min_width(w-50.0);
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(60., 25.));
                column[1].set_min_size(UIVec2::new(125., 25.));
                let mut world_radius = settings.world_radius;
                column[0].label(RichText::new("WORLD RANGE").color(Color32::YELLOW).strong());
                if column[1].add_sized(vec2(125., 25.), Slider::new(&mut world_radius, 100.0..=4000.0).step_by(100.0)).changed() {
                    settings.world_radius = world_radius;
                    set_settings(settings);
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(60., 25.));
                column[1].set_min_size(UIVec2::new(120., 25.));
                let mut particles_num = settings.particles_num;
                column[0].label(RichText::new("PARTICLES NUMBER").color(Color32::YELLOW).strong());
                if column[1].add_sized(vec2(125., 25.), Slider::new(&mut particles_num, 0..=10000).step_by(10.0)).changed() {
                    settings.particles_num = particles_num;
                    set_settings(settings);
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(60., 25.));
                column[1].set_min_size(UIVec2::new(120., 25.));
                let mut particle_size = settings.particle_size;
                column[0].label(RichText::new("PARTICLES SIZE").color(Color32::RED).strong());
                if column[1].add_sized(vec2(125., 25.), Slider::new(&mut particle_size, 0.1..=5.0).step_by(0.1)).changed() {
                    settings.particle_size = particle_size;
                    let mut signals = signals();
                    signals.particles_new_settings = true;
                    set_settings(settings);
                    set_signals(signals);
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(60., 25.));
                column[1].set_min_size(UIVec2::new(120., 25.));
                let mut particle_dense = settings.particle_dense;
                column[0].label(RichText::new("PARTICLES DENSE").color(Color32::BLUE).strong());
                if column[1].add_sized(vec2(125., 25.), Slider::new(&mut particle_dense, 0.1..=5.0).step_by(0.1)).changed() {
                    settings.particle_dense = particle_dense;
                    let mut signals = signals();
                    signals.particles_new_settings = true;
                    set_settings(settings);
                    set_signals(signals);
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(60., 25.));
                column[1].set_min_size(UIVec2::new(120., 25.));
                let mut particle_types = settings.particle_types;
                column[0].label(RichText::new("PARTICLE TYPES").color(Color32::LIGHT_BLUE).strong());
                if column[1].add_sized(vec2(125., 25.), Slider::new(&mut particle_types, 1..=50)).changed() {
                    settings.particle_types = particle_types;
                    set_settings(settings);
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(60., 25.));
                column[1].set_min_size(UIVec2::new(120., 25.));
                let mut field_radius = settings.field;
                column[0].label(RichText::new("FIELD RADIUS").color(Color32::BLUE).strong());
                if column[1].add_sized(vec2(125., 25.), Slider::new(&mut field_radius, 0.0..=500.0).step_by(1.0)).changed() {
                    settings.field = field_radius;
                    set_settings(settings);
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(60., 25.));
                column[1].set_min_size(UIVec2::new(120., 25.));
                let mut force = settings.force;
                column[0].label(RichText::new("FORCE").color(Color32::GREEN).strong());
                if column[1].add_sized(vec2(125., 25.), Slider::new(&mut force, 0.0..=100.0).step_by(1.0)).changed() {
                    settings.force = force;
                    set_settings(settings);
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(60., 25.));
                column[1].set_min_size(UIVec2::new(120., 25.));
                let mut repel = settings.repel;
                column[0].label(RichText::new("REPEL").color(Color32::RED).strong());
                if column[1].add_sized(vec2(125., 25.), Slider::new(&mut repel, 0.0..=1.0).step_by(0.01)).changed() {
                    settings.repel = repel;
                    set_settings(settings);
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(60., 25.));
                column[1].set_min_size(UIVec2::new(120., 25.));
                let mut damping = settings.damping;
                column[0].label(RichText::new("DAMPING").color(Color32::DARK_BLUE).strong());
                if column[1].add_sized(vec2(125., 25.), Slider::new(&mut damping, 0.0..=4.0).step_by(0.1)).changed() {
                    settings.damping = damping;
                    set_settings(settings);
                    let mut signals = signals();
                    signals.particles_new_settings = true;
                    set_signals(signals);
                }
            });
        });
    }

    fn build_about_win(&mut self, egui_ctx: &Context) {
        if self.about_win {
            Window::new("ABOUT").resizable(false).default_pos((SCREEN_WIDTH/2.-150., SCREEN_HEIGHT/6.)).min_height(680.).min_width(120.)
            .title_bar(true).show(egui_ctx, |ui| {
                let big_logo = self.big_logo.clone().unwrap();
                ui.vertical_centered(|pic| {
                    pic.image(big_logo.id(), big_logo.size_vec2());
                });
                ui.add_space(2.0);
                ui.vertical_centered(|title| {
                    title.heading(RichText::new("MAD LAB").color(Color32::GREEN).strong());
                });
                ui.vertical_centered(|author| {
                    author.label(RichText::new("Artur Gwoździowski 2019-2024").color(Color32::LIGHT_BLUE).strong());
                });
                ui.add_space(2.0);
                ui.vertical_centered(|author| {
                    author.label(RichText::new(format!("version {}", env!("CARGO_PKG_VERSION"))).color(Color32::YELLOW).italics());
                });
                ui.add_space(2.0);
                ui.vertical_centered(|closer| {
                    if closer.button(RichText::new("CLOSE").color(Color32::GREEN).strong()).clicked() {
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
            ui.ctx().load_texture("science", ColorImage::example(), Default::default())
        });
        ui.add(egui_macroquad::egui::Image::new(texture, texture.size_vec2()));
        ui.image(texture, texture.size_vec2());
    }
}