# Development setup (Windows x64)

## One-time prerequisites

- Node.js **24.15.0**, npm **11.12.1** (`npm install -g npm@11.12.1` if needed).
- Rust via rustup. `rust-toolchain.toml` selects **1.97.1** with rustfmt/clippy automatically.
- Visual Studio 2022 Build Tools: **Desktop development with C++**, MSVC x64 toolset and Windows SDK.
- Microsoft Edge WebView2 Runtime.
- Git. No .NET SDK or backend service is required.

See [Tauri Windows prerequisites](https://v2.tauri.app/start/prerequisites/) for installer details. First build downloads the pinned Rust toolchain and Cargo dependencies. Later builds use the lockfiles and caches.

## Clone and run

The foundation is merged into main:

```powershell
git clone https://github.com/Lukka-gs/lkos.git
cd lkos
npm ci
npm run dev
```

Use main as the starting point. The repository is public. The local checkout on the owner workstation is `C:\Main\lkos`. `npm run dev` starts Vite on `127.0.0.1:5187`, compiles the Rust host and opens the notch. There is no second server command. Keep only one LKOS instance running; singleton belongs to LUK-19.

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
- Port 5187 busy: close the other dev server; strictPort prevents connecting to the wrong app.
- Passive overlay: use the LKOS tray menu to restore input.
- Missing Rust components: `rustup show`, then retry the command.
- CI errors must be fixed or explicitly documented; never treat a failed runner as a passed gate.

## Overlay spike laboratory

Run `npm run dev:overlay` instead of `npm run dev`. This debug-only window exposes a target button, native state and the same handlers/menu used by the tray. It stores fixtures in `overlay-lab-settings.json` beside normal settings, leaving `settings.json` untouched.

Choose **Posicionar sobre alvo**, click the notch arrow, then **Ativar click-through** and click the same physical position. The underlying target should increment without changing the native counter. **Restaurar interação** should return input to the notch. Use **Menu nativo do tray** to exercise the shared popup callbacks. This does not replace testing the actual tray icon or another application.

Close the lab to restore input, or use the notch/menu exit to terminate LKOS. The lab is omitted from production builds. See [validation](VALIDATION.md) for observed results and remaining manual checks.

If Windows reserves the development port, inspect `netsh interface ipv4 show excludedportrange protocol=tcp`. Port 5187 replaced 1435 after a reservation prevented startup on the owner workstation. For another conflict, change package.json, vite.config.ts and tauri.conf.json (devUrl and devCsp) together; do not change Windows reservations.

After moving an existing checkout, stale Cargo permission paths can be regenerated with:

```powershell
cargo clean --manifest-path src-tauri/Cargo.toml -p tauri
cargo clean --manifest-path src-tauri/Cargo.toml --release -p tauri
npm run dev
```

These commands regenerate package build cache; they do not reset settings.
