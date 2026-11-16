import { useState, ReactNode } from "react";

export const Row = ({ t, d, children }: { t: string; d?: string; children: ReactNode }) => (
  <div className="row"><div><b>{t}</b>{d && <small>{d}</small>}</div>{children}</div>
);
export const Toggle = ({ v, on }: { v: boolean; on: (v: boolean) => void }) => (
  <button className={"tg" + (v ? " on" : "")} role="switch" aria-checked={v} onClick={() => on(!v)} />
);
export const Range = ({ v, min, max, unit, on }: { v: number; min: number; max: number; unit: string; on: (v: number) => void }) => (
  <div className="rg"><input type="range" min={min} max={max} value={v} onChange={e => on(+e.target.value)} /><span>{v}{unit}</span></div>
);
export const Seg = ({ v, opts, on }: { v: string; opts: [string, string][]; on: (v: string) => void }) => (
  <div className="seg">{opts.map(([k, l]) => <button key={k} className={k === v ? "on" : ""} onClick={() => on(k)}>{l}</button>)}</div>
);
const MODS = ["Control", "Shift", "Alt", "Meta"];
export function KeyBox({ v, on }: { v: string; on: (v: string) => void }) {
  const [rec, setRec] = useState(false);
  const down = (e: React.KeyboardEvent) => {
    if (!rec) return;
    e.preventDefault();
    if (MODS.includes(e.key)) return;
    if (e.key !== "Escape") {
      const m = [e.ctrlKey && "Ctrl", e.altKey && "Alt", e.shiftKey && "Shift", e.metaKey && "Meta"].filter(Boolean);
      on([...m, e.key.length === 1 ? e.key.toUpperCase() : e.key].join("+"));
    }
    setRec(false);
  };
  return <button className={"key" + (rec ? " rec" : "")} onClick={() => setRec(true)} onBlur={() => setRec(false)} onKeyDown={down}>{rec ? "Nhấn tổ hợp phím…" : v}</button>;
}
