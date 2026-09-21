# Foundation validation

Latest validation: 2026-09-21, Windows x64, Node 24.15.0, npm 11.12.1, Rust 1.97.1. Checkout: `C:\Main\lkos`.

## Automated gates

Formatting (Biome/rustfmt), lint (Biome/Clippy), module boundaries, TypeScript and Cargo checks passed. Five Vitest tests cover event routing, unsubscribe, subscriber/payload isolation and module lifecycle failure/retry. Seven Rust tests cover four persistence scenarios and three placement scenarios.

Placement tests exercise 100%, 125%, 150% and 200% scale, negative coordinates, taskbar offsets, small work areas and invalid display data. These are coordinate calculations, not physical mixed-DPI testing.

The Windows release build passed. The production frontend contains no lab UI or lab command references. Native lab commands/window creation are compiled only with debug assertions. Packaging, signing and updates remain M9.

Moving the existing Cargo cache left absolute references to the old checkout. Regenerating the Tauri package cache for both debug and release resolved the build failure. No source or settings were deleted.

## Observed desktop behavior

2026-09-21 used `npm run dev:overlay`, one monitor at scale 1, work area 2560 x 1392, origin (0,0). The lab is a separate window in the same process, with separate fixture settings.

| Check | Evidence / result |
| --- | --- |
| Startup | One command opened the notch and diagnostic lab |
| Transparent frameless notch | Target window remained visible through exterior margins |
| IPC | Clicking notch arrow changed native fixture count 0 -> 1 |
| Passive input | With notch over target and passive=true, physical click at the arrow position incremented underlying target 0 -> 1; native count remained 1 |
| Restore input | Restore followed by arrow click incremented native count 1 -> 2; target stayed 1 |
| Native menu | Lab popup uses the same menu and callback as tray; topmost changed true -> false with event `menu:top ok` |
| Persistence | Lab settings contained schemaVersion 1, alwaysOnTop false, probeCount 2 |
| Native menu exit | Choosing Encerrar LKOS closed the process |
| Focus observations | Passive diagnostic showed notchFocused=false and labFocused=true; automation also activates its target, so this is not a full focus-stealing proof |

The actual taskbar tray icon was not clicked: the computer-use interface did not expose the taskbar as a target. Popup validation proves the shared menu path, not tray discoverability. Cross-process click-through and focus behavior in other apps remain unverified.

Previous 2026-09-19 checks also observed the normal development notch, durable settings write, explicit close, release startup without the dev server and restored fixture count after restart.

## Remaining native spike matrix (LUK-18)

1. Access the actual tray icon; restore input and exit from there.
2. Verify click-through against a harmless button in another application, then restore notch input.
3. Verify topmost stacking over ordinary/maximized apps and persisted preference after restart.
4. Verify startup/restore do not steal focus, including keyboard navigation.
5. Exercise physical 100%, 125%, 150% and 200% DPI, mixed-DPI monitors and negative coordinates. Record actual results and transparency artifacts.

LUK-18 and M0 remain in progress. Full monitor reconnection/position persistence is LUK-22; fullscreen/games and accessibility matrix are LUK-63–65. No idle CPU/RAM target, installer, updater or startup integration is declared passed.

## Collaboration status

PR #1 merged the foundation. The repository is public and main protection is disabled by the owner for solo development. Work continues on issue branches with PRs and Windows CI. Local test results do not imply a future remote CI run has passed.

## LUK-19 process validation — 2026-09-21

The single-instance implementation passed `npm run check` (format, lint, boundaries, typecheck, 12 existing unit tests and Windows release build). The explicit local `npm run test:lifecycle` also passed:

- Primary release process created a native window.
- Three consecutive duplicate launches exited with code 0; the primary remained alive.
- Settings SHA-256 remained unchanged across those launches.
- After a forced termination of the owned primary, a new process created its window and retained the exact settings hash.
- The test cleaned up the processes it started.

The first smoke attempt failed because this PowerShell did not expose Get-FileHash; the script now uses .NET SHA-256 and the full smoke test passed on retry. This is process/persistence evidence, not a visual focus test, automatic restart supervisor, renderer-crash recovery, simultaneous cold-start stress test or damaged-settings recovery. LUK-19 remains in progress; physical LUK-18 checks are still required before M0/M1 acceptance.
