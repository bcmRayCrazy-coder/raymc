mod audio;
mod cache;
mod config;
mod embed;
mod player;
mod ui;

extern crate iced;
extern crate iced_anim;
extern crate once_cell;
extern crate rust_embed;

use iced::{
    Font, Theme, application,
    window::{Icon, Settings, icon},
};
use ui::app::App;

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

    let app = application(App::boot, App::update, App::view)
        .theme(|_app: &App| Theme::Dark)
        .subscription(App::subscription)
        .antialiasing(true)
        .default_font(Font::from(Font::with_name("微软雅黑")))
        .title("Ray Music Center")
        .window_size((1024, 768))
        .window(app_settings);
    app.run()
}
