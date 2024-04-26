use std::{
    path::PathBuf,
    sync::{atomic::AtomicBool, Arc, Mutex},
};

pub struct Global {
    pub banner_src: Vec<[String; 2]>,
    pub start_time: u64,
    pub version: String,
    pub counter: u32,
    pub self_path: PathBuf,
    pub arg_low_color: bool,
    pub arg_tty: bool,
    pub arg_preset: i32,
    pub quitting: AtomicBool,
    pub resized: AtomicBool,
}

impl Global {
    pub fn get_instance() -> Arc<Mutex<Global>> {
        static mut instance: Option<Arc<Mutex<Global>>> = None;
        unsafe {
            instance
                .get_or_insert_with(|| Arc::new(Mutex::new(Global::new(0, "1.0.0"))))
                .clone()
        }
    }

    fn new(start_time: u64, version: &str) -> Self {
        Global {
            #[rustfmt::skip] 
            banner_src: vec![
              ["#E62525".to_owned(), "██████╗ ████████╗ ██████╗ ██████╗          ███████╗   ███████╗    ██╗".to_owned()],
              ["#CD2121".to_owned(), "██╔══██╗╚══██╔══╝██╔═══██╗██╔══██╗         ██╔═══██╗ ██╔═════╝  ██████╗ ".to_owned()],
              ["#B31D1D".to_owned(), "██████╔╝   ██║   ██║   ██║██████╔╝ ██████╗ ███████╔╝  ███████═╗ ╚═██╔═╝".to_owned()],
              ["#9A1919".to_owned(), "██╔══██╗   ██║   ██║   ██║██╔═══╝  ╚═════╝ ██╔═══██╗        ██║   ╚═╝".to_owned()],
              ["#000000".to_owned(), "██████╔╝   ██║   ╚██████╔╝██║              ██║   ██║  ███████╔╝ ".to_owned()],
            ],
            start_time,
            version: version.to_owned(),
            counter: 0,
            self_path: PathBuf::new(),
            arg_tty: false,
            arg_low_color: false,
            arg_preset: -1,
            quitting: AtomicBool::new(false),
            resized: AtomicBool::new(false),
        }
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    pub fn get_count(&self) -> u32 {
        self.counter
    }

    pub fn increment(&mut self) {
        self.counter += 1
    }

    pub fn get_time(&self) -> u64 {
        self.start_time
    }

    pub fn set_time(&mut self, time: u64) {
        self.start_time = time;
    }

    pub fn set_arglc(&mut self) {
        self.arg_low_color = true;
    }

    pub fn get_quit_state(&self) -> bool {
        self.quitting.load(std::sync::atomic::Ordering::Acquire)
    }

    pub fn set_quit_state(&self) {
        self.quitting
            .store(true, std::sync::atomic::Ordering::Release);
    }

    pub fn set_self(&mut self, self_path: PathBuf) {
        self.self_path = self_path;
    }

    pub fn get_self(&self) -> &PathBuf {
        &self.self_path
    }

    pub fn get_arg_lc(&self) -> bool {
        self.arg_low_color
    }
}
