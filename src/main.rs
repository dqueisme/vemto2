use eframe::egui;
use vemto_core::ui::VemtoApp;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("Vemto - Laravel Code Generator"),
        ..Default::default()
    };

    eframe::run_native(
        "Vemto",
        native_options,
        Box::new(|cc| Ok(Box::new(VemtoApp::new(cc)))),
    )
}
