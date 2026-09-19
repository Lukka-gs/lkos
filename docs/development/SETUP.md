# Development setup (Windows x64)

## One-time prerequisites

- Node.js **24.15.0**, npm **11.12.1** (`npm install -g npm@11.12.1` if needed).
- Rust via rustup. `rust-toolchain.toml` selects **1.97.1** with rustfmt/clippy automatically.
- Visual Studio 2022 Build Tools: **Desktop development with C++**, MSVC x64 toolset and Windows SDK.
- Microsoft Edge WebView2 Runtime.
- Git. No .NET SDK or backend service is required.

See [Tauri Windows prerequisites](https://v2.tauri.app/start/prerequisites/) for installer details. First build downloads the pinned Rust toolchain and Cargo dependencies. Later builds use the lockfiles and caches.

## Clone and run

The foundation is on `chore/m0-foundation` until its PR is merged:

```powershell
git clone https://github.com/Lukka-gs/lkos.git
cd lkos
git switch chore/m0-foundation
npm ci
npm run dev
```

After merge, use main as the starting point. The repository is public. The local checkout on the owner workstation is `C:\Main\lkos`. `npm run dev` starts Vite on `127.0.0.1:1435`, compiles the Rust host and opens the notch. There is no second server command. Keep only one LKOS instance running; singleton belongs to LUK-19.

Click ↗ to persist a fixture counter and receive a native event. Restart to see the saved count. The tray offers restore interaction, passive click-through, topmost toggle and exit. The × button terminates the app. Stop the development watcher with Ctrl+C after exit if it remains active.

## Commands

| Command | Purpose |
| --- | --- |
| `npm run dev` | Full desktop development |
| `npm run build` | Production UI and Windows executable, no installer |
| `npm run lint` | Biome, import boundaries, Rust Clippy |
| `npm run format` | Format JS/TS/JSON/CSS and Rust |
| `npm run format:check` | Read-only formatter check |
| `npm run typecheck` | TypeScript and Rust checks |
| `npm test` | Vitest + Rust persistence tests |
| `npm run check` | All PR checks and release executable build |

Executable: `src-tauri/target/release/lkos.exe`. A browser-only Vite preview is not a functioning desktop app and has no mocked persistence bridge.

## Settings and troubleshooting

Settings: `%APPDATA%\io.github.lukka-gs.lkos\settings.json`. Do not put real user data in fixture events. Corrupt or future-version settings produce startup failure and preserve the file. Back it up and inspect the error from `npm run dev`; do not silently reset user data.

- Missing linker/SDK: install the C++ workload, reopen the shell.
- Blank webview: check WebView2 Runtime and the dev server output.
- Port 1435 busy: close the other dev server; strictPort prevents connecting to the wrong app.
- Passive overlay: use the LKOS tray menu to restore input.
- Missing Rust components: `rustup show`, then retry the command.
- CI errors must be fixed or explicitly documented; never treat a failed runner as a passed gate.
