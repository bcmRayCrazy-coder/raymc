use std::collections::HashMap;

use iced::{Element, Length::Fill, Renderer, Theme, widget};

use crate::{
    cache,
    ui::{app::QuickKey, message::Message, widget::icon_text::IconText},
};

#[derive(Clone)]
pub struct KeysHint {
    pub keys: HashMap<QuickKey, String>,
}

impl KeysHint {
    fn get_item(&self, key: QuickKey, icon: &'static str) -> Element<'_, Message, Theme, Renderer> {
        match self.keys.get(&key) {
            Some(text) => IconText::new(
                text,
                cache::get_cached_image_handle(icon).expect("Missing resource: key icon"),
            )
            .width(Fill)
            .widget(),
            None => widget::space().width(Fill).into(),
        }
    }

    pub fn widget(&self) -> Element<'_, Message, Theme, Renderer> {
        widget::row![
            self.get_item(QuickKey::KEY0, "icons/key1.png"),
            self.get_item(QuickKey::KEY1, "icons/key2.png"),
            self.get_item(QuickKey::KEY2, "icons/key3.png"),
            self.get_item(QuickKey::KEYL, "icons/keyl.png"),
            self.get_item(QuickKey::KEYM, "icons/keym.png"),
            self.get_item(QuickKey::KEYR, "icons/keyr.png"),
        ]
        .spacing(10)
        .width(Fill)
        .into()
    }
}

impl Default for KeysHint {
    fn default() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }
}
