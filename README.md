# LKOS

Local-first, modular Windows notch. **M0 foundation prototype**: transparent overlay, tray, versioned local settings, IPC/event fixture and independent module contracts. No production gadgets or adaptive engine yet.

Product source: [LKOS in Linear](https://linear.app/lukka-gs/project/lkos-98d7b40f689f).

## Run locally

Windows x64, Node 24.15.0/npm 11.12.1, Rust via rustup, Visual Studio C++ Build Tools and WebView2. See [setup](docs/development/SETUP.md).

```powershell
git clone https://github.com/Lukka-gs/lkos.git
cd lkos
git switch chore/m0-foundation
npm ci
npm run dev
```

The branch command is needed until the foundation PR is merged. One command starts both Vite and the native app. Click ↗ to test persistence and the event bridge; × or the tray menu exits. Keep one instance running at a time.

| Command | Result |
| --- | --- |
| `npm run dev` | Desktop development |
| `npm run build` | Windows release executable |
| `npm run lint` | Biome, architecture boundaries, Clippy |
| `npm run format` | Format frontend and Rust |
| `npm run typecheck` | TypeScript and Rust checks |
| `npm test` | Bus, lifecycle and persistence tests |
| `npm run check` | All validation gates |

Build output: `src-tauri/target/release/lkos.exe`. Installer/update/signing are future M9 work.

## Documentation

- [Product specification](docs/architecture/PRODUCT_SPEC.md)
- [Stack decision and alternatives](docs/architecture/STACK_DECISION.md)
- [Architecture and contracts](docs/architecture/ARCHITECTURE.md)
- [Technical risks](docs/architecture/RISKS.md)
- [Setup](docs/development/SETUP.md), [Git workflow](docs/development/GIT_WORKFLOW.md), [validation evidence](docs/development/VALIDATION.md)
- [Contributing](CONTRIBUTING.md)

CI runs on Windows for pull requests, with install, format check, lint, typecheck, tests and build. Main is the default branch; features use PRs. **Main protection is active:** PR, one approving review, successful Windows CI, up-to-date branch and resolved conversations are required; force pushes and deletion are disabled. See the Git workflow document.
