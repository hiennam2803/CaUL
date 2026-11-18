use std::{
    borrow::Cow, io::BufWriter, path::PathBuf, sync::{atomic::{AtomicBool, Ordering}, Mutex}, thread::sleep,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use enigo::{Axis, Coordinate, Enigo, Mouse, Settings};
use image::{codecs::jpeg::JpegEncoder, imageops, DynamicImage, RgbaImage};
use serde::Deserialize;
use tauri::{AppHandle, Emitter};
use xcap::Monitor;

pub static STOP: AtomicBool = AtomicBool::new(false);
pub static CAPTURING: AtomicBool = AtomicBool::new(false);
static CLIP: Mutex<Option<arboard::Clipboard>> = Mutex::new(None);

#[derive(Deserialize)]
pub struct Region { pub x: u32, pub y: u32, pub w: u32, pub h: u32 }

#[derive(Deserialize)]
pub struct Opts { pub speed: i32, pub delay: u64, pub timeout: u64, pub dir: String, pub fmt: String, pub quality: u8, pub clip: bool, pub save: bool }

fn e<T: ToString>(x: T) -> String { x.to_string() }

// Ảnh xám thu nhỏ (mỗi 4 điểm ảnh theo chiều ngang) để so khớp nhanh.
fn luma(img: &RgbaImage) -> Vec<Vec<u8>> {
    img.rows().map(|r| r.step_by(4).map(|p| ((p.0[0] as u16 * 3 + p.0[1] as u16 * 6 + p.0[2] as u16) / 10) as u8).collect()).collect()
}

// Tìm số dòng mới `d` mà khung `b` cuộn thêm so với khung `a` (None nếu không khớp).
fn shift(a: &[Vec<u8>], b: &[Vec<u8>]) -> Option<usize> {
    let h = a.len();
    let strip = (h / 4).clamp(8, 120).min(h / 2);
    let mut best = (u64::MAX, 0);
    for d in 1..=(h - strip) {
        let c: u64 = (0..strip).step_by(2).map(|k| a[d + k].iter().zip(&b[k]).map(|(x, y)| x.abs_diff(*y) as u64).sum::<u64>()).sum();
        if c < best.0 { best = (c, d); }
    }
    let n = ((strip + 1) / 2 * a[0].len()).max(1) as u64;
    (best.0 / n < 6).then_some(best.1)
}

fn save(img: &RgbaImage, o: &Opts) -> Result<PathBuf, String> {
    let mut dir = o.dir.clone();
    if let Some(rest) = dir.strip_prefix('~') {
        let home = std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")).unwrap_or_default();
        dir = format!("{home}{rest}");
    }
    std::fs::create_dir_all(&dir).map_err(e)?;
    let t = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    let jpg = o.fmt == "JPG";
    let p = PathBuf::from(dir).join(format!("CaUL_{t}.{}", if jpg { "jpg" } else { "png" }));
    if jpg {
        let f = std::fs::File::create(&p).map_err(e)?;
        let enc = JpegEncoder::new_with_quality(BufWriter::new(f), o.quality.clamp(10, 100));
        DynamicImage::ImageRgb8(DynamicImage::ImageRgba8(img.clone()).to_rgb8()).write_with_encoder(enc).map_err(e)?;
    } else {
        img.save(&p).map_err(e)?;
    }
    Ok(p)
}

fn copy_clip(img: &RgbaImage) -> Result<(), String> {
    let mut g = CLIP.lock().map_err(e)?;
    if g.is_none() { *g = Some(arboard::Clipboard::new().map_err(e)?); }
    let data = arboard::ImageData { width: img.width() as usize, height: img.height() as usize, bytes: Cow::Borrowed(img.as_raw()) };
    g.as_mut().unwrap().set_image(data).map_err(e)
}

/// Chụp vùng `r`, tự cuộn xuống, ghép các khung và lưu. `origin` là góc trên-trái màn hình chứa vùng chọn.
pub fn run(app: &AppHandle, origin: (i32, i32), r: Region, o: &Opts) -> Result<String, String> {
    let mon = Monitor::from_point(origin.0 + 1, origin.1 + 1).map_err(e)?;
    let mut mouse = Enigo::new(&Settings::default()).map_err(e)?;
    mouse.move_mouse(origin.0 + (r.x + r.w / 2) as i32, origin.1 + (r.y + r.h / 2) as i32, Coordinate::Abs).map_err(e)?;

    let grab = || -> Result<RgbaImage, String> {
        let f = mon.capture_image().map_err(e)?;
        let (x, y) = (r.x.min(f.width() - 1), r.y.min(f.height() - 1));
        let (w, h) = (r.w.min(f.width() - x), r.h.min(f.height() - y));
        Ok(imageops::crop_imm(&f, x, y, w, h).to_image())
    };

    let first = grab()?;
    let mut prev = luma(&first);
    let mut parts = vec![first];
    let mut clicks = o.speed.max(1);
    let t0 = Instant::now();
    sleep(Duration::from_millis(150));

    while !STOP.load(Ordering::SeqCst) && t0.elapsed().as_secs() < o.timeout {
        mouse.scroll(clicks, Axis::Vertical).map_err(e)?;
        sleep(Duration::from_millis(o.delay));
        let cur = grab()?;
        let cl = luma(&cur);
        if cl == prev { break; } // không còn nội dung để cuộn
        match shift(&prev, &cl) {
            Some(d) => {
                let h = cur.height();
                parts.push(imageops::crop_imm(&cur, 0, h - d as u32, cur.width(), d as u32).to_image());
            }
            None => { parts.push(cur); clicks = (clicks / 2).max(1); } // mất dấu: giảm bước cuộn
        }
        prev = cl;
        let _ = app.emit("cap-frame", parts.len());
    }

    let w = parts[0].width();
    let mut out = RgbaImage::new(w, parts.iter().map(|p| p.height()).sum());
    let mut y = 0i64;
    for p in &parts { imageops::replace(&mut out, p, 0, y); y += p.height() as i64; }
        let (clip, save_file) = (o.clip, o.save || !o.clip);
    let mut msg = Vec::new();
    if save_file { msg.push(save(&out, o)?.display().to_string()); }
    if clip {
        match copy_clip(&out) {
            Ok(()) => msg.push("Đã sao chép vào clipboard (Ctrl+V để dán)".to_string()),
            Err(er) if save_file => msg.push(format!("Không chép được vào clipboard: {er}")),
            Err(er) => return Err(format!("Không chép được vào clipboard: {er}")),
        }
    }
    Ok(msg.join(" · "))
}
