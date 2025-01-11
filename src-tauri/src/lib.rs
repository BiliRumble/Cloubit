use tauri::{Manager, AppHandle};

#[tauri::command]
async fn close_webview_window(window_label: String, app_handle: AppHandle) {
    if let Some(window) = app_handle.get_webview_window(&window_label) {
        window.close().unwrap();
    } else {
        println!("Window with label '{}' not found", window_label);
    }
}

#[cfg(desktop)]
mod tray;

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(all(desktop))]
            {
                let handle = app.handle();
                tray::create_tray(handle)?;
            }
            Ok(())
        })
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![close_webview_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
