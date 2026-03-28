mod commands {
    pub mod read_file;
    pub mod write_file;
}

use std::sync::Mutex;

#[derive(Default)]
struct AppState {
    current_file: Mutex<Option<String>>,
}

#[tauri::command]
fn get_current_file(state: tauri::State<'_, AppState>) -> Option<String> {
    state.current_file.lock().ok().and_then(|value| value.clone())
}

fn main() {
    let initial_file = std::env::args().nth(1);

    tauri::Builder::default()
        .manage(AppState {
            current_file: Mutex::new(initial_file),
        })
        .invoke_handler(tauri::generate_handler![
            get_current_file,
            commands::read_file::read_file,
            commands::write_file::write_file,
        ])
        .run(tauri::generate_context!())
        .expect("failed to run simple-editor tauri backend");
}
