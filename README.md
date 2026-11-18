# CaUL

Ứng dụng chạy nền trên Windows và Linux, chuyên **chụp ảnh màn hình cuộn tự động** và ghép thành một ảnh dài.

Xây dựng bằng Tauri 2 + Rust + React + TypeScript. Phiên bản hiện tại: **0.3.0**.

## Tính năng

- Kéo chọn vùng cần chụp, CaUL tự cuộn xuống, chụp từng đoạn rồi ghép thành một ảnh.
- Tự dừng khi hết nội dung hoặc hết thời gian chờ. Nhấn **Esc** để dừng bất kỳ lúc nào.
- **Tự sao chép ảnh vào clipboard** sau khi chụp, dán ngay bằng `Ctrl+V` như ảnh chụp màn hình thông thường (bật/tắt được).
- Phím tắt toàn cục để mở chế độ chụp, tùy chỉnh được (mặc định `Ctrl+Shift+S`).
- Chạy dưới System Tray, có thể **tự khởi động cùng hệ thống** và chạy ngầm, không hiện cửa sổ khi bật máy.
- Chỉ cho chạy một bản duy nhất: mở app lần nữa sẽ hiện cửa sổ của bản đang chạy.
- Lưu ảnh PNG hoặc JPG (chỉnh được chất lượng) vào thư mục tùy chọn.
- Giao diện sáng, tối hoặc theo hệ thống, tông màu vàng.

## Cách dùng

1. Nhấn phím tắt, bấm **Chụp cuộn** trong cửa sổ chính, hoặc chọn **Chụp cuộn** ở menu khay hệ thống.
2. Kéo chuột chọn vùng nội dung cần cuộn. Chừa khoảng trống ~76px phía trên hoặc dưới vùng chọn để đặt thanh công cụ.
3. Bấm **Bắt đầu**. CaUL cuộn và chụp tự động.
4. Nhấn **Esc** để dừng. Ảnh được ghép, chép vào clipboard (dán bằng `Ctrl+V`) và lưu vào thư mục đã đặt, tên dạng `CaUL_<mã thời gian>.png`.

Nút **Đóng** ẩn cửa sổ xuống khay (app vẫn chạy nền). Nút **Kết thúc** thoát hẳn.

### Clipboard (Ctrl+V)

Sau khi ghép xong, ảnh được chép vào clipboard dưới dạng ảnh, dán được vào Zalo, Word, Paint, Discord, trình duyệt... giống ảnh chụp màn hình thường.

- Chỉ muốn chép vào clipboard như `Win+Shift+S`: tắt **Lưu thành file**.
- Chỉ muốn lưu file: tắt **Sao chép vào clipboard**.
- Tắt cả hai thì app vẫn lưu file để ảnh không bị mất.
- Nếu chép vào clipboard lỗi nhưng file đã lưu, thông báo cuối sẽ ghi rõ lỗi, ảnh vẫn an toàn.
- Linux: ảnh trong clipboard chỉ dán được khi CaUL còn chạy. Đừng bấm **Kết thúc** trước khi dán.

### Khởi động cùng hệ thống

Bật công tắc **Khởi động cùng hệ thống** trong phần **Mở rộng**. Khi bật máy, CaUL tự chạy ngầm: chỉ có icon ở khay hệ thống, không hiện cửa sổ, phím tắt vẫn hoạt động. Bấm vào icon để mở cửa sổ cài đặt.

- Windows: đăng ký ở mục Startup apps (registry).
- Linux: tạo file `~/.config/autostart/caul.desktop`.
- Chỉ kiểm tra bằng **bản đã cài đặt**, không dùng `npm run tauri dev` (bản dev đăng ký đường dẫn file tạm).

### Cài đặt

Cài đặt được lưu lại và không bị reset khi tắt app hoặc khởi động lại máy (chỉ mất khi bấm **Mặc định** hoặc xóa dữ liệu app).

| Mục | Nội dung |
|---|---|
| Chụp cuộn | Tốc độ cuộn, khoảng nghỉ giữa các lần chụp, thời gian chờ tối đa |
| Phím chụp | Bấm vào ô rồi nhấn tổ hợp mới. Esc cố định để dừng |
| Chung | Khởi động cùng hệ thống, chạy dưới nền, thu nhỏ xuống khay |
| Lưu ảnh | Sao chép vào clipboard, lưu thành file, thư mục lưu, định dạng PNG/JPG, chất lượng JPG |
| Giao diện | Sáng, Tối, Hệ thống |

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

| Hệ điều hành | `targets` trong `tauri.conf.json` | Lệnh | File tạo ra |
|---|---|---|---|
| Windows | `["nsis"]` | `npm run tauri build` | `src-tauri/target/release/bundle/nsis/CaUL_0.3.0_x64-setup.exe` |
| Linux | `["deb"]` | `npm run tauri build -- --bundles deb` | `src-tauri/target/release/bundle/deb/CaUL_0.3.0_amd64.deb` |

Windows cần thêm Visual Studio Build Tools (workload "Desktop development with C++"). Cài file deb bằng `sudo apt install ./CaUL_0.3.0_amd64.deb`. Không có máy Linux thì dùng WSL2 hoặc GitHub Actions để build.

Dung lượng dự án có thể lên tới vài GB do thư mục `src-tauri/target` và `node_modules`. Hai thư mục này sinh lại được, xóa an toàn khi cần dọn đĩa (`cargo clean` trong `src-tauri`).

## Cấu trúc thư mục

```
public/logo.svg         Logo ứng dụng
src/
  main.tsx              Điểm vào, chọn cửa sổ chính hoặc lớp phủ
  App.tsx               Cửa sổ chính (cài đặt, hướng dẫn, thông tin)
  Overlay.tsx           Giao diện chụp cuộn (lớp phủ, chọn vùng, trạng thái)
  ui.tsx                Công tắc, thanh trượt, ô ghi phím tắt
  store.ts              Giá trị mặc định và lưu cài đặt
  styles.css            Màu sắc, bố cục, hiệu ứng
src-tauri/
  src/lib.rs            Tray, cửa sổ, phím tắt, khởi động cùng hệ thống, các lệnh Tauri
  src/capture.rs        Chụp màn hình, cuộn, ghép ảnh, chép clipboard, lưu file
  tauri.conf.json       Cấu hình cửa sổ và đóng gói
```

## Giới hạn hiện tại

- Chỉ chụp trên màn hình chứa lớp phủ, chưa chọn được màn hình khác.
- Trang có thanh cố định (header, footer dính) trong vùng chọn sẽ bị lặp ở chỗ ghép. Hãy chọn vùng chỉ gồm phần nội dung cuộn.
- Trang cuộn mượt hoặc có hoạt ảnh có thể ghép lệch. Tăng "Khoảng nghỉ" nếu gặp.
- Linux chạy tốt trên X11. Wayland thường chặn phím tắt toàn cục, giả lập cuộn chuột và chụp màn hình.
- Linux: nội dung clipboard mất khi thoát CaUL.
- Ảnh quá dài có thể chiếm nhiều RAM khi chép vào clipboard.
- Hai công tắc "Chạy dưới nền" và "Thu nhỏ xuống khay" hiện chỉ lưu giá trị, chưa tác động đến hành vi của app.
- Phím mặc định `Ctrl+Shift+S` trùng với "Lưu thành" của nhiều app. Nên đổi sang tổ hợp ít dùng như `Ctrl+Alt+S`.

## Lịch sử phiên bản

### 0.3.0
- Tự sao chép ảnh vào clipboard sau khi chụp, dán bằng `Ctrl+V` như ảnh chụp màn hình thường.
- Thêm hai công tắc trong **Lưu ảnh**: **Sao chép vào clipboard** và **Lưu thành file** (mặc định bật cả hai).
- Tắt cả hai công tắc thì vẫn lưu file, tránh mất ảnh.
- Thông báo cuối hiển thị đường dẫn file và trạng thái clipboard; nếu chép lỗi thì ghi rõ lý do.
- Thêm thư viện `arboard`.

### 0.2.0
- Chụp màn hình cuộn thật: chọn vùng, tự cuộn, ghép ảnh, lưu PNG/JPG.
- Phím tắt toàn cục tùy chỉnh để mở chế độ chụp.
- Tự khởi động cùng hệ thống và chạy ngầm không hiện cửa sổ.
- Chặn mở trùng nhiều bản.
- Esc toàn cục để dừng hoặc hủy, luôn thoát được lớp phủ.
- Giao diện vàng mới, có logo, hộp Hướng dẫn và Thông tin viết lại.

### 0.1.0
- Giao diện mô phỏng: cài đặt, khay hệ thống, lớp phủ chụp cuộn.

## Lộ trình

- Làm cho hai công tắc "Chạy dưới nền" và "Thu nhỏ xuống khay" có tác dụng thật.
- Chọn màn hình khác khi có nhiều màn hình.
- Xử lý thanh cố định khi ghép ảnh.