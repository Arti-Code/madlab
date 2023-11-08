use egui_macroquad::{*, egui::{Context, TopBottomPanel, RichText, Color32, menu, Align2, Label}};
use macroquad::time::{get_frame_time, get_fps}; 
use crate::globals::*;

pub struct UI {
    pointer_over: bool,
    monitor_win: bool,
}


impl UI {

    pub fn new() -> Self {
        Self {
            pointer_over: false,
            monitor_win: false,
        }
    }

    pub fn process(&mut self, fps: i32, fps2: i32) {
        egui_macroquad::ui(|egui_ctx| {
            self.pointer_over = egui_ctx.is_pointer_over_area();
            self.build_top_menu(egui_ctx);
            self.build_monitor_win(egui_ctx, fps, fps2);
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
                    if ui.button(RichText::new("Monitor").strong().color(Color32::GREEN)).clicked() {
                        self.monitor_win = !self.monitor_win;
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

}