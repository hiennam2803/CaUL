import { createRoot } from "react-dom/client";
import App from "./App";
import Overlay from "./Overlay";
import "./styles.css";

const overlay = location.hash === "#overlay";
document.body.classList.toggle("ov", overlay);
createRoot(document.getElementById("root")!).render(overlay ? <Overlay /> : <App />);
