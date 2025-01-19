use tauri::{Manager, AppHandle};

#[tauri::command]
async fn close_webview_window(window_label: String, app_handle: AppHandle) {
	// 关闭webview窗口
	app_handle.get_webview_window(&window_label).unwrap().close().unwrap();
}

mod stmc;
#[tauri::command]
async fn push_to_stmc(
    title: String,
    artist: String,
    cover: String,
) {
    // 调用 update_system_media_info 函数
    stmc::update_system_media_info(&title, &artist, &cover)
		.await;
}


#[cfg(desktop)]
mod tray;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
			let _ = app.get_webview_window("main")
				.expect("no main window")
				.set_focus();
		}))
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
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
        .invoke_handler(tauri::generate_handler![close_webview_window, push_to_stmc])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
