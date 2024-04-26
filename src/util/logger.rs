use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

pub struct Logger {
    pub log_file: PathBuf,
    pub log_levels: Vec<String>,
}

impl Logger {
    fn new() -> Self {
        Logger {
            log_file: PathBuf::new(),
            log_levels: vec![
                "DISABLED".to_owned(),
                "ERROR".to_owned(),
                "WARNING".to_owned(),
                "INFO".to_owned(),
                "DEBUG".to_owned(),
            ],
        }
    }

    pub fn get_instance() -> Arc<Mutex<Logger>> {
        static mut instance: Option<Arc<Mutex<Logger>>> = None;
        unsafe {
            instance
                .get_or_insert_with(|| Arc::new(Mutex::new(Logger::new())))
                .clone()
        }
    }

    pub fn set_file(&mut self, file_path: PathBuf) {
        self.log_file = file_path;
    }

    pub fn get_file(&self) -> &PathBuf {
        &self.log_file
    }

    pub fn get_levels(&self) -> &Vec<String> {
        &self.log_levels
    }
}
