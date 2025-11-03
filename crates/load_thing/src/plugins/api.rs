use std::os::raw::c_char;
use std::sync::{Mutex, OnceLock};

pub type OnRequestFn = unsafe extern "C" fn(ip: *const c_char, path: *const c_char);

#[derive(Clone, Copy)]
pub struct PluginApi {
    pub on_request: OnRequestFn,
}

pub static LOADED_PLUGINS: OnceLock<Mutex<Vec<PluginApi>>> = OnceLock::new();
