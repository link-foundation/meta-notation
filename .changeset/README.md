# Meta-Notation Changesets

This directory contains [changesets](https://github.com/changesets/changesets) which are used to manage versions and changelogs.

## How to add a changeset

When making changes that should trigger a version bump, run:

```bash
npx changeset
```

Or manually trigger a release via GitHub Actions:
1. Go to Actions tab
2. Select "Manual Release" workflow
3. Click "Run workflow"
4. Choose version bump type (patch/minor/major)
5. Add optional description

## Changeset types

- **patch**: Bug fixes and minor changes
- **minor**: New features (backward compatible)
- **major**: Breaking changes
