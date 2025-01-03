use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

pub fn init(app: &AppHandle) -> () {
    let store = app.get_store("store.json");
    if let None = store {
        let mut defaults = std::collections::HashMap::new();
        defaults.insert("autoStart".to_string(), true.into());
        app.store_builder("store.json")
            .defaults(defaults)
            .build()
            .unwrap();
    }
}
