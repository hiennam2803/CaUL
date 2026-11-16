import { useEffect, useState } from "react";

export const DEF = {
  autostart: false, background: true, tray: true,
  speed: 5, delay: 300, timeout: 30,
  startKey: "Ctrl+Shift+S",
  dir: "~/Pictures/CaUL", fmt: "PNG", quality: 90,
  theme: "system",
};
export type S = typeof DEF;

export function useSettings() {
  const [s, set] = useState<S>(() => {
    try { return { ...DEF, ...JSON.parse(localStorage.getItem("caul") || "{}") }; } catch { return DEF; }
  });
  useEffect(() => {
    localStorage.setItem("caul", JSON.stringify(s));
    const mq = matchMedia("(prefers-color-scheme: dark)");
    const apply = () => (document.documentElement.dataset.theme = s.theme === "system" ? (mq.matches ? "dark" : "light") : s.theme);
    apply();
    mq.addEventListener("change", apply);
    return () => mq.removeEventListener("change", apply);
  }, [s]);
  return [s, (p: Partial<S>) => set(v => ({ ...v, ...p }))] as const;
}
