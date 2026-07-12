mod audio;
mod bluetooth;
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

fn load_app_icon() -> Option<Icon> {
    let img = image::load_from_memory_with_format(
        &embed::get_embed_file("icon.png").unwrap().data,
        image::ImageFormat::Png,
    )
    .ok()?
    .to_rgba8();

    let size = img.dimensions();
    println!("Icon size {:?}", size);

    icon::from_rgba(img.into_raw(), size.0, size.1).ok()
}

fn main() -> iced::Result {
    println!("Start Application Ray Music Center");

    cache::load_cached_asset();

    let app_settings = Settings {
        // fullscreen: true,
        icon: load_app_icon(),
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
