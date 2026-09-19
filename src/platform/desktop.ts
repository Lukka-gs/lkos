import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { EventBus, EventEnvelope } from "../shared/contracts";
import type { Settings } from "../settings/types";

export const desktop = {
  settings: () => invoke<Settings>("read_settings"),
  probe: () => invoke<Settings>("run_probe"),
  quit: () => invoke<void>("quit"),
  connect: (bus: EventBus) =>
    listen<EventEnvelope>("foundation-probe", ({ payload }) =>
      bus.publish(payload),
    ),
};
