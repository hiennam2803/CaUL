use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, WebviewUrl, WebviewWindowBuilder, WindowEvent,
};

// Phase 01: chỉ mở cửa sổ phủ toàn màn hình (giao diện mô phỏng).
#[tauri::command]
async fn open_overlay(app: AppHandle) {
    if app.get_webview_window("overlay").is_some() { return; }
    let _ = WebviewWindowBuilder::new(&app, "overlay", WebviewUrl::App("index.html#overlay".into()))
        .fullscreen(true).decorations(false).transparent(true)
        .always_on_top(true).skip_taskbar(true).focused(true).build();
}

#[tauri::command]
async fn close_overlay(app: AppHandle) {
    if let Some(w) = app.get_webview_window("overlay") { let _ = w.close(); }
}

fn show_main(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("main") { let _ = w.show(); let _ = w.unminimize(); let _ = w.set_focus(); }
}

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_overlay, close_overlay])
        .on_window_event(|w, e| {
            // Đóng cửa sổ chính -> thu xuống khay hệ thống
            if let (WindowEvent::CloseRequested { api, .. }, "main") = (e, w.label()) {
                api.prevent_close();
                let _ = w.hide();
            }
        })
        .setup(|app| {
            let cap = MenuItem::with_id(app, "cap", "Chụp cuộn", true, None::<&str>)?;
            let set = MenuItem::with_id(app, "set", "Cài đặt", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Thoát", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&cap, &set, &quit])?;
            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("CaUL")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, e| match e.id.as_ref() {
                    "cap" => { let a = app.clone(); tauri::async_runtime::spawn(open_overlay(a)); }
                    "set" => show_main(app),
                    _ => app.exit(0),
                })
                .on_tray_icon_event(|t, e| {
                    if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = e {
                        show_main(t.app_handle());
                    }
                })
                .build(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("lỗi khi chạy CaUL");
}
