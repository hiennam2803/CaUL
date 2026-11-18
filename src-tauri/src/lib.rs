mod capture;
use std::{
    panic::{catch_unwind, AssertUnwindSafe},
    sync::atomic::Ordering,
    time::Duration,
};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, LogicalSize, Manager, WebviewUrl, WebviewWindowBuilder, WindowEvent,
};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

#[tauri::command]
async fn open_overlay(app: AppHandle) {
    if app.get_webview_window("overlay").is_some() { return; }
    // Ẩn cửa sổ chính để không lọt vào ảnh chụp
    if let Some(m) = app.get_webview_window("main") { let _ = m.hide(); }
    capture::STOP.store(false, Ordering::SeqCst);
    capture::CAPTURING.store(false, Ordering::SeqCst);
    let built = WebviewWindowBuilder::new(&app, "overlay", WebviewUrl::App("index.html#overlay".into()))
        .fullscreen(true).decorations(false).transparent(true)
        .always_on_top(true).skip_taskbar(true).visible(false).build();
    if built.is_err() { return; }
    // Esc toàn cục (luôn hoạt động kể cả khi lớp phủ không nhận được bàn phím):
    // đang chụp thì dừng, chưa chụp thì đóng lớp phủ.
    let _ = app.global_shortcut().on_shortcut("Escape", |app, _, ev| {
        if ev.state() != ShortcutState::Pressed { return; }
        if capture::CAPTURING.load(Ordering::SeqCst) { capture::STOP.store(true, Ordering::SeqCst); }
        else if let Some(w) = app.get_webview_window("overlay") { let _ = w.close(); }
    });
    // Dự phòng: sau 1,5 giây nếu lớp phủ chưa hiện thì hiện luôn
    let a = app.clone();
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(1500));
        if let Some(w) = a.get_webview_window("overlay") {
            if !w.is_visible().unwrap_or(true) { let _ = w.show(); let _ = w.set_focus(); }
        }
    });
}

// Giao diện lớp phủ vẽ xong mới hiện, tránh nháy khung trắng
#[tauri::command]
fn overlay_ready(app: AppHandle) {
    if let Some(w) = app.get_webview_window("overlay") { let _ = w.show(); let _ = w.set_focus(); }
}

#[tauri::command]
async fn close_overlay(app: AppHandle) {
    if let Some(w) = app.get_webview_window("overlay") { let _ = w.close(); }
}

// Chụp thật: lớp phủ chuyển sang "xuyên chuột"; Esc toàn cục (đăng ký lúc mở lớp phủ) để dừng.
#[tauri::command]
async fn start_capture(app: AppHandle, region: capture::Region, opts: capture::Opts) -> Result<(), String> {
    let w = app.get_webview_window("overlay").ok_or("Không có lớp phủ")?;
    let p = w.outer_position().map_err(|e| e.to_string())?;
    let _ = w.set_ignore_cursor_events(true);
    capture::STOP.store(false, Ordering::SeqCst);
    capture::CAPTURING.store(true, Ordering::SeqCst);
    let a = app.clone();
    std::thread::spawn(move || {
        // Dù lỗi hay panic vẫn phải trả lại chuột cho người dùng
        let r = catch_unwind(AssertUnwindSafe(|| capture::run(&a, (p.x, p.y), region, &opts)))
            .unwrap_or_else(|_| Err("Lỗi nội bộ khi chụp".into()));
        capture::CAPTURING.store(false, Ordering::SeqCst);
        if let Some(w) = a.get_webview_window("overlay") { let _ = w.set_ignore_cursor_events(false); let _ = w.set_focus(); }
        let res = match r { Ok(m) => (true, m), Err(e) => (false, e) };
        let _ = a.emit("cap-done", res);
    });
    Ok(())
}

// Đăng ký phím tắt toàn cục mở chế độ chụp
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

// Bật / tắt khởi động cùng hệ điều hành (Windows: registry, Linux: ~/.config/autostart)
#[tauri::command]
fn set_autostart(app: AppHandle, on: bool) -> Result<(), String> {
    let a = app.autolaunch();
    let cur = a.is_enabled().unwrap_or(false);
    let r = if on && !cur { a.enable() } else if !on && cur { a.disable() } else { Ok(()) };
    r.map_err(|e| e.to_string())
}

#[tauri::command]
fn hide_main(app: AppHandle) {
    if let Some(w) = app.get_webview_window("main") { let _ = w.hide(); }
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
        // Mở app lần 2 thì chỉ hiện cửa sổ của bản đang chạy, không chạy trùng
        .plugin(tauri_plugin_single_instance::init(|app, _, _| show_main(app)))
        // Khi khởi động cùng hệ thống, app được chạy với tham số --hidden
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec!["--hidden"])))
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            open_overlay, overlay_ready, close_overlay, start_capture,
            set_hotkey, set_autostart, hide_main, quit, resize_main
        ])
        .on_window_event(|w, e| {
            // Lớp phủ đóng thì gỡ phím Esc toàn cục
            if let (WindowEvent::Destroyed, "overlay") = (e, w.label()) {
                let _ = w.app_handle().global_shortcut().unregister("Escape");
            }
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
            // Cửa sổ chính khởi tạo ẩn; chỉ hiện khi người dùng tự mở app (không có --hidden)
            if !std::env::args().any(|a| a == "--hidden") { show_main(app.handle()); }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("lỗi khi chạy CaUL");
}