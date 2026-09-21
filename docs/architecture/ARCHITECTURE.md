# Architecture

## Boundaries

The native Rust process owns durable settings, Windows handles and lifecycle. The WebView renders local assets and consumes a narrow IPC interface. Core is framework-independent TypeScript. Shell is the composition root, the only layer allowed to assemble concrete modules. No backend, account or remote UI is required.

```text
UI → Shared contracts
Shell → Core + UI + Platform + registered modules
Gadgets / Tools → Shared contracts + injected context
Actions / Triggers / Workspaces → Shared declarative contracts
Platform → native IPC → Rust Shell → Settings / future Windows adapters
Core → Shared
```

`scripts/check-boundaries.mjs` parses TypeScript imports and exports in CI. Gadgets/tools cannot import each other or the Tauri API. Each future module should live in its own subdirectory; shared behavior belongs in Shared or injected services. No dynamic module loader or untrusted plugin execution is supported.

## Directory map

```text
src/
  main.ts                 application entry
  core/                   local event bus, module registry, tests
  shell/                  composition and lifecycle wiring
  ui/                     minimal notch rendering and styles
  gadgets/ tools/         reserved first-party module boundaries
  actions/ triggers/ workspaces/  contracts-only future domains
  platform/               typed native bridge
  settings/               renderer settings DTO
  shared/                 six public contracts and common types
src-tauri/
  capabilities/           minimum renderer event permissions
  src/shell.rs            native commands, window and tray
  src/settings.rs         versioned persistence, migrations, tests
scripts/                  architecture checks
.github/workflows/        Windows PR gates
docs/architecture/        product, stack, contracts and risks
docs/development/         setup, workflow and validation
```

## Contracts

Canonical interfaces: `src/shared/contracts.ts`.

| Contract | Responsibility |
| --- | --- |
| Gadget | Manifest, state, presentation and start/suspend/resume/dispose |
| Tool | Manifest, state, open/close and the same lifecycle |
| Action | Declarative kind, parameters, permissions, timeout and confirmation intent |
| Trigger | Event type, conditions, enabled state, cooldown and referenced effect |
| Workspace | Ordered action IDs with delays/error policy plus referenced layout |
| PlatformAdapter | Availability, supported capabilities and resource lifecycle |

Context injects scoped settings, events, shell requests, granted permissions and an AbortSignal. Registry rejects duplicate IDs and missing permissions, aborts before disposal and cleans failed starts. It is a trusted first-party lifecycle scaffold, not the adaptive engine. Native authorization remains mandatory for future sensitive services; renderer permission checks are not a security boundary. Async start must settle before a caller starts/stops that same module; concurrent lifecycle transitions are not supported by this M0 registry.

## Event bus and IPC

Events have ID, type, source, timestamp, priority, payload and optional TTL, deduplication key and correlation ID. Delivery is synchronous in-process and payloads are cloned per subscriber. One failed subscriber is reported without stopping others. Subscription returns an idempotent disposer. Production scheduling, cancellation by event ID, TTL enforcement and deduplication are M3 work (LUK-27–30).

M0 IPC exposes only `read_settings`, `run_probe`, `quit`. Build-time app command permissions are granted only to the local `notch` window capability. The fixture saves its counter before emitting `foundation-probe` to the notch. Shell subscribes before enabling the button and forwards the native envelope onto the internal bus. No renderer filesystem, process or shell-execution API is granted. Future windows/remote content require an explicit command authorization review.

## Settings

Native single writer under a mutex. Path: Tauri app config directory for `io.github.lukka-gs.lkos`, file `settings.json` (Windows: under `%APPDATA%`). Schema 1 stores only `alwaysOnTop` and the diagnostic `probeCount`. Module/action/workspace settings are defined at their milestones rather than inventing defaults now.

Migration pipeline upgrades the fixture schema 0 to 1, retaining topmost preference. Unknown future versions, malformed JSON and invalid field types fail without overwriting the file. A same-directory temporary file is flushed then atomically replaces the destination; memory commits only after success. Save failures propagate. Startup currently fails closed on corrupt settings; a user-facing recovery screen is M1 work. Back up the file before manual recovery. Secrets must never be added to this plain JSON store.

## Shell and Windows

Single compact transparent frameless window, topmost preference, tray and explicit exit. Placement uses the current monitor (primary fallback), converts logical dimensions to physical pixels and clamps both axes and dimensions to its work area. Scale-factor changes recalculate placement. `focus:false` avoids requesting focus at startup; interaction can focus the UI. Tray selects whole-window passive click-through or restores interaction without requesting focus. Passive mode also disables window focusability. Native overlay operations live in overlay.rs, pure geometry in placement.rs. A debug-only lab shares these handlers and uses separate fixture settings. Per-pixel interactive regions, focus-safe expansion, automatic crash recovery, hot-plug and per-monitor persisted positions remain M1/LUK-18 validation work.

Future Windows APIs live behind Rust adapters: Win32 window/process hooks, WinRT media, Core Audio, OLE drop, DWM thumbnails/backdrops. Each owns cancellation and must release hooks/subscriptions when inactive. UI gets normalized DTOs rather than HWNDs or arbitrary paths to execute.

## Distribution

CI builds the Windows executable with locked dependencies. It does not sign, publish or install the app. M9 will configure installer, signing, startup opt-in, signed updates, channels and migration/rollback policy. Main receives changes through reviewed PRs only.

LUK-19 singleton: the native single-instance plugin registers before setup and owns duplicate-launch arbitration. A later launch restores the current notch through the shared overlay adapter without invoking modules, parsing external commands or reopening settings. Debug and release share one identity. See SETUP.md for the native lifecycle smoke test.
