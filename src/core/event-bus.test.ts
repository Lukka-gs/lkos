import { describe, expect, it, vi } from "vitest";
import { createEventBus } from "./event-bus";
const event = {
  id: "1",
  type: "fixture",
  source: "test",
  timestamp: 0,
  priority: 0,
  payload: { value: 1 },
};
describe("event bus", () => {
  it("routes by type and stops delivery after idempotent unsubscribe", () => {
    const bus = createEventBus(vi.fn());
    const handler = vi.fn();
    const other = vi.fn();
    const dispose = bus.subscribe("fixture", handler);
    bus.subscribe("other", other);
    bus.publish(event);
    dispose();
    dispose();
    bus.publish(event);
    expect(handler).toHaveBeenCalledTimes(1);
    expect(other).not.toHaveBeenCalled();
  });
  it("isolates faulty subscribers and mutable payloads", () => {
    const errors = vi.fn();
    const bus = createEventBus(errors);
    bus.subscribe("fixture", (e) => {
      if (
        e.payload &&
        typeof e.payload === "object" &&
        !Array.isArray(e.payload)
      )
        e.payload.value = 99;
      throw Error("module failed");
    });
    const handler = vi.fn();
    bus.subscribe("fixture", handler);
    bus.publish(event);
    expect(errors).toHaveBeenCalledTimes(1);
    expect(handler).toHaveBeenCalledWith(event);
    expect(event.payload.value).toBe(1);
  });
});
