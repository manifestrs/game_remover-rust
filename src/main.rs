<<<<<<< HEAD
use eframe::egui;
use egui_async::EguiAsyncPlugin;

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };
    eframe::run_native(
        "bettah than bakains game remover!!!",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<App>::default())
        }),
    )
}

#[derive(Debug, Default)]
struct App {
    app_id: String, // we'll cast to u32 later
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // add async support
        ctx.plugin_or_default::<EguiAsyncPlugin>();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("steam gaems removah, but very sussy baka");
            ui.horizontal(|ui| {
                let name_label = ui.label("app id: ");
                ui.text_edit_singleline(&mut self.app_id)
                    .labelled_by(name_label.id)
            });
            ui.label(format!(
                "ur app id is {} but i aint uninstalling it lmao",
                &self.app_id
            ));

            ui.add_space(15.);
            ui.label(egui::RichText::new("rewrite of sobakins game remover: https://github.com/Manifestor-cc/game-remover/").color(egui::Color32::DARK_GRAY));
            ui.label(egui::RichText::new("man idk how to make hyperlinks").color(egui::Color32::DARK_GRAY));
        });
=======
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(app);
}

#[component]
fn app() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        h1 { "hi" }
        h2 { "making sure everything works!" }
>>>>>>> 504caac (gl moving to dioxus)
    }
}
