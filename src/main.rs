mod app;
mod cache;
mod embed;

use app::app::App;
use iced::{
    Font, Theme, application,
    window::{Icon, Settings, icon},
};

fn load_app_icon(path: &str) -> Option<Icon> {
    let img = image::open(path).ok()?;
    icon::from_rgba(img.to_rgba8().to_vec(), 256, 256).ok()
}

fn main() -> iced::Result {
    println!("Start Application Ray Music Center");

    cache::load_cached_asset();

    let app_settings = Settings {
        // fullscreen: true,
        icon: load_app_icon("./assets/icon.png"),
        ..Default::default()
    };

    application(App::boot, App::update, App::view)
        .theme(|_app: &App| Theme::Dark)
        .subscription(App::subscription)
        .antialiasing(true)
        .default_font(Font::from(Font::with_name("微软雅黑")))
        .title("Ray Music Center")
        .window_size((800, 600))
        .window(app_settings)
        .run()
}
