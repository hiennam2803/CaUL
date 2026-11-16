import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useSettings } from "./store";
import { Row, Toggle, Range, Seg, KeyBox } from "./ui";

const NAV = [["general", "⚙", "Chung"], ["scroll", "↕", "Chụp cuộn"], ["keys", "⌨", "Phím tắt"],
  ["save", "▣", "Lưu ảnh"], ["theme", "◐", "Giao diện"], ["about", "ⓘ", "Giới thiệu"]] as const;

export default function App() {
  const [page, setPage] = useState<string>("general");
  const [s, up] = useSettings();
  const demo = () => invoke("open_overlay").catch(() => alert("Chỉ chạy được trong cửa sổ Tauri"));

  const pages: Record<string, JSX.Element> = {
    general: <>
      <Row t="Khởi động cùng hệ điều hành" d="Tự mở CaUL khi đăng nhập"><Toggle v={s.autostart} on={v => up({ autostart: v })} /></Row>
      <Row t="Chạy dưới nền" d="Giữ CaUL hoạt động khi đóng cửa sổ"><Toggle v={s.background} on={v => up({ background: v })} /></Row>
      <Row t="Thu nhỏ xuống System Tray" d="Hiện biểu tượng ở khay hệ thống"><Toggle v={s.tray} on={v => up({ tray: v })} /></Row>
    </>,
    scroll: <>
      <Row t="Tốc độ cuộn" d="Mức độ cuộn mỗi bước"><Range v={s.speed} min={1} max={10} unit="" on={v => up({ speed: v })} /></Row>
      <Row t="Khoảng nghỉ" d="Chờ giữa các lần chụp"><Range v={s.delay} min={100} max={1000} unit=" ms" on={v => up({ delay: v })} /></Row>
      <Row t="Thời gian chờ" d="Tự dừng nếu quá thời gian này"><Range v={s.timeout} min={5} max={120} unit=" s" on={v => up({ timeout: v })} /></Row>
    </>,
    keys: <>
      <Row t="Bắt đầu chụp" d="Bấm rồi nhấn tổ hợp phím mới"><KeyBox v={s.startKey} on={v => up({ startKey: v })} /></Row>
      <Row t="Dừng / hủy" d="Cố định"><kbd>Esc</kbd></Row>
    </>,
    save: <>
      <Row t="Thư mục lưu"><input className="in" value={s.dir} onChange={e => up({ dir: e.target.value })} /></Row>
      <Row t="Định dạng"><Seg v={s.fmt} opts={[["PNG", "PNG"], ["JPG", "JPG"]]} on={v => up({ fmt: v })} /></Row>
      {s.fmt === "JPG" && <Row t="Chất lượng ảnh"><Range v={s.quality} min={10} max={100} unit="%" on={v => up({ quality: v })} /></Row>}
    </>,
    theme: <Row t="Chế độ hiển thị">
      <Seg v={s.theme} opts={[["light", "Sáng"], ["dark", "Tối"], ["system", "Hệ thống"]]} on={v => up({ theme: v })} />
    </Row>,
    about: <div className="about"><div className="logo big">C</div><h2>CaUL</h2><p>Chụp màn hình cuộn tự động</p>
      <small>Phiên bản 0.1.0 · Phase 01 (giao diện mô phỏng)<br />Tauri 2 · Rust · React</small></div>,
  };
  const cur = NAV.find(n => n[0] === page)!;

  return (
    <div className="app">
      <aside>
        <div className="brand"><div className="logo">C</div>CaUL</div>
        {NAV.map(([k, i, l]) => (
          <button key={k} className={"nav" + (k === page ? " on" : "")} onClick={() => setPage(k)}><span>{i}</span>{l}</button>
        ))}
        <button className="btn" onClick={demo}>Thử chụp cuộn</button>
      </aside>
      <main key={page}><h1>{cur[2]}</h1><div className="card">{pages[page]}</div></main>
    </div>
  );
}
