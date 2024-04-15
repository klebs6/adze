use reflex::*;
use reflex::imports::*;

fn main() {

    let youtube_url = determine_youtube_url();

    let app = MyApp::new(youtube_url);

    println!("app: {:#?}", app);

    let options = eframe::NativeOptions {
        initial_window_size: Some(eframe::egui::vec2(300.0, 100.0)),
        ..Default::default()
    };

    eframe::run_native(Box::new(app), options);
}
