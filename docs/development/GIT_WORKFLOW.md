# Git workflow

Repository: https://github.com/Lukka-gs/lkos. Default branch: `main`.

Use `feat/<scope>`, `fix/<scope>`, `refactor/<scope>`, `chore/<scope>` or `docs/<scope>`. M0 foundation uses `chore/m0-foundation`. Main contains only bootstrap until the PR is reviewed and merged.

```powershell
git switch main
git pull --ff-only
git switch -c feat/luk-19-shell-lifecycle
# implement and validate the issue
npm run check
git add <specific-files>
git commit -m "feat(shell): add single-instance lifecycle"
git push -u origin feat/luk-19-shell-lifecycle
```

PR description: problem/result, linked Linear issue, tests, native evidence, limitations. Use small coherent Conventional Commits. Keep documentation, implementation and verification changes separable when useful.

Required protection configuration: PR required, one approval, dismiss stale approvals, required `Windows foundation` status check, branch up to date, resolved conversations, enforcement for administrators, no force pushes or deletion.

**Current status:** The owner authorized public visibility. The repository is public and the protection configuration above is active, including administrator enforcement. PR #1 is ready for review. GitHub refuses self-approval by its author; another authorized reviewer must approve before merge.

CI runs on PRs to main and pushes to main. It installs locked dependencies, checks formatting, lint, architecture, types, tests and compiles the executable on Windows. It has read-only repository permissions and never publishes a release.
