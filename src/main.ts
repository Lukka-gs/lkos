import { bootstrap } from "./shell/bootstrap";
import "./ui/style.css";
const root = document.querySelector<HTMLElement>("#app");
if (!root) throw new Error("Missing app root");
void bootstrap(root);
