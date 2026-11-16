# CaUL

Ứng dụng chạy nền trên Windows và Linux, chuyên **chụp ảnh màn hình cuộn tự động** và ghép thành một ảnh dài.

Xây dựng bằng Tauri 2 + Rust + React + TypeScript.

## Tính năng

- Kéo chọn vùng cần chụp, CaUL tự cuộn xuống, chụp từng khung rồi ghép thành một ảnh.
- Tự dừng khi hết nội dung hoặc hết thời gian chờ. Nhấn **Esc** để dừng bất kỳ lúc nào.
- Chạy dưới System Tray, mở nhanh bằng phím tắt toàn cục.
- Lưu ảnh PNG hoặc JPG (chỉnh được chất lượng) vào thư mục tùy chọn.
- Giao diện sáng, tối hoặc theo hệ thống, tông màu vàng.

## Cách dùng

1. Mở chế độ chụp bằng phím tắt (mặc định `Ctrl+Shift+S`), nút **Chụp cuộn** hoặc menu ở khay hệ thống.
2. Kéo chuột chọn vùng cần chụp. Chừa khoảng trống ~76px phía trên hoặc dưới vùng chọn để đặt thanh công cụ.
3. Bấm **Bắt đầu**. CaUL cuộn và chụp tự động.
4. Ảnh được lưu vào thư mục đã đặt, tên dạng `CaUL_<mã thời gian>.png`.

### Cài đặt

| Mục | Nội dung |
|---|---|
| Chụp cuộn | Tốc độ cuộn, khoảng nghỉ giữa các lần chụp, thời gian chờ tối đa |
| Phím chụp | Phím mở chế độ chụp (bấm vào ô rồi nhấn tổ hợp mới), Esc để dừng |
| Chung | Khởi động cùng hệ thống, chạy dưới nền, thu nhỏ xuống khay |
| Lưu ảnh | Thư mục lưu, định dạng PNG/JPG, chất lượng JPG |
| Giao diện | Sáng, Tối, Hệ thống |

Nút **Đóng** ẩn cửa sổ xuống khay (app vẫn chạy nền). Nút **Kết thúc** thoát hẳn.

## Chạy khi phát triển

Yêu cầu: Node.js 20+, Rust, và [các thư viện nền của Tauri 2](https://tauri.app/start/prerequisites/).

```bash
npm install
npx tauri icon public/logo.svg    # tạo bộ icon từ logo
npm run tauri dev
```

Linux cần thêm:

```bash
sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev patchelf \
  libxdo-dev libxcb1-dev libxrandr-dev libdbus-1-dev libpipewire-0.3-dev libgbm-dev
```

## Đóng gói

Mỗi hệ điều hành phải build trên chính hệ điều hành đó.

| Hệ điều hành | Cấu hình `targets` trong `tauri.conf.json` | Lệnh | File tạo ra |
|---|---|---|---|
| Windows | `["nsis"]` | `npm run tauri build` | `src-tauri/target/release/bundle/nsis/*.exe` |
| Linux | `["deb"]` | `npm run tauri build -- --bundles deb` | `src-tauri/target/release/bundle/deb/*.deb` |

Windows cần thêm Visual Studio Build Tools (workload "Desktop development with C++"). Cài file deb bằng `sudo apt install ./CaUL_0.1.0_amd64.deb`.

Không có máy Linux hoặc Windows tương ứng thì dùng WSL2 hoặc GitHub Actions để build.

## Cấu trúc thư mục

```
public/logo.svg         Logo ứng dụng
src/
  main.tsx              Điểm vào, chọn cửa sổ chính hoặc lớp phủ
  App.tsx               Cửa sổ chính (cài đặt)
  Overlay.tsx           Giao diện chụp cuộn (lớp phủ, chọn vùng, trạng thái)
  ui.tsx                Công tắc, thanh trượt, ô ghi phím tắt
  store.ts              Giá trị mặc định và lưu cài đặt
  styles.css            Màu sắc, bố cục, hiệu ứng
src-tauri/
  src/lib.rs            Tray, cửa sổ, phím tắt toàn cục, các lệnh Tauri
  src/capture.rs        Chụp màn hình, cuộn, ghép ảnh, lưu file
  tauri.conf.json       Cấu hình cửa sổ và đóng gói
```

## Giới hạn hiện tại

- Chỉ chụp trên màn hình chứa lớp phủ, chưa chọn được màn hình khác.
- Trang có thanh cố định (header, footer dính) trong vùng chọn sẽ bị lặp ở chỗ ghép. Hãy chọn vùng chỉ gồm phần nội dung cuộn.
- Trang cuộn mượt hoặc có hoạt ảnh có thể ghép lệch. Tăng "Khoảng nghỉ" nếu gặp.
- Linux chạy tốt trên X11. Wayland thường chặn phím tắt toàn cục, giả lập cuộn chuột và chụp màn hình.
- Công tắc "Khởi động cùng hệ thống", "Chạy dưới nền", "Thu nhỏ xuống khay" hiện chỉ lưu giá trị, chưa tác động đến hệ điều hành.
- Phím tắt mặc định `Ctrl+Shift+S` trùng với "Lưu thành" của nhiều app. Nên đổi sang tổ hợp ít dùng như `Ctrl+Alt+S`.

## Lộ trình

- Phase 01: giao diện mô phỏng (hoàn thành).
- Phase 02: chụp thật, tự cuộn, ghép ảnh, lưu (hoàn thành).
- Tiếp theo: khởi động cùng hệ thống, lưu cài đặt ra file, chọn màn hình, xử lý thanh cố định.
