use std::{
    env::{self},
    ffi::c_int,
    fs,
    path::{Path, PathBuf},
    process::exit,
    sync::{Arc, Mutex},
};

use config::config::Config;
use libc::{SIGCONT, SIGINT, SIGTSTP, SIGWINCH};
use shared::global::*;
use util::*;

use crate::{config::theme::Theme, logger::Logger};

pub mod config;
pub mod include;
pub mod shared;
pub mod util;

fn argument_parser(args: Vec<String>) {
    let instance = Global::get_instance();
    for arg in args.iter().skip(1) {
        if is_in(arg, &["-h".to_owned(), "--help".to_owned()]) {
            println!(
          "usage: btop [-h] [-v] [-/+t] [--utf-foce] [--debug]\n\n\
          optional arguments:\n\
          \t-h, --help            show this help message and exit\n\
          \t-v, --version         show version info and exit\n\
          \t-lc, --low-color      disable truecolor, converts 24-bit colors to 256-color\n\
          \t-t, --tty_on          force (ON) tty mode, max 16 colors and tty friendly graph symbols\n\
          \t+t, --tty_off         force (OFF) tty mode\n\
          \t-p --preset <id>      start with preset, integer value between 0-9\n\
          \t--utf-foce            force start even if no UTF-8 locale was detected\n\
          \t--debug               start in DEBUG mode: shows microsecond timer for information collect\n\
          \t                      and screen draw functions and sets loglevel to DEBUG\n
          "
        );
            exit(0);
        } else if is_in(arg, &["-v".to_owned(), "--version".to_owned()]) {
            {
                let v_instance = instance.lock().unwrap();
                println!("btop-rs version: {}", v_instance.get_version());
            }
        } else if is_in(arg, &["-lc".to_owned(), "--low-color".to_owned()]) {
            {
                let mut v_instance = instance.lock().unwrap();
                v_instance.set_arglc();
            }
        }
        // TODO
    }
}

extern "C" fn _exit_handler() {
    clean_quit(-1);
}

fn clean_quit(sig: i32) {
    let instance = Global::get_instance();
    let g_instance = instance.lock().unwrap();

    if g_instance.get_quit_state() {
        return;
    }
    g_instance.set_quit_state();
    // TODO
    exit(sig);
}

fn _sleep() {}

fn _resume() {}

fn term_resize() {}

extern "C" fn signal_handler(signal: c_int) {
    match signal {
        SIGINT => {
            println!("SIGNAL SIGINT");
            clean_quit(0);
        }
        SIGTSTP => {
            _sleep();
            println!("SIGNAL SIGTSTP");
        }
        SIGCONT => _resume(),
        SIGWINCH => term_resize(),
        _ => {}
    }
}

fn main() {
    let g_instance: Arc<Mutex<Global>> = Global::get_instance();
    let c_instance: Arc<Mutex<Config>> = Config::get_instance();
    let t_instance: Arc<Mutex<Theme>> = Theme::get_instance();
    let l_instance: Arc<Mutex<Logger>> = Logger::get_instance();

    // let mut global = g_instance.lock().unwrap();
    // let mut config = c_instance.lock().unwrap();
    // let mut logger = l_instance.lock().unwrap();
    // let mut theme = t_instance.lock().unwrap();

    {
        let mut global = g_instance.lock().unwrap();
        global.set_time(time_s());
    }

    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        argument_parser(args);
    }

    unsafe {
        libc::atexit(_exit_handler);

        libc::signal(SIGINT, signal_handler as usize);
        libc::signal(SIGTSTP, signal_handler as usize);
        libc::signal(SIGCONT, signal_handler as usize);
        libc::signal(SIGWINCH, signal_handler as usize);
    }

    // 设置启动配置文件、日志和主题路径
    for env in ["XDG_CONFIG_HOME", "HOME"] {
        if let Ok(env_val) = env::var(env) {
            if let Ok(mut perms) = fs::metadata(&env_val).map(|md| md.permissions()) {
                perms.set_readonly(false);
                if !perms.readonly() {
                    let dir = if env == "HOME" {
                        Path::new(&env_val).join(".config/btop-rs")
                    } else {
                        Path::new(&env_val).join("btop-rs")
                    };
                    {
                        let mut config = c_instance.lock().unwrap();
                        config.set_dir(dir);
                        println!("set config file dir path: {:?}", config.get_dir());
                    }
                    break;
                }
            }
        }
    }

    {
        let mut config = c_instance.lock().unwrap();
        let mut logger = l_instance.lock().unwrap();
        let mut theme = t_instance.lock().unwrap();
        if config.get_dir().as_os_str().is_empty() {
            println!("WARNING: Could not get path user HOME folder.");
            println!("Make sure $XDG_CONFIG_HOME or $HOME environment variables is correctly set to fix this.");
        } else {
            if !config.get_dir().is_dir() && !fs::create_dir(config.get_dir()).is_ok() {
                println!("WARNING: Could not create or access btop config directory. Logging and config saving disabled.");
                println!("Make sure $XDG_CONFIG_HOME or $HOME environment variables is correctly set to fix this.");
            } else {
                let config_dir = config.get_dir().clone();
                config.set_file("btop-rs.conf");
                logger.set_file(config_dir.join("btop-rs.log"));
                theme.set_user_dir(config_dir.join("themes"));

                if !theme.get_user_dir().exists() && !fs::create_dir(theme.get_user_dir()).is_ok() {
                    theme.clear_user_dir();
                }

                println!("set config path: {:?}", config.get_file());
            }
        }
    }

    let mut self_path = env::current_exe().expect("Failed to get current executable path");
    self_path.pop();
    {
        let mut global = g_instance.lock().unwrap();
        global.set_self(self_path);
        println!("get the execute path: {:?}", global.get_self());
    }

    {
        let global = g_instance.lock().unwrap();
        let mut theme = t_instance.lock().unwrap();

        if !global.get_self().as_os_str().is_empty() {
            match fs::canonicalize(global.get_self().join("../share/btop-rs/themes")) {
                Ok(canon_path) => {
                    println!("canno: {:?}", canon_path);
                    theme.set_theme_dir(canon_path);
                }
                Err(err) => {
                    println!("Failed to get canonical path: {}", err);
                }
            };
            if let Ok(perms) = fs::metadata(theme.get_theme_dir()) {
                if !perms.is_dir() || perms.permissions().readonly() {
                    theme.clear_theme_dir();
                }
            };
        }
    }

    {
        let mut theme = t_instance.lock().unwrap();

        if theme.get_theme_dir().as_os_str().is_empty() {
            for theme_path in [
                "/usr/local/share/btop-rs/themes",
                "/usr/share/btop-rs/themes",
            ] {
                if let Ok(perms) = fs::metadata(theme_path) {
                    if perms.is_dir() && !perms.permissions().readonly() {
                        theme.set_theme_dir(PathBuf::from(theme_path));
                        break;
                    }
                }
            }
        }

        println!("user theme dir: {:?}", theme.get_user_dir());
        println!("theme dir: {:?}", theme.get_theme_dir());
    }

    let mut load_warnings: Vec<String> = Vec::new();
    {
        let mut config = c_instance.lock().unwrap();
        match config.load(&mut load_warnings) {
            Ok(_) => {}
            Err(_) => {}
        }
    }
    // while true {}
}
