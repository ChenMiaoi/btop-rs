use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::{is_bool, is_int, is_valid_int, parse_bool, Global, InvalidIntReason};

pub struct Config {
    descriptions: Vec<[String; 2]>,

    pub conf_dir: PathBuf,
    pub conf_file: PathBuf,

    pub strings: HashMap<String, String>,
    pub strings_tmp: HashMap<String, String>,
    pub bools: HashMap<String, bool>,
    pub bools_tmp: HashMap<String, bool>,
    pub ints: HashMap<String, i32>,
    pub ints_tmp: HashMap<String, i32>,

    pub valid_graph_symbols: Vec<String>,
    pub valid_graph_symbols_def: Vec<String>,
    pub valid_boxes: Vec<String>,
    pub temp_scales: Vec<String>,

    pub current_boxes: Vec<String>,
    pub preset_list: Vec<String>,
    pub current_preset: i32,

    pub write_new: bool,
}

impl Config {
    fn new() -> Self {
        Self {
            #[rustfmt::skip]
            descriptions: vec![
                ["color_theme".to_owned(),      "#* Name of a btop-rs++/bpytop/bashtop formatted \".theme\" file, \"Default\" and \"TTY\" for builtin themes.\n\
                                                #* Themes should be placed in \"../share/btop-rs/themes\" relative to binary or \"$HOME/.config/btop-rs/themes\"".to_owned()],
                ["theme_background".to_owned(), "#* If the theme set background should be shown, set to False if you want terminal background transparency.".to_owned()],
            ],
            conf_dir: PathBuf::new(), // 默认为一个空路径
            conf_file: PathBuf::new(),

            strings: HashMap::new(),
            strings_tmp: HashMap::new(),
            bools: HashMap::new(),
            bools_tmp: HashMap::new(),
            ints: HashMap::new(),
            ints_tmp: HashMap::new(),

            valid_graph_symbols: Vec::new(),
            valid_graph_symbols_def: Vec::new(),
            valid_boxes: Vec::new(),
            temp_scales: Vec::new(),

            current_boxes: Vec::new(),
            preset_list: Vec::new(),
            current_preset: 0, // 默认为0

            write_new: false,
        }
    }

    pub fn get_instance() -> Arc<Mutex<Config>> {
        static mut instance: Option<Arc<Mutex<Config>>> = None;
        unsafe {
            instance
                .get_or_insert_with(|| Arc::new(Mutex::new(Config::new())))
                .clone()
        }
    }

    pub fn set_dir(&mut self, dir_path: PathBuf) {
        self.conf_dir = dir_path;
    }

    pub fn get_dir(&self) -> &PathBuf {
        &self.conf_dir
    }

    pub fn set_file(&mut self, file_name: &str) {
        self.conf_file = self.conf_dir.join(file_name);
    }

    pub fn get_file(&self) -> &PathBuf {
        &self.conf_file
    }

    pub fn load(&mut self, load_warnings: &mut Vec<String>) -> std::io::Result<()> {
        if !self.conf_file.exists() {
            self.write_new = true;
        }

        let mut valid_names: Vec<String> = self
            .descriptions
            .iter()
            .map(|desc| desc[0].clone())
            .collect();

        println!("get name: {:?}", valid_names);

        let g_instance = Global::get_instance();
        let global = g_instance.lock().unwrap();

        let file = File::open(&self.conf_file)?;
        let mut reader = BufReader::new(file);

        let mut version_line = String::new();
        if reader.read_line(&mut version_line)? == 0 {
            println!("Config file is empty");
            return Ok(());
        }

        if !version_line.contains(global.get_version()) {
            self.write_new = true;
            println!("Version information not found");
        }

        for line in reader.lines() {
            let line = line?;
            let trim_line = line.trim();

            if trim_line.is_empty() || trim_line.starts_with('#') {
                continue;
            }

            let mut part = line.split('=');
            if let (Some(key), Some(value)) = (part.next(), part.next()) {
                let key = key.trim();
                let value = value.trim();

                if !valid_names.contains(&key.to_owned()) {
                    continue;
                }

                if self.bools.contains_key(key) {
                    if !is_bool(value) {
                        load_warnings.push(format!(
                            "Got an invalid bool value for config name: {}",
                            key
                        ));
                    } else {
                        match parse_bool(value) {
                            Some(v) => self.bools.insert(key.to_owned(), v),
                            None => panic!("can't parse str to bool"),
                        };
                    }
                } else if self.ints.contains_key(key) {
                    if !is_int(value) {
                        load_warnings.push(format!(
                            "Got an invalid integer value for config name: {}",
                            key
                        ));
                    } else {
                        match is_valid_int(key, value) {
                            Ok(v) => match self.ints.insert(key.to_owned(), v) {
                                _ => {}
                            },
                            Err(err) => match err {
                                InvalidIntReason::ValueTooHigh => load_warnings.push(
                                    "Config value update_ms set too high (>86400000).".to_owned(),
                                ),
                                InvalidIntReason::ValueTooLow => load_warnings
                                    .push("Config value update_ms set too low (<100).".to_owned()),
                                InvalidIntReason::ParseError => {
                                    load_warnings.push("Invalid numerical value!".to_owned())
                                }
                            },
                        };
                    }
                } else if self.strings.contains_key(key) {
                    // TODO
                }
            }
        } // end for

        if !load_warnings.is_empty() {
            self.write_new = true;
        }

        Ok(())
    }
}
