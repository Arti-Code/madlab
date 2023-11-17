use egui_macroquad::{*, egui::{Context, TopBottomPanel, RichText, Color32, menu, Align2, Label, Slider, Window}};
use egui_macroquad::egui::Vec2 as UIVec2;
use macroquad::time::{get_frame_time, get_fps}; 
use crate::globals::*;

pub struct UI {
    pointer_over: bool,
    monitor_win: bool,
    settings_win: bool,
}


impl UI {

    pub fn new() -> Self {
        Self {
            pointer_over: false,
            monitor_win: false,
            settings_win: false,
        }
    }

    pub fn process(&mut self, fps: i32, fps2: i32) {
        egui_macroquad::ui(|egui_ctx| {
            self.pointer_over = egui_ctx.is_pointer_over_area();
            self.build_top_menu(egui_ctx);
            self.build_monitor_win(egui_ctx, fps, fps2);
            self.build_settings_win(egui_ctx);
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
                ui.label(RichText::new("MAD LAB").heading().strong().color(Color32::RED));
                ui.separator();
                
                menu::menu_button(ui, RichText::new("SIM").strong(), |ui| {
                    if ui.button(RichText::new("Restart").strong().color(Color32::GREEN)).clicked() {
                        signals.restart = true;
                        set_global_signals(signals);
                    }
                    if ui.button(RichText::new("Quit").strong().color(Color32::RED)).clicked() {
                        std::process::exit(0);
                        //signals.quit = true;
                        //set_global_signals(signals);
                    }
                });

                menu::menu_button(ui, RichText::new("WORLD").strong(), |ui| {
                    if ui.button(RichText::new("Shuffle interactions").strong().color(Color32::GREEN)).clicked() {
                        signals.shuffle_interactions = true;
                        set_global_signals(signals);
                    }
                    if ui.button(RichText::new("Monitor").strong().color(Color32::GREEN)).clicked() {
                        self.monitor_win = !self.monitor_win;
                    }
                    if ui.button(RichText::new("Settings").strong().color(Color32::GREEN)).clicked() {
                        self.settings_win = !self.settings_win;
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
                    ui.add(Label::new(RichText::new(txt).color(Color32::BLUE).strong()));
                })
            });
        }
    }

    fn build_settings_win(&mut self, egui_ctx: &Context) {
        if !self.settings_win {
            return;
        }
        let mut settings = get_settings();
        egui::Window::new("SETTINGS").id("settings_win".into()).default_pos((SCREEN_WIDTH/2., SCREEN_HEIGHT/2.)).fixed_size([380., 400.])
        .title_bar(true).show(egui_ctx, |ui| {
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut field_radius = settings.field;
                column[0].label(RichText::new("FIELD RADIUS").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut field_radius, 0.0..=500.0).step_by(10.0)).changed() {
                    settings.field = field_radius;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut field_radius = settings.field;
                column[0].label(RichText::new("FORCE").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut field_radius, 0.0..=500.0).step_by(10.0)).changed() {
                    settings.field = field_radius;
                    signals.new_settings = true;
                }
            });
        });
    }
}