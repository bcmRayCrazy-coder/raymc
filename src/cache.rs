use std::{collections::HashMap, sync::RwLock};

use iced::widget::image;
use once_cell::sync::Lazy;

use crate::embed::get_image_handle;

pub struct CachedAsset {
    image_handle: HashMap<String, image::Handle>,
}

impl CachedAsset {
    pub fn new() -> Self {
        Self {
            image_handle: HashMap::new(),
        }
    }

    pub fn cache_image_handle(&mut self, path: &str, image_handle: image::Handle) {
        self.image_handle.insert(path.to_string(), image_handle);
    }
}

static CACHED_ASSET: Lazy<RwLock<CachedAsset>> = Lazy::new(|| RwLock::new(CachedAsset::new()));

fn load_cache_image(cached_asset: &mut CachedAsset, file_path: &str) {
    cached_asset.cache_image_handle(
        file_path,
        get_image_handle(file_path).expect("Missing basic assets!"),
    );
}

pub fn get_cached_image_handle(path: &str) -> Option<image::Handle> {
    let cached_asset = CACHED_ASSET.read().ok()?;
    let image_handle = cached_asset.image_handle.get(&path.to_string());
    image_handle.cloned()
}

pub fn get_cached_image_handle_list() -> Vec<image::Handle> {
    let cached_asset = CACHED_ASSET.read().unwrap();
    cached_asset.image_handle.values().cloned().collect()
}

const CACHED_IMAGES: [&str; 18] = [
    "icons/album.png",
    "icons/options.png",
    "icons/playlist.png",
    "icons/quit.png",
    "icons/play.png",
    "icons/play_disabled.png",
    "icons/pause.png",
    "icons/pause_disabled.png",
    "icons/key1.png",
    "icons/key2.png",
    "icons/key3.png",
    "icons/keyl.png",
    "icons/keym.png",
    "icons/keyr.png",
    "icon.png",
    "bg.png",
    "menu_icon_bg.png",
    "user.png",
];

pub fn load_cached_asset() {
    println!("Loading Cached Assets. {} images.", CACHED_IMAGES.len());

    let mut cached_asset = CACHED_ASSET.write().unwrap();

    for image_path in CACHED_IMAGES {
        load_cache_image(&mut cached_asset, image_path);
    }
}
