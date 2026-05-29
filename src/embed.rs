use iced::widget::image;
use rust_embed::{Embed, EmbeddedFile};

#[derive(Embed)]
#[folder = "assets/embed"]
pub struct EmbedAsset;

pub fn get_embed_file(file_path: &str) -> Option<EmbeddedFile> {
    EmbedAsset::get(file_path)
}

pub fn get_image_handle(file_path: &str) -> Option<image::Handle> {
    let file = get_embed_file(file_path)?;
    Some(image::Handle::from_bytes(file.data.into_owned()))
}

