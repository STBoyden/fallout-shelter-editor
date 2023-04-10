use components::dwellers_view;
use crypt::{decrypt, encrypt};
use data::Save;
use eframe::egui;

mod components;
mod crypt;
mod data;

fn main() {
    let options = eframe::NativeOptions::default();
    _ = eframe::run_native(
        "Fallout Shelter Editor",
        options,
        Box::new(|cc| Box::new(App::new(cc))),
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    NoOp,
    Vault,
    Dwellers,
}

impl Default for State {
    fn default() -> Self { Self::NoOp }
}

#[derive(Default)]
struct App {
    save_data: Option<Save>,
    ui_state: State,
}

impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self { Self::default() }

    fn load_file(&mut self) {
        let file = rfd::FileDialog::new()
            .add_filter("Fallout Shelter Save File", &["sav"])
            .pick_file();

        if file.is_none() {
            rfd::MessageDialog::new()
                .set_title("Error")
                .set_description("No file selected.")
                .show();

            return;
        }

        let file = file.unwrap();
        let decrypted = decrypt(file);

        if let Err(e) = decrypted {
            rfd::MessageDialog::new()
                .set_title("Error")
                .set_description(&format!("Could not decrypt file: {}", e))
                .show();

            return;
        }

        let decrypted = decrypted.unwrap();
        self.save_data = Some(decrypted);
    }

    fn save_file(&self) {
        let file = rfd::FileDialog::new()
            .add_filter("Fallout Shelter Save File", &["sav"])
            .save_file();

        if file.is_none() {
            rfd::MessageDialog::new()
                .set_title("Error")
                .set_description("No file selected.")
                .show();

            return;
        }

        assert!(self.save_data.is_some());

        let file = file.unwrap();
        let save_data = self.save_data.as_ref().unwrap();

        let res = encrypt(save_data, file);

        if let Err(e) = res {
            rfd::MessageDialog::new()
                .set_title("Error")
                .set_description(&format!("Could not encrypt file: {}", e))
                .show();

            return;
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.group(|ui| {
                ui.horizontal_wrapped(|ui| {
                    if ui.button("Load file").clicked() {
                        self.load_file();
                    }

                    ui.add_enabled_ui(self.save_data.is_some(), |ui| {
                        if ui.button("Save file").clicked() {
                            self.save_file();
                        }
                    });
                });
            });

            if let Some(data) = &mut self.save_data {
                ui.horizontal_wrapped(|ui| {
                    ui.heading(format!("Vault {}", data.vault.vault_name));
                });
                ui.separator();
                ui.horizontal_wrapped(|ui| {
                    ui.selectable_value(&mut self.ui_state, State::Vault, "Vault");
                    ui.selectable_value(&mut self.ui_state, State::Dwellers, "Dwellers");
                });

                match self.ui_state {
                    State::NoOp => {
                        self.ui_state = State::Vault;
                    },
                    State::Vault => {},
                    State::Dwellers => dwellers_view(ui, data),
                }
            }
        });
    }
}
