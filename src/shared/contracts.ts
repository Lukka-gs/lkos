/** Declarative contracts only. Future engines are deliberately not implemented. */
export type ModuleId = string;
export type Permission =
  | "window.read"
  | "media.read"
  | "clipboard.read"
  | "clipboard.write"
  | "keyboard.observe"
  | "process.launch"
  | "script.execute"
  | "filesystem.read";
export type Json =
  | null
  | boolean
  | number
  | string
  | Json[]
  | { [key: string]: Json };
export interface EventEnvelope<T = Json> {
  readonly id: string;
  readonly type: string;
  readonly source: ModuleId;
  readonly timestamp: number;
  readonly priority: number;
  readonly payload: T;
  readonly ttlMs?: number;
  readonly deduplicationKey?: string;
  readonly correlationId?: string;
}
export type Dispose = () => void;
export type ModuleState = "idle" | "loading" | "ready" | "empty" | "error";
export interface EventBus {
  publish(event: EventEnvelope): void;
  subscribe(type: string, handler: (event: EventEnvelope) => void): Dispose;
}
export interface SettingsPort {
  read(module: ModuleId): Promise<Readonly<Record<string, Json>>>;
  write(module: ModuleId, value: Record<string, Json>): Promise<void>;
}
export interface ShellPort {
  requestAttention(module: ModuleId): void;
  requestLayout(module: ModuleId, width: number, height: number): void;
}
export interface ModuleContext {
  readonly events: EventBus;
  readonly settings: SettingsPort;
  readonly shell: ShellPort;
  readonly signal: AbortSignal;
  /** Informational mirror; authorization must also be enforced natively. */
  readonly permissions: ReadonlySet<Permission>;
}
export interface ModuleManifest {
  readonly id: ModuleId;
  readonly version: string;
  readonly settingsVersion: number;
  readonly permissions: readonly Permission[];
}
export interface ModuleLifecycle {
  start(context: ModuleContext): Promise<void>;
  suspend(): Promise<void>;
  resume(): Promise<void>;
  /** Must release subscriptions, timers and native handles. Idempotent. */
  dispose(): Promise<void>;
}
export interface Gadget extends ModuleLifecycle {
  readonly kind: "gadget";
  readonly manifest: ModuleManifest;
  readonly state: ModuleState;
  readonly presentation: { readonly label: string; readonly icon: string };
}
export interface Tool extends ModuleLifecycle {
  readonly kind: "tool";
  readonly manifest: ModuleManifest;
  readonly state: ModuleState;
  open(): Promise<void>;
  close(): Promise<void>;
}
export interface Action {
  readonly id: string;
  readonly kind:
    | "open-app"
    | "open-file"
    | "open-folder"
    | "open-url"
    | "shortcut"
    | "command"
    | "powershell";
  readonly parameters: Readonly<Record<string, Json>>;
  readonly permissions: readonly Permission[];
  readonly requiresConfirmation: boolean;
  readonly timeoutMs: number;
}
export interface Trigger {
  readonly id: string;
  readonly enabled: boolean;
  readonly eventType: string;
  readonly conditions: Readonly<Record<string, Json>>;
  readonly effect: {
    readonly kind: "show-gadget" | "run-action" | "activate-workspace";
    readonly targetId: string;
  };
  readonly cooldownMs: number;
}
export interface Workspace {
  readonly id: string;
  readonly steps: readonly {
    readonly actionId: string;
    readonly delayMs: number;
    readonly onError: "stop" | "continue";
  }[];
  readonly layout: {
    readonly baseGadgetId: string;
    readonly gadgetIds: readonly string[];
    readonly toolIds: readonly string[];
  };
}
export interface PlatformAdapter extends ModuleLifecycle {
  readonly id: string;
  readonly capabilities: readonly Permission[];
  isAvailable(): Promise<boolean>;
}
