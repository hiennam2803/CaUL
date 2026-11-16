mod capture;
use std::sync::atomic::Ordering;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, LogicalSize, Manager, WebviewUrl, WebviewWindowBuilder, WindowEvent,
};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

#[tauri::command]
async fn open_overlay(app: AppHandle) {
    if app.get_webview_window("overlay").is_some() { return; }
    // Ẩn cửa sổ chính để không lọt vào ảnh chụp
    if let Some(m) = app.get_webview_window("main") { let _ = m.hide(); }
    let _ = WebviewWindowBuilder::new(&app, "overlay", WebviewUrl::App("index.html#overlay".into()))
        .fullscreen(true).decorations(false).transparent(true)
        .always_on_top(true).skip_taskbar(true).visible(false).build();
}

#[tauri::command]
async fn close_overlay(app: AppHandle) {
    if let Some(w) = app.get_webview_window("overlay") { let _ = w.close(); }
}

// Chụp thật: lớp phủ chuyển sang "xuyên chuột", Esc (toàn cục) để dừng.
#[tauri::command]
async fn start_capture(app: AppHandle, region: capture::Region, opts: capture::Opts) -> Result<(), String> {
    let w = app.get_webview_window("overlay").ok_or("Không có lớp phủ")?;
    let p = w.outer_position().map_err(|e| e.to_string())?;
    let _ = w.set_ignore_cursor_events(true);
    capture::STOP.store(false, Ordering::SeqCst);
    let _ = app.global_shortcut().on_shortcut("Escape", |_, _, ev| {
        if ev.state() == ShortcutState::Pressed { capture::STOP.store(true, Ordering::SeqCst); }
    });
    let a = app.clone();
    std::thread::spawn(move || {
        let r = capture::run(&a, (p.x, p.y), region, &opts);
        let _ = a.global_shortcut().unregister("Escape");
        if let Some(w) = a.get_webview_window("overlay") { let _ = w.set_ignore_cursor_events(false); let _ = w.set_focus(); }
        let res = match r { Ok(p) => (true, p.display().to_string()), Err(e) => (false, e) };
        let _ = a.emit("cap-done", res);
    });
    Ok(())
}

#[tauri::command]
fn hide_main(app: AppHandle) {
    if let Some(w) = app.get_webview_window("main") { let _ = w.hide(); }
}

#[tauri::command]
fn overlay_ready(app: AppHandle) {
    if let Some(w) = app.get_webview_window("overlay") { let _ = w.show(); let _ = w.set_focus(); }
}

#[tauri::command]
fn set_hotkey(app: AppHandle, old: Option<String>, key: String) -> Result<(), String> {
    let gs = app.global_shortcut();
    if let Some(o) = old { let _ = gs.unregister(o.as_str()); }
    gs.on_shortcut(key.as_str(), |app, _, ev| {
        if ev.state() == ShortcutState::Pressed {
            let a = app.clone();
            tauri::async_runtime::spawn(open_overlay(a));
        }
    }).map_err(|e| e.to_string())
}

#[tauri::command]
fn quit(app: AppHandle) { app.exit(0); }

// Mở rộng / thu gọn cửa sổ chính (cửa sổ không cho kéo, nên bật tạm resizable)
#[tauri::command]
fn resize_main(app: AppHandle, expanded: bool) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.set_resizable(true);
        let _ = w.set_size(LogicalSize::new(560.0, if expanded { 520.0 } else { 300.0 }));
        let _ = w.set_resizable(false);
    }
}

fn show_main(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("main") { let _ = w.show(); let _ = w.unminimize(); let _ = w.set_focus(); }
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            open_overlay, close_overlay, start_capture, hide_main, quit, resize_main, set_hotkey, overlay_ready
        ])
        .on_window_event(|w, e| {
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
