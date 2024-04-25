use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

pub struct Theme {
    pub theme_dir: PathBuf,
    pub user_theme_dir: PathBuf,
}

impl Theme {
    fn new() -> Self {
        Theme {
            theme_dir: PathBuf::new(),
            user_theme_dir: PathBuf::new(),
        }
    }

    pub fn get_instance() -> Arc<Mutex<Theme>> {
        static mut instance: Option<Arc<Mutex<Theme>>> = None;
        unsafe {
            instance
                .get_or_insert_with(|| Arc::new(Mutex::new(Theme::new())))
                .clone()
        }
    }

    pub fn set_theme_dir(&mut self, theme_dir: PathBuf) {
        self.theme_dir = theme_dir;
    }

    pub fn get_theme_dir(&self) -> &PathBuf {
        &self.theme_dir
    }

    pub fn set_user_dir(&mut self, dir_path: PathBuf) {
        self.user_theme_dir = dir_path;
    }

    pub fn get_user_dir(&self) -> &PathBuf {
        &self.user_theme_dir
    }

    pub fn clear_user_dir(&mut self) {
        self.user_theme_dir.clear();
    }

    pub fn clear_theme_dir(&mut self) {
        self.theme_dir.clear();
    }
}
