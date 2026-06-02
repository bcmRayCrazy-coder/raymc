use std::path::PathBuf;
use std::{env, fs};

static APP_PATH_NAME: &str = "raymc";

pub fn get_app_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        if let Ok(path) = env::var("APPDATA") {
            return PathBuf::from(path).join(APP_PATH_NAME);
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        if let Ok(path) = env::var("XDG_CONFIG_HOME") {
            if !path.trim().is_empty() {
                return PathBuf::from(path).join(APP_PATH_NAME);
            }
        }
        if let Ok(path) = env::var("HOME") {
            return PathBuf::from(path).join(".config").join(APP_PATH_NAME);
        }
    }

    PathBuf::from(
        env::current_dir()
            .unwrap_or(PathBuf::from("."))
            .join(APP_PATH_NAME),
    )
}

pub fn init_app_dir() {
    let app_dir = get_app_dir();
    println!("Current app dir {:?}", app_dir);

    if !app_dir.exists() {
        fs::create_dir_all(app_dir).expect("Unable to create app dir!");
        println!("App dir created");
    }
}
