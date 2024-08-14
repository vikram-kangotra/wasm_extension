#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

wai_bindgen_rust::export!("../extension.wai");

#[allow(dead_code)]
mod vlc {

    pub mod msg {

        #[link(wasm_import_module = "vlc_msg")]
        extern "C" {
            fn msg_dbg(message: *const u8, length: usize);
        }

        pub fn dbg(msg: &str) {
            unsafe {
                msg_dbg(msg.as_ptr(), msg.len());
            }
        }
    }

    pub mod var {

        #[link(wasm_import_module = "vlc_var")]
        extern "C" {
            fn var_trigger_callback(var_name: *const u8, length: usize);
        }

        pub fn trigger_callback(name: &str) {
            unsafe {
                var_trigger_callback(name.as_ptr(), name.len());
            }
        }
    }

    pub mod playlist {

        #[link(wasm_import_module = "vlc_playlist")]
        extern "C" {
            fn playlist_play();
            fn playlist_status() -> u32;
        }

        pub fn play() {
            unsafe {
                playlist_play();
            }
        }

        pub fn status() -> u32 {
            unsafe {
                playlist_status()
            }
        }
    }

    pub mod config {
        use std::{ffi::CString, path::{Path, PathBuf}};

        #[link(wasm_import_module = "vlc_config")]
        extern "C" {
            fn config_gettype(name: *const u8, length: usize) -> i32;
            fn config_getstring(name: *const u8, length: usize) -> *const i8;
            fn config_getint(name: *const u8, length: usize) -> i32;
            fn config_getfloat(name: *const u8, length: usize) -> f32;
            fn config_putstring(name: *const u8, length: usize, value: *const u8, value_length: usize);
            fn config_putint(name: *const u8, length: usize, value: i32);
            fn config_putfloat(name: *const u8, length: usize, value: f32);
            fn config_datadir() -> *const i8;
            fn config_userdatadir() -> *const i8;
            fn config_homedir() -> *const i8;
            fn config_configdir() -> *const i8;
            fn config_cachedir() -> *const i8;
            fn config_datadir_list(ptr: *const u8, length: usize) -> *const i8;
        }

        pub fn get_type(name: &str) -> i32 {
            let ptr = name.as_ptr();
            let len = name.len();
            unsafe {
                config_gettype(ptr, len)
            }
        }

        pub fn get_string(name: &str) -> String {
            let ptr = name.as_ptr();
            let len = name.len();
            unsafe {
                let ptr = config_getstring(ptr, len);
                CString::from_raw(ptr as *mut i8).to_string_lossy().to_string()
            }
        }

        pub fn get_int(name: &str) -> i32 {
            let ptr = name.as_ptr();
            let len = name.len();
            unsafe {
                config_getint(ptr, len)
            }
        }

        pub fn get_float(name: &str) -> f32 {
            let ptr = name.as_ptr();
            let len = name.len();
            unsafe {
                config_getfloat(ptr, len)
            }
        }

        pub fn get_bool(name: &str) -> bool {
            get_int(name) != 0
        }

        pub fn put_string(name: &str, value: &str) {
            let name_ptr = name.as_ptr();
            let name_len = name.len();
            let value_ptr = value.as_ptr();
            let value_len = value.len();
            unsafe {
                config_putstring(name_ptr, name_len, value_ptr, value_len);
            }
        }

        pub fn put_int(name: &str, value: i32) {
            let name_ptr = name.as_ptr();
            let name_len = name.len();
            unsafe {
                config_putint(name_ptr, name_len, value);
            }
        }

        pub fn put_float(name: &str, value: f32) {
            let name_ptr = name.as_ptr();
            let name_len = name.len();
            unsafe {
                config_putfloat(name_ptr, name_len, value);
            }
        }

        pub fn put_bool(name: &str, value: bool) {
            put_int(name, if value { 1 } else { 0 });
        }

        pub fn datadir() -> String {
            unsafe {
                let ptr = config_datadir();
                CString::from_raw(ptr as *mut i8).to_string_lossy().to_string()
            }
        }

        pub fn userdatadir() -> String {
            unsafe {
                let ptr = config_userdatadir();
                CString::from_raw(ptr as *mut i8).to_string_lossy().to_string()
            }
        }

        pub fn homedir() -> String {
            unsafe {
                let ptr = config_homedir();
                CString::from_raw(ptr as *mut i8).to_string_lossy().to_string()
            }
        }

        pub fn configdir() -> String {
            unsafe {
                let ptr = config_configdir();
                CString::from_raw(ptr as *mut i8).to_string_lossy().to_string()
            }
        }

        pub fn cachedir() -> String {
            unsafe {
                let ptr = config_cachedir();
                CString::from_raw(ptr as *mut i8).to_string_lossy().to_string()
            }
        }

        pub fn datadir_list(path: &Path) -> Vec<PathBuf> {
            let path_str = path.to_str().unwrap();
            let path_ptr = path_str.as_ptr();
            let path_len = path_str.len();
            let mut path_bufs = Vec::new();

            unsafe {
                let mut current_ptr = config_datadir_list(path_ptr, path_len);

                loop {
                    let str_ptr = u32::from_le_bytes(current_ptr.cast::<[u8; 4]>().read());

                    if str_ptr == 0 {
                        break;
                    }

                    let cstr = CString::from_raw(str_ptr as *mut i8);
                    let path = cstr.to_string_lossy().to_string();
                    path_bufs.push(PathBuf::from(path));

                    current_ptr = current_ptr.add(4);
                }
            }

            path_bufs
        }
    }
}

struct Extension;

impl extension::Extension for Extension {

    fn allocate_memory(len: u32) -> u32 {
       let data = vec![0u8; len as usize]; 
       let ptr = data.as_ptr();
       std::mem::forget(data);
       ptr as u32
    }

    fn descriptor() -> extension::Description {
        extension::Description {
            title: "test".to_string(),
            version: "0.0.1".to_string(),
            author: "VideoLAN".to_string(),
            shortdesc: "Test example".to_string(),
            description: "Test description".to_string(),
        }
    }

    fn activate() {
        vlc::msg::dbg("Activated");
        vlc::msg::dbg(&format!("Data dir: {}", vlc::config::datadir()));
        vlc::msg::dbg(&format!("User data dir: {}", vlc::config::userdatadir()));
        vlc::msg::dbg(&format!("Home dir: {}", vlc::config::homedir()));
        vlc::msg::dbg(&format!("Config dir: {}", vlc::config::configdir()));
        vlc::msg::dbg(&format!("Cache dir: {}", vlc::config::cachedir()));

        let paths = vlc::config::datadir_list(std::path::Path::new("extensions"));
        vlc::msg::dbg(&format!("Data dir list: {:?}", paths));

        /*
        vlc::config::put_int("vlcwasm_my-int", 42);
        vlc::msg::dbg(&format!("Test int: {}", vlc::config::get_int("vlcwasm_my-int")));

        vlc::config::put_string("vlcwasm_my-string", "Hello from WASM");
        vlc::msg::dbg(&format!("Test string: {}", vlc::config::get_string("vlcwasm_my-string")));

        vlc::config::put_float("vlcwasm_my-float", 3.14);
        vlc::msg::dbg(&format!("Test float: {}", vlc::config::get_float("vlcwasm_my-float")));

        vlc::config::put_bool("vlcwasm_my-bool", true);
        vlc::msg::dbg(&format!("Test bool: {}", vlc::config::get_bool("vlcwasm_my-bool")));
        */

        vlc::msg::dbg(&format!("Playlist status: {}", vlc::playlist::status()));

        // vlc::var::trigger_callback("test-wasm-activate");
    }

    fn deactivate() {
        vlc::msg::dbg("Deactivated");
        // vlc::var::trigger_callback("test-wasm-deactivate");
    }
}
