use crate::data::{DwellerGender, DwellerSpecial, Save};
use eframe::egui;
use egui_extras::{Size, StripBuilder};

pub fn dwellers_view(ui: &mut egui::Ui, data: &mut Save) {
    ui.horizontal(|ui| {
        if ui
            .button("Max all dweller stats, happiness, health and levels")
            .clicked()
        {
            data.dwellers.dwellers.iter_mut().for_each(|d| {
                d.stats
                    .stats
                    .iter_mut()
                    .skip(1)
                    .filter(|stat| stat.value != 10)
                    .for_each(|stat| stat.value = 10);

                d.health.max_health = 500.0;
                d.health.health = 500.0;
                d.health.radiation = 1.0;
                d.happiness.happiness_value = 100.0;
                d.experience.current_level = 50;
            });
        }

        if ui.button("Max happiness for all dwellers").clicked() {
            data.dwellers
                .dwellers
                .iter_mut()
                .filter(|d| d.happiness.happiness_value != 100.0)
                .for_each(|d| d.happiness.happiness_value = 100.0);
        }

        if ui.button("Max SPECIAL stats for all dwellers").clicked() {
            data.dwellers.dwellers.iter_mut().for_each(|d| {
                d.stats
                    .stats
                    .iter_mut()
                    .skip(1)
                    .filter(|stat| stat.value != 10)
                    .for_each(|stat| stat.value = 10)
            });
        }

        if ui.button("Min happiness for all dwellers").clicked() {
            data.dwellers
                .dwellers
                .iter_mut()
                .filter(|d| d.happiness.happiness_value != 1.0)
                .for_each(|d| d.happiness.happiness_value = 1.0);
        }

        if ui.button("Min SPECIAL stats for all dwellers").clicked() {
            data.dwellers.dwellers.iter_mut().for_each(|d| {
                d.stats
                    .stats
                    .iter_mut()
                    .skip(1)
                    .filter(|stat| stat.value != 1)
                    .for_each(|stat| stat.value = 1)
            });
        }

        if ui.button("Toggle all female dwellers pregnant").clicked() {
            data.dwellers
                .dwellers
                .iter_mut()
                .filter(|d| d.gender == DwellerGender::Female)
                .for_each(|d| {
                    d.pregnant = !d.pregnant;

                    if !d.pregnant {
                        d.baby_ready = false;
                    }
                });
        }

        if ui
            .button("Toggle all pregnant dwellers ready to give birth")
            .clicked()
        {
            data.dwellers
                .dwellers
                .iter_mut()
                .filter(|d| d.pregnant && d.gender == DwellerGender::Female)
                .for_each(|d| d.baby_ready = !d.baby_ready);
        }
    });

    egui::ScrollArea::both().show(ui, |ui| {
        StripBuilder::new(ui)
            .sizes(
                Size::remainder().at_least(150.0),
                data.dwellers.dwellers.len(),
            )
            .vertical(|mut strip| {
                data.dwellers.dwellers.iter_mut().for_each(|dweller| {
                    strip.strip(|builder| {
                        builder
                            .sizes(Size::remainder().at_least(300.0), 4)
                            .horizontal(|mut strip| {
                                strip.cell(|ui| {
                                    ui.text_edit_singleline(&mut dweller.first_name);
                                    ui.text_edit_singleline(&mut dweller.last_name);
                                });

                                strip.cell(|ui| {
                                    ui.vertical(|ui| {
                                        egui::ComboBox::new(
                                            format!(
                                                "{} {}",
                                                dweller.first_name, dweller.last_name
                                            ),
                                            "Gender",
                                        )
                                        .selected_text(dweller.gender.to_string())
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(
                                                &mut dweller.gender,
                                                DwellerGender::Male,
                                                DwellerGender::Male.to_string(),
                                            );
                                            ui.selectable_value(
                                                &mut dweller.gender,
                                                DwellerGender::Female,
                                                DwellerGender::Female.to_string(),
                                            )
                                        });

                                        ui.add(
                                            egui::Slider::new(
                                                &mut dweller.health.max_health,
                                                1.0..=500.0,
                                            )
                                            .trailing_fill(true)
                                            .integer()
                                            .text("Max health"),
                                        );

                                        ui.checkbox(
                                            &mut dweller.experience.need_lvl_up,
                                            "Need level up",
                                        );

                                        if dweller.gender == DwellerGender::Female {
                                            ui.group(|ui| {
                                                ui.checkbox(
                                                    &mut dweller.pregnant,
                                                    "Pregnant",
                                                );

                                                ui.checkbox(
                                                    &mut dweller.baby_ready,
                                                    "Baby ready",
                                                );
                                            });
                                        }
                                    });
                                });

                                strip.cell(|ui| {
                                    ui.group(|ui| {
                                        ui.horizontal(|ui| {
                                            dweller
                                                .stats
                                                .stats
                                                .iter_mut()
                                                .enumerate()
                                                .skip(1)
                                                .for_each(|(index, stat)| {
                                                    ui.add(
                                                        egui::Slider::new(
                                                            &mut stat.value,
                                                            1..=10,
                                                        )
                                                        .vertical()
                                                        .text(
                                                            DwellerSpecial::from_index(
                                                                index,
                                                            )
                                                            .to_string(),
                                                        )
                                                        .trailing_fill(true),
                                                    );
                                                });

                                            ui.separator();

                                            ui.add(
                                                egui::Slider::new(
                                                    &mut dweller
                                                        .happiness
                                                        .happiness_value,
                                                    0.0..=100.0,
                                                )
                                                .vertical()
                                                .integer()
                                                .text("Happiness")
                                                .trailing_fill(true),
                                            );

                                            ui.separator();

                                            ui.add(
                                                egui::Slider::new(
                                                    &mut dweller.experience.current_level,
                                                    1..=50,
                                                )
                                                .vertical()
                                                .integer()
                                                .text("Level")
                                                .trailing_fill(true),
                                            );

                                            ui.separator();

                                            ui.add(
                                                egui::Slider::new(
                                                    &mut dweller.health.health,
                                                    1.0..=dweller.health.max_health,
                                                )
                                                .vertical()
                                                .integer()
                                                .text("Health")
                                                .trailing_fill(true),
                                            );

                                            ui.add(
                                                egui::Slider::new(
                                                    &mut dweller.health.radiation,
                                                    1.0..=dweller.health.max_health - 1.0,
                                                )
                                                .vertical()
                                                .integer()
                                                .text("Radiation")
                                                .trailing_fill(true),
                                            );
                                        });
                                    });
                                });
                            });
                    });
                })
            });
    });
}
