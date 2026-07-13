use std::{fmt::Debug, path::PathBuf};

use souvlaki::{MediaControls, MediaMetadata, PlatformConfig};

use crate::player::song::PlaySong;

fn generate_cover_url(path: &PathBuf) -> Option<&str> {
    let str_path = path.to_str()?;

    #[cfg(target_os = "windows")]
    {
        return Some(("file://".to_owned() + str_path).leak());
    }

    #[cfg(target_os = "linux")]
    {
        return Some(("file://".to_owned() + str_path).leak());
    }

    // Unimplement: MacOS

    #[allow(unused)]
    None
}

#[derive(Debug)]
#[allow(unused)]
pub enum MediaControlError {
    ControlsError,
    AttachError(souvlaki::Error),
    DetachError(souvlaki::Error),
    SetMetadataError(souvlaki::Error),
}

pub struct MediaControlManager {
    controls: Box<MediaControls>,
}

impl MediaControlManager {
    pub fn new() -> Result<Self, MediaControlError> {
        let config = PlatformConfig {
            display_name: "Ray Media Center",
            dbus_name: "raymc",
            hwnd: None,
        };

        Ok(Self {
            controls: Box::new(
                MediaControls::new(config).map_err(|_| MediaControlError::ControlsError)?,
            ),
        })
    }

    pub fn set_song(&mut self, song: PlaySong) -> Result<(), MediaControlError> {
        let cover_url = match song.cover.as_ref() {
            None => None,
            Some(path) => generate_cover_url(path),
        };

        self.controls
            .set_metadata(MediaMetadata {
                title: Some(song.name()),
                cover_url: cover_url,
                ..Default::default()
            })
            .map_err(|err| MediaControlError::SetMetadataError(err))
    }

    pub fn attach(&mut self) -> Result<(), MediaControlError> {
        self.controls
            .attach(|event| {
                //
                println!("Control event {:?}", event);
            })
            .map_err(|err| MediaControlError::AttachError(err))
    }

    pub fn detach(&mut self) -> Result<(), MediaControlError> {
        self.controls
            .detach()
            .map_err(|err| MediaControlError::DetachError(err))
    }
}
