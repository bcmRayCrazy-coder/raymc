use iced::widget::image;
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "assets/embed"]
struct EmbedAsset;

pub fn get_image_handle(file_path: &str) -> Option<image::Handle> {
    let file = EmbedAsset::get(file_path)?;
    Some(image::Handle::from_bytes(file.data.into_owned()))
}
