# Release Process

This document describes how to create a new release of RustX.

## Automated Releases

RustX uses GitHub Actions to automatically build and publish releases for multiple platforms.

### Supported Platforms

The release workflow builds binaries for:

- **Linux**
  - x86_64 (`rustx-linux-x86_64.tar.gz`)
  - ARM64 (`rustx-linux-aarch64.tar.gz`)

- **macOS** 
  - Intel x86_64 (`rustx-macos-x86_64.tar.gz`)
  - Apple Silicon ARM64 (`rustx-macos-aarch64.tar.gz`)

- **Windows**
  - x86_64 (`rustx-windows-x86_64.zip`)

## Creating a Release

### 1. Update Version

Update the version in the following files:

```bash
# Update version in Cargo.toml
version = "0.6.0"

# Update in crates/cli/src/main.rs
#[command(version = "0.6.0")]

# Update CHANGELOG.md
## [0.6.0] - 2025-XX-XX
```

### 2. Commit Changes

```bash
git add Cargo.toml crates/cli/src/main.rs CHANGELOG.md Cargo.lock
git commit -m "Bump version to 0.6.0"
git push origin main
```

### 3. Create and Push Tag

```bash
# Create annotated tag
git tag -a v0.6.0 -m "Release v0.6.0"

# Push tag to trigger release workflow
git push origin v0.6.0
```

### 4. Automated Build

Once the tag is pushed, GitHub Actions will automatically:

1. Create a GitHub Release
2. Build binaries for all platforms
3. Upload binaries to the release
4. Update release notes

### 5. Verify Release

Check the [Releases page](https://github.com/GrandpaEJx/RustX/releases) to verify:

- All binaries are uploaded
- Release notes are correct
- Download links work

## Manual Release (if needed)

If the automated release fails, you can build manually:

```bash
# Linux x86_64
cargo build --release --target x86_64-unknown-linux-gnu
tar czf rustx-linux-x86_64.tar.gz -C target/x86_64-unknown-linux-gnu/release rustx

# macOS x86_64
cargo build --release --target x86_64-apple-darwin
tar czf rustx-macos-x86_64.tar.gz -C target/x86_64-apple-darwin/release rustx

# macOS ARM64 (M1/M2)
cargo build --release --target aarch64-apple-darwin
tar czf rustx-macos-aarch64.tar.gz -C target/aarch64-apple-darwin/release rustx

# Windows x86_64
cargo build --release --target x86_64-pc-windows-msvc
# Create zip from target/x86_64-pc-windows-msvc/release/rustx.exe
```

Then manually upload to GitHub Releases.

## Testing Installation

After release, test the installation script:

```bash
# Test on Linux/macOS
curl -sSL https://raw.githubusercontent.com/GrandpaEJx/RustX/main/install.sh | bash

# Verify installation
rustx --version
rustx examples/hello_world.rsx
```

## Rollback

If a release has issues:

1. Delete the bad tag and release from GitHub
2. Fix the issue
3. Create a new patch version (e.g., v0.6.1)

## Version Numbering

RustX follows [Semantic Versioning](https://semver.org/):

- **Major** (1.0.0): Breaking changes
- **Minor** (0.6.0): New features, backward compatible
- **Patch** (0.5.1): Bug fixes, backward compatible

## Checklist

Before creating a release:

- [ ] All tests pass (`cargo test`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Formatted (`cargo fmt`)
- [ ] Version updated in all files
- [ ] CHANGELOG.md updated
- [ ] Examples tested
- [ ] Documentation updated
- [ ] Benchmarks run (if performance changes)

## CI/CD Workflows

- **ci.yml**: Runs on every push/PR (test, clippy, fmt)
- **release.yml**: Runs on tag push (build + release)

View workflow runs: [GitHub Actions](https://github.com/GrandpaEJx/RustX/actions)
