// Disable the console window on Windows in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;

fn main() -> eframe::Result<()> {
    // Set up the options for the native window.
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 600.0]) // Set a default window size.
            .with_min_inner_size([600.0, 400.0]), // Set a minimum window size.
        ..Default::default()
    };

    // Run the eframe application.
    eframe::run_native(
        "URL Tracker", // The title of the window.
        native_options,
        // The Box::new closure creates the application state.
        Box::new(|_cc| Ok(Box::<app::UrlTrackerApp>::default())),
    )
}