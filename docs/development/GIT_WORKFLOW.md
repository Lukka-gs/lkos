# Git workflow

Repository: https://github.com/Lukka-gs/lkos. Default branch: `main`.

The repository is public. PR #1 merged the M0 foundation. At the owner's request, main protection and mandatory independent reviews are disabled while the project has one developer. Continue using feature branches and PRs; this policy does not mean developing features directly on main.

Use `feat/<scope>`, `fix/<scope>`, `refactor/<scope>`, `chore/<scope>` or `docs/<scope>`.

```powershell
git switch main
git pull --ff-only
git switch -c fix/luk-18-overlay-validation
# implement and validate the issue
npm run check
git add <specific-files>
git commit -m "fix(overlay): clamp placement to monitor work area"
git push -u origin fix/luk-18-overlay-validation
```

PR description: problem/result, linked Linear issue, tests, native evidence, limitations. Use small coherent Conventional Commits. Review the diff and passing CI before merge. GitHub does not allow an author to approve their own PR; an independent approval is currently not required. Do not force-push main.

CI runs on PRs to main and pushes to main. It installs locked dependencies, checks formatting, lint, architecture, types, tests and compiles the executable on Windows. It has read-only repository permissions and never publishes a release.

When collaborators join, reconsider PR protection, an independent review, required Windows CI, resolved conversations and restrictions on force pushes/deletion.
