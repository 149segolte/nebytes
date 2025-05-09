#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use egui::{Align, Layout, Rect, RichText, Separator, UiBuilder, Vec2, pos2, vec2};

fn main() -> eframe::Result {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "nebytes",
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
        //         panel_fill: egui::Color32::from_rgb(16, 16, 16),
        //         ..Default::default()
        //     },
        // );

        egui_extras::install_image_loaders(&cc.egui_ctx);

        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "noto_sans".to_owned(),
            std::sync::Arc::new(
                // .ttf and .otf supported
                egui::FontData::from_static(include_bytes!(
                    "../assets/fonts/NotoSans_Condensed-Medium.ttf"
                )),
            ),
        );
        fonts.font_data.insert(
            "jetbrains_mono".to_owned(),
            std::sync::Arc::new(
                // .ttf and .otf supported
                egui::FontData::from_static(include_bytes!(
                    "../assets/fonts/JetBrainsMono-Regular.ttf"
                )),
            ),
        );

        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "noto_sans".to_owned());
        fonts
            .families
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .insert(0, "jetbrains_mono".to_owned());
        cc.egui_ctx.set_fonts(fonts);

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
            ui.spacing_mut().item_spacing = Vec2::new(0.0, 0.0);
            let start = ui.cursor().min;
            let split_width = ui.available_width() / 2.0;

            let mut split1 = ui.new_child(
                UiBuilder::new()
                    .max_rect(Rect::from_min_max(
                        start,
                        start + vec2(split_width, ui.available_height()),
                    ))
                    .layout(Layout::top_down(Align::Center).with_main_align(Align::Center)),
            );
            split1.label("Part A");

            let mut spacer = ui.new_child(
                UiBuilder::new()
                    .max_rect(Rect::from_min_max(
                        start + vec2(split_width - 1.0, 0.0),
                        start + vec2(split_width + 1.0, ui.available_height()),
                    ))
                    .layout(Layout::left_to_right(Align::Center)),
            );
            spacer.set_width(2.0);
            spacer.set_height(ui.available_height());
            spacer.add(Separator::default().shrink(32.0).spacing(2.0));

            let mut split2 = ui.new_child(
                UiBuilder::new()
                    .max_rect(Rect::from_min_max(
                        start + vec2(split_width, 0.0),
                        start + ui.available_size(),
                    ))
                    .layout(Layout::top_down(Align::Center).with_main_align(Align::Center)),
            );
            split2.set_height(ui.available_height());
            split2.add(
                egui::Image::new(egui::include_image!("../assets/logo.png"))
                    .maintain_aspect_ratio(true)
                    .fit_to_exact_size(Vec2::new(96.0, 96.0)),
            );
            split2.add_space(8.0);
            split2.label(RichText::new("Welcome to NEbytes!").size(24.0));
        });
    }
}
