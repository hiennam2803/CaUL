import { ReactNode, useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { DEF, useSettings } from "./store";
import { Row, Toggle, Range, Seg, KeyBox } from "./ui";


const cmd = (c: string, a?: Record<string, unknown>) => invoke(c, a).catch(() => { });

const Grp = ({ t, cls = "", children }: { t: string; cls?: string; children: ReactNode }) => (
  <fieldset className={"grp " + cls}><legend>{t}</legend>{children}</fieldset>
);


const DLG = {
  help: ["Hướng dẫn", "1. Nhấn phím tắt hoặc chọn Chụp cuộn.\n2. Bấm Bắt đầu để chụp tự động.\n3. Nhấn Esc để dừng hoặc hủy."],
  about: ["CaUL 0.1.0", "Chụp màn hình cuộn tự động.\nPhase 01 – giao diện mô phỏng.\nTauri 2 · Rust · React"],
};



export default function App() {
  const [s, up] = useSettings();
  const [ex, setEx] = useState(false);
  const [dlg, setDlg] = useState<"" | "help" | "about">("");
  const oldKey = useRef<string>();
  useEffect(() => {
    cmd("set_hotkey", { old: oldKey.current, key: s.startKey });
    oldKey.current = s.startKey;
  }, [s.startKey]);
  const toggle = () => { setEx(!ex); cmd("resize_main", { expanded: !ex }); };

  return (
    <div className="win">
      <header>
        <div className="logo" />
        <div><b>CaUL</b><small>Chụp màn hình cuộn</small></div>
        <span className="chip">v0.1.0</span>
      </header>

      <div className="top">
        <Grp t="Chụp cuộn">
          <Row t="Tốc độ cuộn"><Range v={s.speed} min={1} max={10} unit="" on={v => up({ speed: v })} /></Row>
          <Row t="Khoảng nghỉ"><Range v={s.delay} min={100} max={1000} unit=" ms" on={v => up({ delay: v })} /></Row>
          <Row t="Thời gian chờ"><Range v={s.timeout} min={5} max={120} unit=" s" on={v => up({ timeout: v })} /></Row>
          <Row t="Phím chụp">
            <div className="rg"><KeyBox v={s.startKey} on={v => up({ startKey: v })} /><small>Dừng: <kbd>Esc</kbd></small></div>
          </Row>
        </Grp>
        <div className="side">
          <button className="b pri" onClick={() => cmd("open_overlay")}>Chụp cuộn</button>
          <button className="b" onClick={() => cmd("hide_main")}>Đóng</button>
          <button className="b" onClick={() => cmd("quit")}>Kết thúc</button>
          <button className="b" onClick={toggle}>{ex ? "Thu gọn ▴" : "Mở rộng ▾"}</button>
        </div>
      </div>

      <div className="foot">
        <button className="b" onClick={() => setDlg("help")}>Hướng dẫn</button>
        <button className="b" onClick={() => setDlg("about")}>Thông tin</button>
        <button className="b" onClick={() => up(DEF)}>Mặc định</button>
      </div>

      {ex && (
        <div className="ext">
          <Grp t="Chung">
            <Row t="Khởi động cùng hệ thống"><Toggle v={s.autostart} on={v => up({ autostart: v })} /></Row>
            <Row t="Chạy dưới nền"><Toggle v={s.background} on={v => up({ background: v })} /></Row>
            <Row t="Thu nhỏ xuống khay"><Toggle v={s.tray} on={v => up({ tray: v })} /></Row>
          </Grp>
          <Grp t="Lưu ảnh">
            <Row t="Thư mục"><input className="in" value={s.dir} onChange={e => up({ dir: e.target.value })} /></Row>
            <Row t="Định dạng"><Seg v={s.fmt} opts={[["PNG", "PNG"], ["JPG", "JPG"]]} on={v => up({ fmt: v })} /></Row>
            <div className={s.fmt === "JPG" ? "" : "off"}>
              <Row t="Chất lượng"><Range v={s.quality} min={10} max={100} unit="%" on={v => up({ quality: v })} /></Row>
            </div>
          </Grp>
          <Grp t="Giao diện" cls="full">
            <Row t="Chế độ hiển thị">
              <Seg v={s.theme} opts={[["light", "Sáng"], ["dark", "Tối"], ["system", "Hệ thống"]]} on={v => up({ theme: v })} />
            </Row>
          </Grp>
        </div>
      )}

      {dlg && (
        <div className="mask" onClick={() => setDlg("")}>
          <div className="dlg" onClick={e => e.stopPropagation()}>
            <div className="logo big" />
            <h3>{DLG[dlg][0]}</h3>
            <p>{DLG[dlg][1]}</p>
            <button className="btn" onClick={() => setDlg("")}>OK</button>
          </div>
        </div>
      )}
    </div>
  );
}