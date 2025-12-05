# Upstream Sync Workflow

This repository uses a selective upstream sync strategy. The `master` branch contains our custom VPN automation system, while `upstream-sync` tracks the original hwdsl2/wireguard-install repository.

## Branches

- **`master`**: Main branch with our custom VPN automation, Tailscale integrations, and documentation
- **`upstream-sync`**: Tracks upstream hwdsl2/wireguard-install changes only

## Manual Upstream Sync Process

### 1. Sync Upstream Changes
```bash
# Switch to upstream-sync branch
git checkout upstream-sync

# Pull latest upstream changes
git pull upstream master

# Push to our remote (optional, for backup)
git push origin upstream-sync
```

### 2. Review Upstream Changes
```bash
# See what's new in upstream
git log --oneline upstream-sync ^master

# Or compare specific files
git diff master..upstream-sync --stat
```

### 3. Selective Merge (Manual Decision)
For each upstream change, decide whether to merge:

**Option A: Merge Everything**
```bash
git checkout master
git merge upstream-sync
```

**Option B: Cherry-pick Specific Changes**
```bash
git checkout master
git cherry-pick <commit-hash>
```

**Option C: Manual File Updates**
```bash
# Copy specific files you want
cp docs/wg-install-hwds12/some-upstream-file.md docs/
```

## What to Sync vs Not Sync

### ✅ Usually Sync:
- Bug fixes in `wireguard.sh`
- Security updates
- New OS support
- Performance improvements

### ❌ Usually Don't Sync:
- Documentation changes (we have our own)
- UI changes (we have custom docs)
- Feature additions (we have our automation system)
- Chinese translations (we removed them)

## Current Upstream Files Preserved

The original upstream content is preserved in:
- `docs/wg-install-hwds12/` - Original hwdsl2 documentation
- `wireguard.sh` - Original installer script
- `README-wg-auto.md` - Modified upstream README

## Emergency Upstream Override

If you need to completely override with upstream:
```bash
git checkout upstream-sync
git merge upstream-sync --strategy=ours master  # Keep upstream
# OR
git checkout master
git reset --hard upstream-sync  # Complete upstream override (dangerous!)
```

## Maintenance Schedule

- Check upstream weekly: `git fetch upstream`
- Major releases: Review and potentially sync
- Security fixes: Sync immediately