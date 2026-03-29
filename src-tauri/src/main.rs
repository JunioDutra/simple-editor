mod commands {
    pub mod read_file;
    pub mod write_file;
}

use std::{path::PathBuf, sync::Mutex};

#[derive(Default)]
struct AppState {
    current_file: Mutex<Option<String>>,
}

#[tauri::command]
fn get_current_file(state: tauri::State<'_, AppState>) -> Option<String> {
    state
        .current_file
        .lock()
        .ok()
        .and_then(|value| value.clone())
}

fn resolve_initial_file() -> Option<String> {
    let args = std::env::args_os().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        return None;
    }

    let mut candidates: Vec<PathBuf> = Vec::new();
    let mut iter = args.iter().peekable();

    while let Some(arg) = iter.next() {
        let arg_text = arg.to_string_lossy();

        match arg_text.as_ref() {
            "--" => {
                if let Some(next_arg) = iter.next() {
                    candidates.push(PathBuf::from(next_arg));
                }
            }
            "--file" | "-f" => {
                if let Some(next_arg) = iter.next() {
                    candidates.push(PathBuf::from(next_arg));
                }
            }
            value if value.starts_with('-') => {}
            _ => candidates.push(PathBuf::from(arg)),
        }
    }

    let current_dir = std::env::current_dir().ok();
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(|path| path.to_path_buf());

    candidates
        .into_iter()
        .filter_map(|path| {
            if path.is_absolute() && path.is_file() {
                return Some(path);
            }

            if path.is_file() {
                return Some(path);
            }

            current_dir
                .as_ref()
                .map(|dir| dir.join(&path))
                .filter(|joined| joined.is_file())
                .or_else(|| {
                    project_root
                        .as_ref()
                        .map(|dir| dir.join(&path))
                        .filter(|joined| joined.is_file())
                })
        })
        .find_map(|path| {
            std::fs::canonicalize(path)
                .ok()
                .map(|absolute| absolute.to_string_lossy().to_string())
        })
}

fn main() {
    let initial_file = resolve_initial_file();

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
