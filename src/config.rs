use std::path::PathBuf;
use std::{env, fs};

static APP_PATH_NAME: &str = "raymc";

pub fn get_app_dir() -> (PathBuf, bool) {
    let mut app_dir = PathBuf::new();

    #[cfg(target_os = "windows")]
    {
        if let Ok(path) = env::var("APPDATA") {
            app_dir = PathBuf::from(path).join(APP_PATH_NAME);
        } else {
            app_dir = PathBuf::from(
                env::current_dir()
                    .unwrap_or(PathBuf::from("."))
                    .join(APP_PATH_NAME),
            );
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        if let Ok(path) = env::var("XDG_CONFIG_HOME") {
            if !path.trim().is_empty() {
                app_dir = PathBuf::from(path).join(APP_PATH_NAME);
            }
        } else if let Ok(path) = env::var("HOME") {
            app_dir = PathBuf::from(path).join(APP_PATH_NAME);
        } else {
            app_dir = PathBuf::from(
                env::current_dir()
                    .unwrap_or(PathBuf::from("."))
                    .join(APP_PATH_NAME),
            );
        }
    }

    if !app_dir.exists() || !app_dir.is_dir() {
        fs::create_dir_all(&app_dir).expect("Unable to create app dir!");
        println!("Created app dir");
        return (app_dir, true);
    }

    (app_dir, false)
}

pub fn get_app_subdir(subdir: &str) -> (PathBuf, bool) {
    let app_subdir = get_app_dir().0.join(subdir);
    if !app_subdir.exists() || !app_subdir.is_dir() {
        fs::create_dir_all(&app_subdir).expect("Unable to create app sub dir!");
        println!("Created empty dir {:?}", app_subdir);
        return (app_subdir, true);
    }
    (app_subdir, false)
}
