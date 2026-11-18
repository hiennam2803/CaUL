import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useSettings } from "./store";

type St = "idle" | "run" | "done" | "err";
type R = { x: number; y: number; w: number; h: number };

export default function Overlay() {
  const [s] = useSettings();
  const [st, setSt] = useState<St>("idle");
  const [r, setR] = useState<R | null>(null);
  const [n, setN] = useState(0);
  const [msg, setMsg] = useState("");
  const a0 = useRef<[number, number] | null>(null);
  const close = () => invoke("close_overlay").catch(() => window.close());

  useEffect(() => {
    requestAnimationFrame(() => invoke("overlay_ready").catch(() => { }));
  }, []);

  useEffect(() => {
    invoke("overlay_ready").catch(() => {});
  }, []);

    // Chụp xong thì đóng lớp phủ ngay (lỗi thì giữ lại để đọc)
  useEffect(() => {
    if (st === "done") close();
  }, [st]);

  useEffect(() => {
    const k = (e: KeyboardEvent) => e.key === "Escape" && st !== "run" && close();
    addEventListener("keydown", k);
    return () => removeEventListener("keydown", k);
  }, [st]);

  useEffect(() => {
    const u = [
      listen<number>("cap-frame", e => setN(e.payload)),
      listen<[boolean, string]>("cap-done", e => { setSt(e.payload[0] ? "done" : "err"); setMsg(e.payload[1]); }),
    ];
    return () => u.forEach(p => p.then(f => f()));
  }, []);

  const down = (e: React.MouseEvent) => {
    if (st !== "idle" || (e.target as HTMLElement).closest(".bar")) return;
    a0.current = [e.clientX, e.clientY];
    setR({ x: e.clientX, y: e.clientY, w: 0, h: 0 });
  };
  const move = (e: React.MouseEvent) => {
    const a = a0.current;
    if (a) setR({ x: Math.min(a[0], e.clientX), y: Math.min(a[1], e.clientY), w: Math.abs(e.clientX - a[0]), h: Math.abs(e.clientY - a[1]) });
  };
  const up = () => { a0.current = null; setR(v => (v && v.w > 40 && v.h > 40 ? v : null)); };

  // Thanh công cụ đặt ngoài vùng chọn (trên hoặc dưới) để không lọt vào ảnh.
  const above = !!r && r.y >= 76;
  const below = !!r && innerHeight - (r.y + r.h) >= 76;
  const room = !r || above || below;
  const top = !r ? 20 : above ? r.y - 70 : r.y + r.h + 14;
  const left = r ? Math.min(Math.max(r.x + r.w / 2, 260), innerWidth - 260) : innerWidth / 2;

  const start = () => {
    if (!r) return;
    const d = devicePixelRatio, v = (x: number) => Math.round(x * d);
    setN(0); setSt("run");
    invoke("start_capture", {
      region: { x: v(r.x), y: v(r.y), w: v(r.w), h: v(r.h) },
      opts: { speed: s.speed, delay: s.delay, timeout: s.timeout, dir: s.dir, fmt: s.fmt, quality: s.quality, clip: s.clip, save: s.save },
    }).catch(e => { setMsg(String(e)); setSt("err"); });
  };

  return (
    <div className={"dim " + st + (r ? " has" : "")} onMouseDown={down} onMouseMove={move} onMouseUp={up}>
      {r && <div className="sel" style={{ left: r.x, top: r.y, width: r.w, height: r.h }} />}
      <div className="bar" style={{ top, left }}>
        <div className="logo" />
        <div className="stat">
          {st === "idle" && <><b>{r ? `Vùng ${r.w}×${r.h}` : "Kéo chuột để chọn vùng cần chụp"}</b>
            <small>{room ? "Esc để hủy" : "Chừa khoảng trống trên hoặc dưới vùng chọn"}</small></>}
          {st === "run" && <><b>Đang chụp… {n} khung</b><small>Nhấn Esc để dừng</small><div className="pg ind"><i /></div></>}
          {st === "done" && <><b>Đã lưu ảnh</b><small title={msg}>{msg}</small></>}
          {st === "err" && <><b>Không chụp được</b><small title={msg}>{msg}</small></>}
        </div>
        {st === "idle" && <button className="btn" disabled={!r || !room} onClick={start}>Bắt đầu</button>}
        {(st === "done" || st === "err") && <button className="btn" onClick={close}>Đóng</button>}
        {st !== "run" && <button className="x" onClick={close} title="Hủy (Esc)">✕</button>}
      </div>
    </div>
  );
}
