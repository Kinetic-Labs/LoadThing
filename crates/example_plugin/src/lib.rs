/// Example Plugin
///
/// An example plugin to help you get started
/// developing plugins for LoadThing
use std::collections::HashMap;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::{Mutex, OnceLock};

// This is the data structure that will store the request counts.
// It's a HashMap where the key is the IP address (String) and the value is another
// HashMap where the key is the requested path (String) and the value is the count (u64).
// We use a Mutex to ensure thread-safe access from multiple threads.
type IpLog = Mutex<HashMap<String, HashMap<String, u64>>>;

// A global, thread-safe, lazily-initialized static variable to hold our IP log.
static IP_LOGGER: OnceLock<IpLog> = OnceLock::new();

// This function is called by the host application when the plugin is loaded.
#[unsafe(no_mangle)]
pub extern "C" fn load_thing_init() {
    // Initialize the global logger.
    IP_LOGGER.get_or_init(|| Mutex::new(HashMap::new()));
    println!("[ip_logger_plugin] Initialized.");
}

// This function will be called by LoadThing on each request.
#[unsafe(no_mangle)]
pub extern "C" fn on_request(ip: *const c_char, path: *const c_char) {
    let logger = match IP_LOGGER.get() {
        Some(logger) => logger,
        None => {
            eprintln!("[ip_logger_plugin] Error: Logger not initialized.");
            return;
        }
    };

    let ip_str = unsafe { CStr::from_ptr(ip) }.to_string_lossy().into_owned();
    let path_str = unsafe { CStr::from_ptr(path) }
        .to_string_lossy()
        .into_owned();

    let count = {
        let log = logger.lock();
        if let Ok(mut log) = log {
            let path_map = log.entry(ip_str).or_default();
            let count = path_map.entry(path_str).or_default();
            *count += 1;
            *count
        } else {
            return;
        }
    };

    let log = logger.lock();
    if let Ok(log) = log {
        let ip_lookup = unsafe { CStr::from_ptr(ip) }.to_string_lossy();
        let path_lookup = unsafe { CStr::from_ptr(path) }.to_string_lossy();
        let ip_key = log
            .get_key_value(ip_lookup.as_ref())
            .map(|(k, _)| k)
            .unwrap();
        let path_map = log.get(ip_key).unwrap();
        let path_key = path_map
            .get_key_value(path_lookup.as_ref())
            .map(|(k, _)| k)
            .unwrap();

        // Recommended: name your plugin in logs
        println!(
            "[example_plugin] Request logged: IP = {}, Path = {}, Count = {}",
            ip_key, path_key, count
        );
    }
}
