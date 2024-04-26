use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    sync::{atomic::AtomicBool, Arc, Mutex},
};

use log::{error, info, warn};

use crate::{
    is_bool, is_in, is_int, logger::Logger, parse_bool, ssplit, str2tuple, str2vec, var2tuple,
    Global,
};

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
    pub arg_low_color: bool,

    pub locked: AtomicBool,
    pub write_lock: AtomicBool,
}

impl Config {
    fn new() -> Self {
        Self {
            // #[rustfmt::skip]
            descriptions: vec![
                str2vec!(
                    "color_theme",
                    "#* Name of a btop++/bpytop/bashtop formatted \".theme\" \
                    file, \"Default\" and \"TTY\" for builtin themes.\n\
                    #* Themes should be placed in \"../share/btop/themes\" \
                    relative to binary or \"$HOME/.config/btop/themes\""
                ),
                str2vec!(
                    "theme_background",
                    "#* If the theme set background should be shown, \
                    set to False if you want terminal background transparency."
                ),
                str2vec!(
                    "truecolor",
                    "#* Sets if 24-bit truecolor should be used, \
                        will convert 24-bit colors to 256 color (6x6x6 color cube) if false."
                ),
                str2vec!(
                    "force_tty",
                    "#* Set to true to force tty mode regardless if a real tty has been detected or not.\n\
                    #* Will force 16-color mode and TTY theme, set all graph symbols to \"tty\" and swap out other non tty friendly symbols."
                ),
                str2vec!(
                    "presets", 
                    "#* Define presets for the layout of the boxes. Preset 0 is always all boxes shown with default settings. Max 9 presets.\n\
                    #* Format: \"box_name:P:G,box_name:P:G\" P=(0 or 1) for alternate positons, G=graph symbol to use for box.\n\
                    #* Use withespace \" \" as seprator between different presets.\n\
                    #* Example: \"cpu:0:default,mem:0:tty,proc:1:default cpu:0:braille,proc:0:tty\""),
                str2vec!("rounded_corners", "#* Rounded corners on boxes, is ignored if TTY mode is ON."),
                str2vec!(
                    "graph_symbol", 
                    "#* Default symbols to use for graph creation, \"braille\", \"block\" or \"tty\".\n\
                    #* \"braille\" offers the highest resolution but might not be included in all fonts.\n\
                    #* \"block\" has half the resolution of braille but uses more common characters.\n\
                    #* \"tty\" uses only 3 different symbols but will work with most fonts and should work in a real TTY.\n\
                    #* Note that \"tty\" only has half the horizontal resolution of the other two, so will show a shorter historical view."),
                str2vec!("graph_symbol_cpu", "# Graph symbol to use for graphs in cpu box, \"default\", \"braille\", \"block\" or \"tty\"."),
                str2vec!("graph_symbol_mem", "# Graph symbol to use for graphs in cpu box, \"default\", \"braille\", \"block\" or \"tty\"."),
                str2vec!("graph_symbol_net", "# Graph symbol to use for graphs in cpu box, \"default\", \"braille\", \"block\" or \"tty\"."),
                str2vec!("graph_symbol_proc", "# Graph symbol to use for graphs in cpu box, \"default\", \"braille\", \"block\" or \"tty\"."),
                str2vec!("shown_boxes", "#* Manually set which boxes to show. Available values are \"cpu mem net proc\", separate values with whitespace."),
                str2vec!("update_ms", "#* Update time in milliseconds, recommended 2000 ms or above for better sample times for graphs."),
                str2vec!(
                    "proc_sorting", 
                    "#* Processes sorting, \"pid\" \"program\" \"arguments\" \"threads\" \
                    \"user\" \"memory\" \"cpu lazy\" \"cpu responsive\",\n\
                    #* \"cpu lazy\" sorts top process over time (easier to follow), \"cpu \
                    responsive\" updates top process directly."),
                str2vec!("proc_reversed", "#* Reverse sorting order, True or False."),
                str2vec!("proc_tree", "#* Show processes as a tree."),
                str2vec!("proc_colors", "#* Use the cpu graph colors in the process list."),
                str2vec!("proc_gradient", "#* Use a darkening gradient in the process list."),
                str2vec!("proc_per_core", "#* If process cpu usage should be of the core it's running on or usage of the total available cpu power."),
                str2vec!("proc_mem_bytes", "#* Show process memory as bytes instead of percent."),
                str2vec!("proc_info_smaps", "#* Use /proc/[pid]/smaps for memory information in the process info box (very slow but more accurate)"),
                str2vec!("proc_left", "#* Show proc box on left side of screen instead of right."),
                str2vec!(
                    "cpu_graph_upper", 
                    "#* Sets the CPU stat shown in upper half of the CPU graph, \"total\" is always available.\n\
                    #* Select from a list of detected attributes from the options menu."),
                str2vec!(
                    "cpu_graph_lower", 
                    "#* Sets the CPU stat shown in lower half of the CPU graph, \"total\" is always available.\n\
                    #* Select from a list of detected attributes from the options menu."),
                str2vec!("cpu_invert_lower", "#* Toggles if the lower CPU graph should be inverted."),
                str2vec!("cpu_single_graph", "#* Set to True to completely disable the lower CPU graph."),
                str2vec!("cpu_bottom", "#* Show cpu box at bottom of screen instead of top."),
                str2vec!("show_uptime", "#* Shows the system uptime in the CPU box."),
                str2vec!("check_temp", "#* Show cpu temperature."),
                str2vec!("cpu_sensor", "#* Which sensor to use for cpu temperature, use options menu to select from list of available sensors."),
                str2vec!("show_coretemp", "#* Show temperatures for cpu cores also if check_temp is True and sensors has been found."),
                str2vec!(
                    "cpu_core_map", 
                    "#* Set a custom mapping between core and coretemp, can be needed on certain cpus to get correct temperature for correct core.\n\
                    #* Use lm-sensors or similar to see which cores are reporting temperatures on your machine.\n\
                    #* Format \"x:y\" x=core with wrong temp, y=core with correct temp, use space as separator between multiple entries.\n\
                    #* Example: \"4:0 5:1 6:3\""),
                str2vec!("temp_scale", "#* Which temperature scale to use, available values: \"celsius\", \"fahrenheit\", \"kelvin\" and \"rankine\"."),
                str2vec!("show_cpu_freq", "#* Show CPU frequency."),
                str2vec!(
                    "clock_format", 
                    "#* Draw a clock at top of screen, formatting according to strftime, empty string to disable.\n\
                    #* Special formatting: /host = hostname | /user = username | /uptime = system uptime"),
                str2vec!("background_update", "#* Update main ui in background when menus are showing, set this to false if the menus is flickering too much for comfort."),
                str2vec!("custom_cpu_name", "#* Custom cpu model name, empty string to disable."),
                str2vec!(
                    "disks_filter", 
                    "#* Optional filter for shown disks, should be full path of a mountpoint, separate multiple values with whitespace \" \".\n\
                    #* Begin line with \"exclude=\" to change to exclude filter, otherwise defaults to \"most include\" filter. Example: disks_filter=\"exclude=/boot /home/user\"."),
                str2vec!("mem_graphs", "#* Show graphs instead of meters for memory values."),
                str2vec!("mem_below_net", "#* Show mem box below net box instead of above."),
                str2vec!("show_swap", "#* If swap memory should be shown in memory box."),
                str2vec!("swap_disk", "#* Show swap as a disk, ignores show_swap value above, inserts itself after first disk."),
                str2vec!("show_disks", "#* If mem box should be split to also show disks info."),
                str2vec!("only_physical", "#* Filter out non physical disks. Set this to False to include network disks, RAM disks and similar."),
                str2vec!("use_fstab", "#* Read disks list from /etc/fstab. This also disables only_physical."),
                str2vec!("show_io_stat", "#* Toggles if io activity % (disk busy time) should be shown in regular disk usage view."),
                str2vec!("io_mode", "#* Toggles io mode for disks, showing big graphs for disk read/write speeds."),
                str2vec!("io_graph_combined", "#* Set to True to show combined read/write io graphs in io mode."),
                str2vec!(
                    "io_graph_speeds", 
                    "#* Set the top speed for the io graphs in MiB/s (100 by default), use format \"mountpoint:speed\" separate disks with whitespace \" \".\n\
                    #* Example: \"/mnt/media:100 /:20 /boot:1\"."),
                str2vec!("net_download", "#* Set fixed values for network graphs in Mebibits. Is only used if net_auto is also set to False."),
                str2vec!("net_upload", ""),
                str2vec!("net_auto", "#* Use network graphs auto rescaling mode, ignores any values set above and rescales down to 10 Kibibytes at the lowest."),
                str2vec!("net_sync", "#* Sync the auto scaling for download and upload to whichever currently has the highest scale."),
                str2vec!("net_iface", "#* Starts with the Network Interface specified here."),
                str2vec!("show_battery", "#* Show battery stats in top right if battery is present."),
                str2vec!(
                    "log_level", 
                    "#* Set loglevel for \"~/.config/btop/error.log\" levels are: \"ERROR\" \"WARNING\" \"INFO\" \"DEBUG\".\n\
                    #* The level set includes all lower levels, i.e. \"DEBUG\" will show all logging info."),
            ],
            conf_dir: PathBuf::new(), // 默认为一个空路径
            conf_file: PathBuf::new(),

            strings: vec![
                str2tuple!("color_theme", "Default"),
                str2tuple!("shown_boxes", "cpu mem net proc"),
                str2tuple!("graph_symbol", "braille"),
                str2tuple!(
                    "presets", 
                    "cpu:1:default,proc:0:default cpu:0:default,mem:0:default,net:0:default \
                    cpu:0:block,net:0:tty"),
                str2tuple!("graph_symbol_cpu", "default"),
                str2tuple!("graph_symbol_mem", "default"),
                str2tuple!("graph_symbol_net", "default"),
                str2tuple!("graph_symbol_proc", "default"),
                str2tuple!("proc_sorting", "cpu lazy"),
                str2tuple!("cpu_graph_upper", "total"),
                str2tuple!("cpu_graph_lower", "total"),
                str2tuple!("cpu_sensor", "Auto"),
                str2tuple!("cpu_core_map", ""),
                str2tuple!("temp_scale", "celsius"),
                str2tuple!("clock_format", "%X"),
                str2tuple!("custom_cpu_name", ""),
                str2tuple!("disks_filter", ""),
                str2tuple!("io_graph_speeds", ""),
                str2tuple!("net_iface", ""),
                str2tuple!("log_level", "WARNING"),
                str2tuple!("proc_filter", ""),
                str2tuple!("proc_command", ""),
                str2tuple!("selected_name", ""),
            ].into_iter().collect(),
            strings_tmp: HashMap::new(),
            bools: vec![
                var2tuple!("theme_background", true),   var2tuple!("truecolor", true),
                var2tuple!("rounded_corners", true),    var2tuple!("proc_reversed", false),
                var2tuple!("proc_tree", false),         var2tuple!("proc_colors", true),
                var2tuple!("proc_gradient", true),      var2tuple!("proc_per_core", true),
                var2tuple!("proc_mem_bytes", true),     var2tuple!("proc_info_smaps", false),
                var2tuple!("proc_left", false),         var2tuple!("cpu_invert_lower", true),
                var2tuple!("cpu_single_graph", false),  var2tuple!("cpu_bottom", false),
                var2tuple!("show_uptime", true),        var2tuple!("check_temp", true),
                var2tuple!("show_coretemp", true),      var2tuple!("show_cpu_freq", true),
                var2tuple!("background_update", true),  var2tuple!("mem_graphs", true),
                var2tuple!("mem_below_net", false),     var2tuple!("show_swap", true),
                var2tuple!("swap_disk", true),          var2tuple!("show_disks", true),
                var2tuple!("only_physical", true),      var2tuple!("use_fstab", false),
                var2tuple!("show_io_stat", true),       var2tuple!("io_mode", false),
                var2tuple!("io_graph_combined", false), var2tuple!("net_auto", true),
                var2tuple!("net_sync", false),          var2tuple!("show_battery", true),
                var2tuple!("tty_mode", false),          var2tuple!("force_tty", false),
                var2tuple!("lowcolor", false),          var2tuple!("show_detailed", false),
                var2tuple!("proc_filtering", false),
            ].into_iter().collect(),
            bools_tmp: HashMap::new(),
            ints: vec![
                var2tuple!("update_ms", 2000),    var2tuple!("net_download", 100),     
                var2tuple!("net_upload", 100),    var2tuple!("detailed_pid", 0),  
                var2tuple!("selected_pid", 0),    var2tuple!("proc_start", 0),
                var2tuple!("proc_selected", 0), var2tuple!("proc_last_selected", 0),
            ].into_iter().collect(),
            ints_tmp: HashMap::new(),

            valid_graph_symbols: vec!["braille".to_owned(), "block".to_owned(), "tty".to_owned()],
            valid_graph_symbols_def: vec![
                "default".to_owned(),
                "braille".to_owned(),
                "block".to_owned(),
                "tty".to_owned(),
            ],
            valid_boxes: vec![
                "cpu".to_owned(),
                "mem".to_owned(),
                "net".to_owned(),
                "proc".to_owned(),
            ],
            temp_scales: vec![
                "celsius".to_owned(),
                "fahrenheit".to_owned(),
                "kelvin".to_owned(),
                "rankine".to_owned(),
            ],

            current_boxes: Vec::new(),
            preset_list: vec!["cpu:0:default,mem:0:default,net:0:default,proc:0:default".to_owned()],
            current_preset: 0, // 默认为0

            write_new: false,
            arg_low_color: false,

            locked: AtomicBool::new(false),
            write_lock: AtomicBool::new(false),
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

    pub fn get_current_boxes(&self) -> &Vec<String> {
        &self.current_boxes
    }

    pub fn get_boxes(&self, key: &str) -> String {
        match self.strings.get(key) {
            Some(value) => value.to_owned(),
            None => {
                error!("strings no [{}]", key);
                String::new()
            }
        }
    }

    pub fn get_arg_lc(&self) -> bool {
        self.arg_low_color
    }

    pub fn set_bool(&mut self, key: &str, value: bool) {
        if self.locked(key) {
            self.bools_tmp.insert(key.to_owned(), value);
        } else {
            self.bools.insert(key.to_owned(), value);
        }
    }

    pub fn get_bool(&self, key: &str) -> bool {
        match self.bools.get(key) {
            Some(value) => value.to_owned(),
            None => {
                error!("bools no [{}]", key);
                false
            }
        }
    }

    fn locked(&mut self, key: &str) -> bool {
        self.write_lock.load(std::sync::atomic::Ordering::SeqCst);
        if !self.write_new && self.descriptions.iter().find(|a| a[0] == key).is_some() {
            self.write_new = true;
        }
        return self.locked.load(std::sync::atomic::Ordering::SeqCst);
    }

    pub fn load(&mut self, load_warnings: &mut Vec<String>) -> std::io::Result<()> {
        if !self.conf_file.exists() {
            self.write_new = true;
        }

        // 需要展示的一些信息的key
        let valid_names: Vec<String> = self
            .descriptions
            .iter()
            .map(|desc| desc[0].clone())
            .collect();

        // info!("get name: {:?}", valid_names);

        let g_instance = Global::get_instance();
        let global = g_instance.lock().unwrap();

        info!("config path: {:?}", self.get_file());
        let file = File::open(&self.conf_file)?;
        let mut reader = BufReader::new(file);

        // 首先读取版本号，版本号我们设置在第一行的为止
        // 类似于:
        // ``` txt
        // #? Config file for btop-rs v. 1.0.0
        // ```
        let mut version_line = String::new();
        if reader.read_line(&mut version_line)? == 0 {
            warn!("Config file is empty");
            return Ok(());
        }

        // 判断版本号是否存储在
        if !version_line.contains(global.get_version()) {
            self.write_new = true;
            info!("Version information found");
        }

        for line in reader.lines() {
            let line = line?;
            let trim_line = line.trim();

            // println!("get line: {}", trim_line);

            if trim_line.is_empty() || trim_line.starts_with('#') {
                continue;
            }

            // 拆分配置项，配置项格式通常为：
            // ``` rust
            // key = value
            // ```
            let mut part = line.split('=');
            if let (Some(key), Some(value)) = (part.next(), part.next()) {
                let key = key.trim();
                let value = value.trim();

                // info!("get config: [{} = {}]", key, value);

                if !valid_names.contains(&key.to_owned()) {
                    continue;
                }

                if self.bools.contains_key(key) {
                    // 如果是value: bool类型的参数配置
                    // ``` rust
                    // value: [true, false, True, False]
                    // ```
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
                        info!("get config: [{} = {}]", key, value);
                    }
                } else if self.ints.contains_key(key) {
                    // 如果是value: int类型的参数配置
                    // 我们规定，对于`update_time`参数，必须有一个最小值和最大值
                    if !is_int(value) {
                        load_warnings.push(format!(
                            "Got an invalid integer value for config name: {}",
                            key
                        ));
                    } else {
                        match self.is_valid_int(key, value) {
                            Ok(v) => match self.ints.insert(key.to_owned(), v) {
                                Some(_) => warn!("get config: [{} = {}]", key, value),
                                None => todo!(),
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
                    // 对于value: String类型的配置参数
                    let value = value.trim_matches('"');

                    match self.is_valid_string(key, value) {
                        Ok(true) => match self.strings.insert(key.to_owned(), value.to_owned()) {
                            Some(_) => warn!("get config: [{} = {}]", key, value),
                            None => todo!(),
                        },
                        Ok(false) => todo!(),
                        Err(err) => match err {
                            InvalidStrReason::ParseError => load_warnings.push(format!(
                                "Got an invalid string value for config name: {}",
                                key
                            )),
                            InvalidStrReason::LogLevel => {
                                load_warnings.push(format!("Invalid log_level: {}", value))
                            }
                            InvalidStrReason::GraphSymbolIdentifier => load_warnings.push(format!(
                                "Invalid graph symbol identifier for {} : {}",
                                key, value
                            )),
                            InvalidStrReason::ShownBoxes => {
                                load_warnings.push("Invalid box name(s) in shown_boxes!".to_owned())
                            }
                            InvalidStrReason::Err(err) => match err {
                                InvalidPresetReason::TooManyPresets => {
                                    load_warnings.push("Too many presets entered!".to_owned())
                                }
                                InvalidPresetReason::TooManyBoxes => load_warnings
                                    .push("Too many boxes entered for preset!".to_owned()),
                                InvalidPresetReason::MalformattedError => load_warnings.push(
                                    "Malformatted preset in config value presets!".to_owned(),
                                ),
                                InvalidPresetReason::InvalidBoxName => load_warnings
                                    .push("Invalid box name in config value presets!".to_owned()),
                                InvalidPresetReason::InvalidPositionValue => load_warnings.push(
                                    "Invalid position value in config value presets!".to_owned(),
                                ),
                                InvalidPresetReason::InvalidGraphName => load_warnings
                                    .push("Invalid graph name in config value presets!".to_owned()),
                            },
                            InvalidStrReason::PresetsError => todo!(),
                            InvalidStrReason::CpuCoreMapError => {
                                load_warnings.push("Invalid formatting of cpu_core_map!".to_owned())
                            }
                            InvalidStrReason::IOGraphSpeedError => load_warnings
                                .push("Invalid formatting of io_graph_speeds!".to_owned()),
                        },
                    }
                }
            }
        } // end for

        if !load_warnings.is_empty() {
            self.write_new = true;
        }

        Ok(())
    }
}

pub enum InvalidIntReason {
    ValueTooHigh,
    ValueTooLow,
    ParseError,
}

pub enum InvalidStrReason {
    ParseError,
    LogLevel,
    GraphSymbolIdentifier,
    ShownBoxes,
    PresetsError,
    Err(InvalidPresetReason),
    CpuCoreMapError,
    IOGraphSpeedError,
}

pub enum InvalidPresetReason {
    TooManyPresets,
    TooManyBoxes,
    MalformattedError,
    InvalidBoxName,
    InvalidPositionValue,
    InvalidGraphName,
}

impl Config {
    fn is_valid_int(&self, key: &str, value: &str) -> Result<i32, InvalidIntReason> {
        let parsed_value = match key {
            "update_ms" => match value.parse::<i32>() {
                Ok(parsed) if parsed < 100 => Err(InvalidIntReason::ValueTooLow),
                Ok(parsed) if parsed > 86400000 => Err(InvalidIntReason::ValueTooHigh),
                Ok(parsed) => Ok(parsed),
                _ => Err(InvalidIntReason::ParseError),
            },
            _ => match value.parse::<i32>() {
                Ok(parsed) => Ok(parsed),
                _ => Err(InvalidIntReason::ParseError),
            },
        };

        // match parsed_value {
        //     Ok(parsed) => Ok(parsed),
        //     Err(err) => Err(err),
        // }
        parsed_value
    }

    fn is_valid_string(&mut self, key: &str, value: &str) -> Result<bool, InvalidStrReason> {
        let l_instance = Logger::get_instance();
        let logger = l_instance.lock().unwrap();

        match key {
            // ``` rust
            // log_level: ["DISABLED", "ERROR", "WARNING", "INFO", "DEBUG"]
            // ```
            "log_level" => match logger.get_levels().contains(&value.to_owned()) {
                true => Ok(true),
                false => Err(InvalidStrReason::LogLevel),
            },
            // ``` rust
            // graph_symbol: ["braille", "block", "tty"]
            // ```
            "graph_symbol" => match self.valid_graph_symbols.contains(&value.to_owned()) {
                true => Ok(true),
                false => Err(InvalidStrReason::GraphSymbolIdentifier),
            },
            // ``` rust
            // graph_symbol_: ["graph_symbol_cpu", "graph_symbol_gpu", "graph_symbol_mem", "graph_symbol_net", "graph_symbol_proc"]
            // ```
            "graph_symbol_" if key.starts_with("graph_symbol_") && value.ne("default") => {
                match self.valid_graph_symbols.contains(&value.to_owned()) {
                    true => Ok(true),
                    false => Err(InvalidStrReason::GraphSymbolIdentifier),
                }
            }
            // ``` rust
            // shown_boxes: ssplit("cpu mem net proc", ' ');
            // shown_boxes: ["cpu", "mem", "net", "proc"]
            // ```
            "shown_boxes" if !value.is_empty() => match self.check_boxes(value) {
                true => Ok(true),
                false => Err(InvalidStrReason::ShownBoxes),
            },
            // ``` rust
            // presets: "cpu:0:default,mem:0:default,net:0:default,proc:0:default"
            // presets: ["cpu:0:default", "mem:0:default", "net:0:default", "proc:0:default"]
            // presets: [["cpu", "0", "default"], ["mem", "0", "default"], ["net", "0", "default"], ["proc", "0", "default"]]
            // ```
            "presets" => match self.is_valid_presets(value) {
                Ok(true) => Ok(true),
                Ok(false) => Err(InvalidStrReason::PresetsError),
                Err(_) => todo!(),
            },
            // ``` rust
            // cpu_core_map: ["x:y"]
            // ```
            "cpu_core_map" => {
                let maps = ssplit(value, ' ');
                let mut all_good = true;

                for map in maps {
                    let map_split = ssplit(map, ':');
                    if map_split.len() != 2 {
                        all_good = false;
                    } else if !is_int(map_split[0]) || !is_int(map_split[1]) {
                        all_good = false;
                    }

                    if !all_good {
                        return Err(InvalidStrReason::CpuCoreMapError);
                    }
                }
                Ok(true)
            }
            // ``` rust
            // io_graph_speeds: ["mountpoint: speed"]
            // ```
            "io_graph_speeds" => {
                let maps = ssplit(value, ' ');
                let mut all_good = true;

                for map in maps {
                    let map_split = ssplit(map, ':');
                    if map_split.len() != 2 {
                        all_good = false;
                    } else if map_split[0].is_empty() || !is_int(map_split[1]) {
                        all_good = false;
                    }

                    if !all_good {
                        return Err(InvalidStrReason::IOGraphSpeedError);
                    }
                }

                Ok(true)
            }
            _ => Err(InvalidStrReason::ParseError),
        }
    }

    fn is_valid_presets(&mut self, value: &str) -> Result<bool, InvalidPresetReason> {
        let presets = ssplit(value, ' ');
        let mut new_presets = presets.clone();

        if presets.len() > 9 {
            return Err(InvalidPresetReason::TooManyPresets);
        }

        for preset in presets {
            let boxes = ssplit(preset, ',');
            if boxes.len() > 4 {
                return Err(InvalidPresetReason::TooManyPresets);
            }

            for b in boxes {
                let vals = ssplit(b, ':');
                if vals.len() != 3 {
                    return Err(InvalidPresetReason::MalformattedError);
                }

                if !is_in(&vals[0], &["cpu", "mem", "net", "proc"]) {
                    return Err(InvalidPresetReason::InvalidBoxName);
                }

                if !is_in(&vals[1], &["0", "1"]) {
                    return Err(InvalidPresetReason::InvalidPositionValue);
                }

                if !self.valid_graph_symbols_def.contains(&vals[2].to_owned()) {
                    return Err(InvalidPresetReason::InvalidGraphName);
                }
                warn!("get config boxes: {:?}", vals);
            }
            new_presets.push(preset);
        }

        self.preset_list = new_presets.iter().map(|&s| s.to_owned()).collect();

        Ok(true)
    }

    pub fn check_boxes(&mut self, value: &str) -> bool {
        let boxes = ssplit(value, ' ');
        let t_boxes = boxes.clone();

        for b in boxes {
            if !self.valid_boxes.contains(&b.to_owned()) {
                return false;
            }
        }

        let boxes: Vec<String> = t_boxes.iter().map(|&s| s.to_string()).collect();
        warn!("get config boxes: {:?}", boxes);
        self.set_current_boxes(boxes.clone());
        true
    }

    fn set_current_boxes(&mut self, boxes: Vec<String>) {
        self.current_boxes = boxes.clone();
    }
}
