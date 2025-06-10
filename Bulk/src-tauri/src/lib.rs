// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn meal(calories; i32) -> String {
    format!("Good job you are {} calories, keep it up!", calories)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, meal]) // this is needed for javascript aka
                                                         // front end to get permsion to use and
                                                         // call and return
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

