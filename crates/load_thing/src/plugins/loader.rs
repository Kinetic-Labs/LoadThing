use crate::plugins::api::{OnRequestFn, PluginApi, LOADED_PLUGINS};
use libloading::Library;
use std::fs;
use std::sync::Mutex;

const PLUGINS_DIR: &str = "plugins";

pub fn load_plugins() {
    LOADED_PLUGINS.get_or_init(|| Mutex::new(Vec::new()));

    if fs::create_dir_all(PLUGINS_DIR).is_err() {
        eprintln!("Failed to create plugins directory");
        return;
    }

    for entry in fs::read_dir(PLUGINS_DIR).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if let Some(ext) = path.extension() {
            if ext == "so" || ext == "dylib" || ext == "dll" {
                unsafe {
                    match Library::new(&path) {
                        Ok(lib) => {
                            match lib.get::<unsafe extern "C" fn()>(b"load_thing_init") {
                                Ok(init) => {
                                    init();
                                }
                                Err(e) => {
                                    eprintln!("[loader] Error getting init function for {:?}: {}", &path, e);
                                    continue;
                                }
                            }

                            match lib.get::<OnRequestFn>(b"on_request") {
                                Ok(on_request) => {
                                    let api = PluginApi {
                                        on_request: *on_request,
                                    };
                                    if let Some(plugins) = LOADED_PLUGINS.get() {
                                        plugins.lock().unwrap().push(api);
                                    }
                                }
                                Err(e) => {
                                    eprintln!("[plugin_loader] Error getting on_request function for {:?}: {}", &path, e);
                                }
                            }
                            std::mem::forget(lib);
                        }
                        Err(e) => {
                            eprintln!("[plugin_loader] Error loading library {:?}: {}", &path, e);
                        }
                    }
                }
            }
        }
    }
}
