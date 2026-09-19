# Consolidated functional specification

Authority: [LKOS blueprint](https://linear.app/lukka-gs/document/lkos-product-and-development-blueprint-135c9791f73e) and [project](https://linear.app/lukka-gs/project/lkos-98d7b40f689f), read on 2026-09-18, including every milestone M0–M9 and all 67 issues. This document consolidates requirements, not a claim that they are implemented.

## Product

LKOS is a local Windows application that places an adaptive, customizable notch over the desktop. No account, hosted service or mandatory backend. It should remain discreet, useful while collapsed and inexpensive when inactive. Native integrations use supported local signals or official APIs; unavailable information is reported as unavailable rather than inferred through fragile scraping.

## States and interaction

- **Closed:** one user-selected base gadget, initially Focus Dot. Contextual events may add temporary gadgets alongside it. Examples: Focus Dot + download; Now Playing + supported AI activity. Multiple events respect priority, width and overflow policy.
- **Expanded:** clicking the notch opens an animated panel with rounded icon tiles and labels when space allows. Initial installation has three tools; users may add, remove, reorder and customize them. Excess tools require explicit overflow/pagination rather than silent clipping. Clicking outside or closing returns to the collapsed state.
- **Passive:** where configured, pointer input passes through the overlay. Users must retain a reliable route back to interactive mode. M0 demonstrates a whole-window tray toggle; production hit testing is not yet specified.
- **Inactive/suspended:** disabled modules release their timers/hooks. Fullscreen/game policy is validated in M8 and must not assume overlays can draw over exclusive fullscreen.
- **Module loading/empty/error:** each module exposes explicit state. One module failure must not stop unrelated subscribers or corrupt settings.

The default location is near the upper-right region with breathing room around window controls. Position should eventually support free movement/snap, monitors, persisted coordinates and recovery after display removal. The foundation uses a conservative offset, not the final positioning engine.

## Personalization

Position, dimensions, corner radius, opacity, blur, tint, highlights/reflection, shadow, glass intensity and motion. Gadget base/contextual participation, order, size, pin/hidden state and icons. Tool count, order, tile composition and custom icons. Actions, triggers, combos and workspace layouts. Changes persist and have safe restore behavior. Motion must support reduced motion; visuals require accessibility labels, keyboard focus and contrast.

## Module catalog and roadmap

| Milestone | Product scope | Issues |
| --- | --- | --- |
| M0 | Functional spec, stack, contracts, Windows overlay spike and risks | LUK-5, 15–18 |
| M1 | Process lifecycle, tray, startup, singleton/recovery, schema, position/snap, monitor/DPI changes | LUK-6, 19–22 |
| M2 | Glass material/fallback, transitions/morph, appearance editor, layout/icons | LUK-7, 23–26 |
| M3 | Event/context model, priorities, base + temporary scheduler, TTL/overflow/resize, triggers | LUK-8, 27–30 |
| M4 | Focus Dot, Volume, Now Playing, Mic, System, Download, sustainable AI integrations | LUK-9, 31–37 |
| M5 | Expanded panel, Clipboard Visual, Download Shelf, Quick Notes, Screenshot Shelf, Link Catcher, Color Catcher | LUK-10, 38–44 |
| M6 | Actions/editor, combo sequence engine, workspaces/layouts, workspace editor | LUK-11, 45–49 |
| M7 | Desktop Dock, Drag Preview, Screen Ruler, Window Snap Preview, Desktop Spacer, Cursor Halo, KeyCast, Drop Zone, Window Peek, Desktop Timeline | LUK-12, 50–59 |
| M8 | Automated tests, performance budgets, security, fullscreen, accessibility, supported device matrix | LUK-13, 60–65 |
| M9 | Installer/uninstall, updates, onboarding/defaults, beta/feedback, release checklist | LUK-14, 66–69, 81–82 |

### Gadgets

Focus Dot shows active application/window identity and continuous focus duration; idle, pause and privacy need policy. Volume reflects and optionally controls system volume. Now Playing uses Windows media sessions, handles multiple/paused sessions and supported artwork/progress/controls. Mic indicates activity and app attribution only when reliable. System shows selected CPU/RAM/network/battery/storage metrics. Download progress needs a trustworthy source; browser integration is a spike. AI activity/usage similarly requires supported data and no aggressive polling.

### Tools

Clipboard history supports local text/images/links/colors with limits, exclusions and cleanup. Download and Screenshot Shelves expose recent supported items, opening, copying/dragging and source locations. Quick Notes stays a simple local autosaving note tool. Link Catcher recognizes copied URLs; QR is conditional scope. Color Catcher samples screen color and keeps a short history.

Advanced tools are independent: temporary file references, contextual file drag operations, measurements, snap previews, reserved desktop areas, cursor highlighting, optional key display with sensitive exclusions, edge drop actions, window thumbnails and a privacy-conscious local timeline. No global tracking beyond supported sources is promised.

### Actions, combos, workspaces and triggers

Actions open apps/files/folders/URLs, send shortcuts or execute explicitly configured commands/PowerShell. Editors expose names/icons/parameters and confirmation policy. Combos reference ordered actions, delays and stop/continue failure behavior. Workspaces add contextual layouts and gadget sets. Triggers map supported typed events and conditions to referenced gadgets/actions/workspaces; each rule can be disabled. These definitions never imply automatic execution of external text.

## Safety and data

Native services validate inputs and enforce capabilities. Sensitive integrations need clear opt-in, retention, exclusion and disabling controls. Plain settings contain no secrets. User files are referenced until an explicit action requests copying/moving. Uninstall must not erase user data without consent. Diagnostics must omit sensitive data and feedback is voluntary.

## Foundation scope approved for this task

Executable notch, tray, explicit exit, IPC fixture, local versioned settings/migration scaffold, basic internal bus, contracts and registry tests, reproducible commands, CI and collaboration documentation. No Focus Dot, Spotify, downloads, clipboard, action execution, workspaces, adaptive scheduler or final Liquid Glass implementation.

## Decisions still reserved for later milestones

The exact three initial tools (M9), detailed overflow/priorities (M3), supported Windows/device matrix (M8), performance budgets (M8), sources for download/AI metrics (M4), final glass material (M2), distribution/signing/update keys (M9). These are explicit future product work, not hidden prerequisites for the M0 contracts.

## Definition of done

Functional work must be implemented, integrated through contracts, configurable where applicable, have loading/empty/error states, be tested, avoid perceptible performance regressions and be documented. M0 additionally requires recorded native overlay evidence; compilation alone does not satisfy LUK-18.
