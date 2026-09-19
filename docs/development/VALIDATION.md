# Foundation validation

Validation date: 2026-09-19. Host: Windows build 26200, x64; Node 24.15.0, npm 11.12.1, Rust 1.97.1. The development target and release executable were run locally.

## Automated gates

- Biome formatting/lint, import boundaries, rustfmt and Clippy with warnings denied.
- TypeScript strict check and Rust check.
- Five Vitest tests: event routing/unsubscribe, subscriber failure and payload isolation, independent module lifecycle, duplicate/permission rejection, failed startup cleanup/retry.
- Four Rust tests: restart persistence, v0 migration, corrupt/future/invalid schema preservation, failed save without in-memory commit.
- Release build with locked dependencies, executable around 7.4 MiB (size is not a RAM measurement).

The first sandboxed Vitest attempt could not spawn workers (EPERM). Running the same tests outside the sandbox passed; no test was disabled. Initial compile exposed the required Windows icon, which was added. Vite now ignores native build output to avoid unintended UI reloads.

## Observed desktop behavior

| Check | Evidence / result |
| --- | --- |
| Single development command | `npm run dev` started Vite + Cargo and opened the notch |
| Native renderer | Computer Use displayed a compact frameless notch with transparent exterior margins |
| IPC + event bus | Clicking ↗ changed status to `Evento nativo recebido · 1` |
| Durable write | Native settings file contained schema 1 and probeCount 1 |
| Exit control | Clicking × ended the app; no LKOS process remained |
| Release without dev server | Release executable opened and displayed `Pronto · 1 testes salvos` |
| Restart persistence | Counter restored from the previous development process |
| Tray construction | Native tray/menu creation succeeded during startup; interactive menu checks remain below |

Computer Use did not expose the Windows taskbar as a targetable window in this session, so tray menu actions and real click-through delivery to another application are not claimed as verified. LUK-18 remains open.

## Remaining native spike matrix (LUK-18)

1. Use tray to toggle topmost, focus another ordinary app, verify stacking, restart and confirm preference.
2. Put a harmless target button in another app beneath the notch. Select tray click-through and verify the underlying button receives input. Restore through tray and verify the notch receives input again.
3. Verify startup/restore do not steal focus; test keyboard navigation and explicit exit through tray.
4. Test 100%, 125%, 150% and 200% DPI, including mixed-DPI monitors and negative coordinates. Record screen setup and actual result.
5. Check ordinary apps, maximized windows, GPU rendering variations and taskbar arrangements. Record transparent-edge artifacts if any.

Full monitor reconnection/position persistence is LUK-22; fullscreen/games and accessibility matrix are LUK-63–65. No idle CPU/RAM target is declared passed. No installer, updater, production startup integration or signing was tested.

## Collaboration blockers

GitHub refused private main branch protection with HTTP 403, requiring Pro or public visibility. Visibility is unchanged. Main holds only the bootstrap; foundation changes are proposed by PR. CI results and review must be checked on that PR before merge. M0 cannot close until LUK-18 evidence is completed; repository protection also remains an environment-delivery blocker.
