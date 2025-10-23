# Publishing Workflow

This repository includes an automated workflow to publish the `array_range_query` crate to crates.io when the version changes.

## How It Works

The workflow is triggered on every push to the `main` branch that modifies `Cargo.toml`. It performs the following steps:

1. **Version Detection**: Extracts the current version from `Cargo.toml`
2. **Version Comparison**: Queries crates.io API to check if the version already exists
3. **Conditional Publishing**: Only publishes if the version is new (not already on crates.io)

## Setup Requirements

To enable automatic publishing, you need to configure a secret in your GitHub repository:

### Setting up CARGO_REGISTRY_TOKEN

1. **Generate a crates.io API token**:
   - Go to https://crates.io/settings/tokens
   - Click "New Token"
   - Give it a descriptive name (e.g., "GitHub Actions - array_range_query")
   - Click "Generate"
   - Copy the generated token

2. **Add the token to GitHub Secrets**:
   - Go to your repository on GitHub
   - Navigate to Settings → Secrets and variables → Actions
   - Click "New repository secret"
   - Name: `CARGO_REGISTRY_TOKEN`
   - Value: Paste your crates.io API token
   - Click "Add secret"

## Publishing a New Version

To publish a new version:

1. Update the version number in `Cargo.toml`:
   ```toml
   [package]
   version = "0.2.3"  # Increment as appropriate
   ```

2. Commit and push to main:
   ```bash
   git add Cargo.toml
   git commit -m "Bump version to 0.2.3"
   git push origin main
   ```

3. The GitHub Action will automatically:
   - Detect the version change
   - Verify it's a new version
   - Publish to crates.io

## Workflow Files

- `.github/workflows/publish.yml` - Automated publishing workflow
- `.github/workflows/arq_main.yml` - CI/CD workflow (tests, linting, build)

## Manual Publishing

If you need to publish manually:

```bash
cargo publish --token YOUR_TOKEN
```

Or if you have the token in your environment:

```bash
cargo publish
```
