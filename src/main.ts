/// <reference types="vite/client" />
import { bootstrap } from "./shell/bootstrap";
import "./ui/style.css";
const root = document.querySelector<HTMLElement>("#app");
if (!root) throw new Error("Missing app root");
if (import.meta.env.DEV && location.hash === "#overlay-lab") {
  void import("./shell/overlay-lab").then(({ startLab }) => startLab(root));
} else {
  void bootstrap(root);
}
