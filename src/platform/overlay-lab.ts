import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
export type LabAction =
  | "place-target"
  | "passive"
  | "restore"
  | "menu"
  | "default-position";
export const lab = {
  action: (action: LabAction) => invoke<void>("lab_action", { action }),
  snapshot: () => invoke<Record<string, unknown>>("lab_snapshot"),
  observe: (handler: (message: string) => void) =>
    listen<string>("overlay-observation", ({ payload }) => handler(payload)),
};
