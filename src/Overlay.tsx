import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useSettings } from "./store";

type St = "idle" | "run" | "stop" | "done";
const TXT = { idle: "Sẵn sàng chụp cuộn", run: "Đang chụp…", stop: "Đã dừng", done: "Hoàn tất (mô phỏng)" };

export default function Overlay() {
  const [s] = useSettings();
  const [st, setSt] = useState<St>("idle");
  const [pct, setPct] = useState(0);
  const close = () => invoke("close_overlay").catch(() => window.close());

  useEffect(() => {
    const k = (e: KeyboardEvent) => e.key === "Escape" && (st === "run" ? setSt("stop") : close());
    addEventListener("keydown", k);
    return () => removeEventListener("keydown", k);
  }, [st]);

  useEffect(() => {
    if (st !== "run") return;
    const t = setInterval(() => setPct(p => {
      if (p >= 100) { setSt("done"); return 100; }
      return Math.min(100, p + s.speed * 1.5);
    }), s.delay);
    return () => clearInterval(t);
  }, [st, s.speed, s.delay]);

  const start = () => { setPct(0); setSt("run"); };

  return (
    <div className={"dim " + st}>
      <div className="frame" />
      <div className="bar">
        <div className="logo">C</div>
        <div className="stat"><b>{TXT[st]}</b>
          {st === "run" && <div className="pg"><i style={{ width: pct + "%" }} /></div>}
          {st === "idle" && <small>Bấm Bắt đầu hoặc {s.startKey}</small>}
        </div>
        {st === "run"
          ? <button className="btn" onClick={() => setSt("stop")}>Dừng</button>
          : <button className="btn" onClick={st === "done" ? close : start}>{st === "done" ? "Đóng" : "Bắt đầu"}</button>}
        <button className="x" onClick={close} title="Hủy (Esc)">✕</button>
      </div>
    </div>
  );
}
