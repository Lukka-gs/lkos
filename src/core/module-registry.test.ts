import { expect, it, vi } from "vitest";
import type { Gadget, ModuleContext } from "../shared/contracts";
import { createEventBus } from "./event-bus";
import { ModuleRegistry } from "./module-registry";

function fixture(id: string): Gadget {
  return {
    kind: "gadget",
    manifest: { id, version: "1", settingsVersion: 1, permissions: [] },
    state: "idle",
    presentation: { label: id, icon: "fixture" },
    start: vi.fn(async () => {}),
    suspend: vi.fn(async () => {}),
    resume: vi.fn(async () => {}),
    dispose: vi.fn(async () => {}),
  };
}
function context(): Omit<ModuleContext, "signal"> {
  return {
    events: createEventBus(vi.fn()),
    settings: { read: async () => ({}), write: async () => {} },
    shell: { requestAttention: vi.fn(), requestLayout: vi.fn() },
    permissions: new Set(),
  };
}
it("accepts independent module fixtures without core changes and releases resources", async () => {
  const registry = new ModuleRegistry();
  const a = fixture("a");
  const b = fixture("b");
  registry.register(a);
  registry.register(b);
  await registry.start("a", context());
  await registry.start("b", context());
  const started = vi.mocked(a.start).mock.calls[0]?.[0];
  expect(started?.signal.aborted).toBe(false);
  await registry.stop("a");
  await registry.stop("a");
  expect(started?.signal.aborted).toBe(true);
  expect(a.dispose).toHaveBeenCalledTimes(1);
  expect(b.dispose).not.toHaveBeenCalled();
  await registry.stop("b");
});
it("rejects duplicate IDs and missing permissions before starting", async () => {
  const registry = new ModuleRegistry();
  const a = fixture("a");
  registry.register(a);
  expect(() => registry.register(a)).toThrow("Duplicate");
  const restricted = {
    ...fixture("restricted"),
    manifest: {
      id: "restricted",
      version: "1",
      settingsVersion: 1,
      permissions: ["clipboard.read" as const],
    },
  };
  registry.register(restricted);
  await expect(registry.start("restricted", context())).rejects.toThrow(
    "Missing permission",
  );
  expect(restricted.start).not.toHaveBeenCalled();
});
it("cleans up a failed start and allows retry", async () => {
  const registry = new ModuleRegistry();
  const a = fixture("a");
  vi.mocked(a.start).mockRejectedValueOnce(new Error("fixture failure"));
  registry.register(a);
  await expect(registry.start("a", context())).rejects.toThrow(
    "fixture failure",
  );
  expect(a.dispose).toHaveBeenCalledOnce();
  await expect(registry.start("a", context())).resolves.toBeUndefined();
  await registry.stop("a");
});
