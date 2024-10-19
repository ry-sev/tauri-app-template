#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default();

    #[cfg(target_os = "macos")]
    #[cfg(not(debug_assertions))]
    let app = app.on_window_event(|event| {
        use tauri::Manager;
        if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
            if event.window().label() == "main" {
                tauri::AppHandle::hide(&event.window().app_handle())
                    .expect("Window should hide on macOS");
                api.prevent_close();
            }
        }
    });

    let app = app
        .setup(move |app| {
            let _window = tauri::WebviewWindowBuilder::new(
                app,
                "main",
                tauri::WebviewUrl::App("index.html".into()),
            )
            .title("tauri-app")
            .inner_size(1920.0, 1080.0)
            .min_inner_size(800.0, 600.0)
            .center()
            .build()?;

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|_app_handle, _event| {});
}
