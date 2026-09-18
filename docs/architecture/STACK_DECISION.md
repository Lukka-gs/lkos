# Stack decision — ADR 001

Status: accepted for the M0 foundation; native behavior must pass the LUK-18 smoke matrix before M0 closes.
Date: 2026-09-18. Product authority: [blueprint](https://linear.app/lukka-gs/document/lkos-product-and-development-blueprint-135c9791f73e), all ten milestones and all 67 project issues, including LUK-15–18. No existing codebase was found.

## Decision

Tauri 2, Rust native host, TypeScript UI and Vite. The M0 UI uses the DOM directly: its two diagnostic controls do not justify a component framework. UI modules consume framework-independent contracts; a component library can be selected in M2 without changing native services or domain contracts. Windows x64 is the initial development target. Windows 11 is the initial validation target; broader support is not claimed before LUK-65.

## Comparison

| Requirement | Tauri 2 / Rust | Electron | WinUI 3 / .NET |
| --- | --- | --- | --- |
| Transparent, frameless, topmost | Window APIs, WebView2 compositor; validate GPU behavior | BrowserWindow supports these; transparent-window restrictions need testing | AppWindow/HWND interop; arbitrary overlay transparency needs native composition work |
| Click-through / focus | Native window API, explicit interactive/passive modes | setIgnoreMouseEvents and focus options | Win32 hit-testing and extended styles |
| Windows APIs, processes, windows, media | Rust windows crate / Win32 / WinRT behind adapters | Native addon or companion process for deeper integrations | Strong WinRT/Win32 integration and C# interop |
| Clipboard, hotkeys, tray, startup | Plugins or scoped native adapters | Mature built-in APIs; hooks still native | Windows APIs; tray/hotkeys require interop |
| Drag/drop, multi-monitor, DPI | Built-in window/drop APIs; outbound OLE and global gestures need spikes | Strong window APIs; cross-app drag and hooks still need validation | Strong native access; monitor/DPI lifecycle still application responsibility |
| Liquid Glass / animation | CSS transforms and custom rendering; desktop blur requires DWM adapter | CSS and consistent bundled Chromium; same desktop-blur caveat | Composition and system backdrops; custom Liquid Glass still bespoke |
| Background cost | No bundled Node/Chromium; Rust event-driven host; WebView2 still costs memory | Bundled Chromium/Node increases distribution/runtime baseline | Native UI is a credible low-overhead choice; actual measurements required |
| Modular collaboration | Stable TS contracts + Rust adapters; two language toolchains | Single JS/TS ecosystem, native boundary remains | C# contracts/XAML, stronger Windows coupling |
| Packaging / update | NSIS/MSI and signed updater path | Mature packaging/update ecosystem | MSIX / Windows App SDK deployment options |

The choice favors a small native service layer and flexible animated UI without shipping Node in the application. This is an engineering judgment, not a measured performance victory. Electron is the fallback if WebView2 prevents reliable overlay behavior. WinUI is the fallback if native composition, accessibility or input integration dominates the product. No fourth framework materially improves these trade-offs for this Windows backlog.

## Future requirements and boundaries

- LUK-31–37: focus, Core Audio, GSMTC media, microphone and metrics stay native, using subscriptions where available. Download/AI data requires supported sources; no promise of global download progress or quota scraping.
- LUK-39–44: clipboard and capture require user-controlled retention, exclusions and capability checks. No clipboard access in M0.
- LUK-45–49: actions are typed definitions, not arbitrary renderer-to-shell commands. Execution will require native validation, user intent and cancellation. M0 has contracts only.
- LUK-50–59: global drag/drop, OLE drag-out, DWM thumbnails, snap and reserved desktop areas need dedicated adapters/spikes; Tauri alone does not provide these product features.
- LUK-22, 63–65: physical monitor coordinates and logical UI dimensions are distinct. Hot-plug, negative coordinates, mixed DPI, fullscreen and topmost conflicts require real-device testing.
- LUK-23–26: CSS backdrop-filter does not blur applications behind a WebView. Native backdrop plus CSS highlight/reflection is a future experiment, with opaque/reduced-motion fallbacks.
- LUK-66–82: M0 builds an executable. Installer signing, update signing keys, beta channels, rollback and startup opt-in belong to M9. Never embed private signing keys.

## Native/renderer trust boundary

Bundle local UI only. CSP restricts scripts and connections. Expose narrow typed commands; no shell/filesystem/general invocation plugins. Native host owns settings and window policy. Future module permissions are enforced by the native service registry, never by renderer metadata alone. First-party modules are trusted code; the event bus is not a security sandbox. Third-party plugins would need a separate design.

## Evidence and primary references

- [Tauri window customization](https://v2.tauri.app/learn/window-customization/) and [configuration](https://v2.tauri.app/reference/config/).
- [Rust WebviewWindow API](https://docs.rs/tauri/latest/tauri/webview/struct.WebviewWindow.html): ignore cursor events, focus, monitor and topmost controls.
- [Tauri tray](https://v2.tauri.app/learn/system-tray/), [security](https://v2.tauri.app/security/), [Windows packaging](https://v2.tauri.app/distribute/windows-installer/).
- [Electron BrowserWindow](https://www.electronjs.org/docs/latest/api/browser-window).
- [Microsoft Windows App SDK windowing](https://learn.microsoft.com/en-us/windows/apps/develop/ui/windowing-overview).

See `docs/development/VALIDATION.md` for actual test results and remaining evidence. API availability is not proof of correct behavior over other Windows applications.
