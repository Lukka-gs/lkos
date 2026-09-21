# Contributing

Read `docs/architecture/PRODUCT_SPEC.md`, `ARCHITECTURE.md` and the relevant Linear issue before changing code. Keep scope within that issue; do not add future features during foundation work.

1. Follow `docs/development/SETUP.md`.
2. Branch from updated main using `feat/`, `fix/`, `refactor/`, `chore/` or `docs/`.
3. Keep modules independent and native APIs behind Platform adapters. Core must not import concrete gadgets/tools.
4. Add behavior tests for affected contracts, failure paths and migrations. Describe required Windows manual checks honestly.
5. Run `npm run check`.
6. Make small Conventional Commits and open a PR linked to its Linear issue. Include behavior, test results and unresolved risks.
7. Review the diff and passing Windows CI before merge. Independent approval is optional while development is solo. Never force-push main or work directly on it.

Only mark an issue Done when its actual acceptance criteria are met. Compilation does not prove click-through, DPI, focus, installation or low idle consumption. Do not commit local settings, credentials, generated build output or signing keys.
