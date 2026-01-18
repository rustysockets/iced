# Fork Workflow: Public Contributions + Private Divergence
**Upstream**: `iced-rs/iced`  
**Public fork**: `rustysockets/iced`  
**Private overlay repo**: `rustysockets/riced`

This document describes a safe, repeatable workflow for:
- Contributing publicly to `iced-rs/iced` via the public fork `rustysockets/iced`
- Maintaining private/product changes in `rustysockets/riced` that may diverge indefinitely
- Preventing accidental pushes to upstream or leaking private code to the public fork

---

## Principles

- **Never push to upstream** (`iced-rs/iced`) directly.
- **Push public PR branches only to `origin`** (your public fork).
- **Push private branches only to `private`** (your private repo).
- Keep PRs **small, scoped, and easily reviewable** (core team time is limited).
- Always **push explicitly** (avoid `git push` without a remote).

---

## Remote layout (one-time setup per clone)

Run from your local clone:

```bash
cd /home/scott/code/Github/rustysockets-io/iced
```

### 1) Configure `origin` to your public fork

```bash
git remote set-url origin ssh://git@ssh.github.com:443/rustysockets/iced
git remote set-url --push origin ssh://git@ssh.github.com:443/rustysockets/iced
```

### 2) Add `upstream` pointing at the official repo (fetch-only)

```bash
git remote add upstream ssh://git@ssh.github.com:443/iced-rs/iced 2>/dev/null || true
git remote set-url upstream ssh://git@ssh.github.com:443/iced-rs/iced
```

### 3) Safety rail: disable push to `upstream`

```bash
git remote set-url --push upstream DISABLED
```

### 4) Add `private` pointing at your private overlay repo

```bash
git remote add private ssh://git@ssh.github.com:443/rustysockets/riced 2>/dev/null || true
git remote set-url private ssh://git@ssh.github.com:443/rustysockets/riced
git remote set-url --push private ssh://git@ssh.github.com:443/rustysockets/riced
```

### 5) Verify remotes

```bash
git remote -v
```

Expected:
- `origin` fetch/push → `rustysockets/iced`
- `upstream` fetch → `iced-rs/iced`
- `upstream` push → `DISABLED`
- `private` fetch/push → `rustysockets/riced`

---

## Branch naming conventions (reduces mistakes)

Use naming that encodes intent:
- **Public PR branches**: `pub/<topic>` or `scott/<topic>`
- **Private branches**: `priv/<topic>`

Examples:
- `pub/winit-no-panic-send-event`
- `scott/iced-wgpu-raster-cache-entry`
- `priv/product-patches`

---

## Keeping your fork synced with upstream

This updates your local `master` and then publishes it to your public fork:

```bash
git checkout master
git fetch upstream
git merge --ff-only upstream/master
git push origin master
```

If the upstream default branch is `main`, swap `master` → `main`.

---

## Public contribution workflow (open-source PR)

### 1) Branch from updated `master`

```bash
git checkout master
git pull --ff-only
git checkout -b pub/<topic>
```

### 2) Make changes (keep scope tight)

Guidelines:
- One behavior change / subsystem per PR
- Avoid drive-by formatting
- Avoid unrelated `Cargo.lock` changes unless necessary

### 3) Verify locally (targeted)

Examples:

```bash
cargo check -p iced_winit
cargo test  -p iced_winit
```

### 4) Commit (reviewer-friendly)

```bash
git status
git add -A
git commit -m "<area>: <short change>" -m "<what/why/how verified (optional)>"
```

### 5) Push ONLY to your public fork

```bash
git push -u origin pub/<topic>
```

### 6) Open PR to upstream

Open a PR with:
- **Base**: `iced-rs/iced` `master`
- **Compare**: `rustysockets/iced` `pub/<topic>`

---

## Private work workflow (diverge freely in `rustysockets/riced`)

### 1) Branch from `master` (or from a specific upstream tag/commit)

```bash
git checkout master
git pull --ff-only
git checkout -b priv/<topic>
```

### 2) Commit normally

```bash
git add -A
git commit -m "priv: <short change>" -m "<notes>"
```

### 3) Push ONLY to the private repo

```bash
git push -u private priv/<topic>
```

### Optional: maintain a single private integration branch

If your private repo uses `main` as the integration branch:

```bash
git checkout priv/<topic>
git push private priv/<topic>:main
```

---

## Updating private branches with upstream changes (optional but recommended)

```bash
git fetch upstream
git checkout priv/<topic>
git rebase upstream/master
git push --force-with-lease private priv/<topic>
```

Use `--force-with-lease` to avoid overwriting remote work unexpectedly.

---

## Stash workflow (avoid mixing experiments into PRs)

If you made local changes that shouldn’t go into the current PR:

```bash
git stash push -u -m "wip"
```

List / restore:

```bash
git stash list
git stash pop
```

---

## “Before pushing” checklist (30 seconds)

```bash
git branch --show-current
git remote -v
```

Then push explicitly:
- Public branch: `git push -u origin <branch>`
- Private branch: `git push -u private <branch>`

Avoid `git push` with no remote specified.

---

## Troubleshooting

### `git remote -v+` fails
`+` is not valid. Use:

```bash
git remote -v
```

### Port 22 blocked (SSH timeout)
Use SSH over port 443 (already used in this doc):
- `ssh://git@ssh.github.com:443/<org>/<repo>`

### Accidentally changed files you don’t want in a PR

```bash
git status
git restore <file>
# or
git restore .
```

---

## Quick commands reference

```bash
# remotes
git remote -v
git remote get-url origin
git remote get-url --push origin
git remote get-url private
git remote get-url --push private
git remote get-url upstream
git remote get-url --push upstream

# public sync
git fetch upstream
git merge --ff-only upstream/master
git push origin master

# safe rewrite push
git push --force-with-lease
```

