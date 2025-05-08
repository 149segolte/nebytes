#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use egui::Vec2;

fn main() -> eframe::Result {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "eframe test",
        native_options,
        Box::new(|cc| Ok(Box::new(AppState::new(cc)))),
    )
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct AppState {}

impl Default for AppState {
    fn default() -> Self {
        Self {}
    }
}

impl AppState {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // cc.egui_ctx.set_visuals_of(
        //     egui::Theme::Dark,
        //     egui::Visuals {
        //         panel_fill: egui::Color32::RED,
        //         ..Default::default()
        //     },
        // );

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for AppState {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                let size = ui.available_size();
                ui.add_sized(Vec2::new(size.x / 2.0, size.y), egui::Label::new("Part A"));
                ui.add_sized(Vec2::new(1.0, size.y - 64.0), egui::Separator::default());
                ui.add_sized(Vec2::new(size.x / 2.0, size.y), egui::Label::new("Part B"));
            });

            egui::SidePanel::left("file_handler")
                .resizable(false)
                .show_inside(ui, |ui| {
                    ui.label(
                        "The scroll area below has many labels with interactive tooltips. \
                  The purpose is to test that the tooltips close when you scroll.",
                    )
                    .on_hover_text("Try hovering a label below, then scroll!");
                    egui::ScrollArea::vertical()
                        .auto_shrink(false)
                        .show(ui, |ui| {
                            for i in 0..1000 {
                                ui.label(format!("This is line {i}")).on_hover_ui(|ui| {
                                    ui.style_mut().interaction.selectable_labels = true;
                                    ui.label(
                            "This tooltip is interactive, because the text in it is selectable.",
                        );
                                });
                            }
                        });
                });

            ui.heading("eframe template");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
            });

            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/main/",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
