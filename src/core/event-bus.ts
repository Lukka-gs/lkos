import type { EventBus, EventEnvelope } from "../shared/contracts";

/** Synchronous local delivery. TTL, scheduling and deduplication belong to M3. */
export function createEventBus(onError: (error: unknown) => void): EventBus {
  const listeners = new Map<string, Set<(event: EventEnvelope) => void>>();
  return {
    subscribe(type, handler) {
      const bucket = listeners.get(type) ?? new Set();
      bucket.add(handler);
      listeners.set(type, bucket);
      return () => {
        bucket.delete(handler);
        if (!bucket.size) listeners.delete(type);
      };
    },
    publish(event) {
      for (const handler of [...(listeners.get(event.type) ?? [])]) {
        try {
          handler(structuredClone(event));
        } catch (error) {
          onError(error);
        }
      }
    },
  };
}
