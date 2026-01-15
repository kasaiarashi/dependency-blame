# Package Manager Setup

This document explains how to create releases and publish dependency-blame to various package managers.

## GitHub Releases

The GitHub Actions workflow (`.github/workflows/release.yml`) automatically:
- Builds binaries for Windows, Linux, and macOS (x64 and ARM64)
- Creates a GitHub release when you push a version tag
- Uploads binaries as release assets

### Creating a New Release

1. **Update version** in `Cargo.toml`:
   ```toml
   [package]
   version = "0.2.0"
   ```

2. **Update CHANGELOG.md**:
   - Move items from `[Unreleased]` to a new version section
   - Add release date
   - Update comparison links at the bottom

3. **Commit changes**:
   ```bash
   git add Cargo.toml CHANGELOG.md
   git commit -m "Bump version to 0.2.0"
   git push origin master
   ```

4. **Create and push tag**:
   ```bash
   git tag v0.2.0
   git push origin v0.2.0
   ```

5. **Wait for GitHub Actions** to build and create the release automatically

The workflow will create a release with binaries for:
- `dependency-blame-linux-x64` - Linux x86_64
- `dependency-blame-windows-x64.exe` - Windows x86_64
- `dependency-blame-macos-x64` - macOS Intel
- `dependency-blame-macos-arm64` - macOS Apple Silicon

## Cargo (crates.io)

Publishing to crates.io:

1. **Ensure you're logged in**:
   ```bash
   cargo login
   ```

2. **Test the package**:
   ```bash
   cargo publish --dry-run
   ```

3. **Publish**:
   ```bash
   cargo publish
   ```

Users can then install with:
```bash
cargo install dependency-blame
```

## Chocolatey (Windows)

To publish dependency-blame to Chocolatey:

1. **Create Chocolatey package structure**:
   ```
   chocolatey/
   ├── dependency-blame.nuspec
   └── tools/
       └── chocolateyinstall.ps1
   ```

2. **Update version** in `dependency-blame.nuspec`

3. **Update URL and checksum** in `chocolateyinstall.ps1`:
   ```powershell
   # Get checksum from GitHub release
   certutil -hashfile dependency-blame-windows-x64.exe SHA256
   ```

4. **Package**:
   ```powershell
   cd chocolatey
   choco pack
   ```

5. **Test locally**:
   ```powershell
   choco install dependency-blame -source .
   ```

6. **Publish**:
   ```powershell
   choco push dependency-blame.0.2.0.nupkg --source https://push.chocolatey.org/
   ```

**Note**: You need a Chocolatey API key from https://community.chocolatey.org/account

## Scoop (Windows)

To make dependency-blame available via Scoop:

1. **Create a scoop-bucket repository** (if you don't have one)

2. **Add manifest** (`dependency-blame.json`) to the bucket:
   ```json
   {
     "version": "0.1.0",
     "description": "Analyze why dependencies exist in your project",
     "homepage": "https://github.com/kasaiarashi/dependency-blame",
     "license": "MIT",
     "architecture": {
       "64bit": {
         "url": "https://github.com/kasaiarashi/dependency-blame/releases/download/v0.1.0/dependency-blame-windows-x64.exe",
         "bin": "dependency-blame-windows-x64.exe",
         "hash": "<SHA256_HASH>"
       }
     },
     "checkver": {
       "github": "https://github.com/kasaiarashi/dependency-blame"
     },
     "autoupdate": {
       "architecture": {
         "64bit": {
           "url": "https://github.com/kasaiarashi/dependency-blame/releases/download/v$version/dependency-blame-windows-x64.exe"
         }
       }
     }
   }
   ```

3. **Get hash**:
   ```powershell
   (Get-FileHash dependency-blame-windows-x64.exe -Algorithm SHA256).Hash
   ```

4. **Users install with**:
   ```powershell
   scoop bucket add kasaiarashi https://github.com/kasaiarashi/scoop-bucket
   scoop install dependency-blame
   ```

## Homebrew (macOS)

For macOS support via Homebrew:

1. **Create a Homebrew tap repository** named `homebrew-tap`

2. **Add formula** (`dependency-blame.rb`):
   ```ruby
   class DependencyBlame < Formula
     desc "Analyze why dependencies exist in your project"
     homepage "https://github.com/kasaiarashi/dependency-blame"
     version "0.1.0"

     on_macos do
       if Hardware::CPU.arm?
         url "https://github.com/kasaiarashi/dependency-blame/releases/download/v0.1.0/dependency-blame-macos-arm64"
         sha256 "<ARM64_SHA256>"
       else
         url "https://github.com/kasaiarashi/dependency-blame/releases/download/v0.1.0/dependency-blame-macos-x64"
         sha256 "<X64_SHA256>"
       end
     end

     def install
       bin.install Dir["dependency-blame*"].first => "dependency-blame"
     end

     test do
       system "#{bin}/dependency-blame", "--version"
     end
   end
   ```

3. **Users install with**:
   ```bash
   brew tap kasaiarashi/tap
   brew install dependency-blame
   ```

## AUR (Arch Linux)

For Arch Linux users via AUR:

1. **Create PKGBUILD**:
   ```bash
   # Maintainer: kasaiarashi
   pkgname=dependency-blame
   pkgver=0.1.0
   pkgrel=1
   pkgdesc="Analyze why dependencies exist in your project"
   arch=('x86_64')
   url="https://github.com/kasaiarashi/dependency-blame"
   license=('MIT')
   depends=()
   source=("$pkgname-$pkgver::https://github.com/kasaiarashi/dependency-blame/releases/download/v$pkgver/dependency-blame-linux-x64")
   sha256sums=('<SHA256>')

   package() {
     install -Dm755 "$srcdir/$pkgname-$pkgver" "$pkgdir/usr/bin/dependency-blame"
   }
   ```

2. **Submit to AUR** following the [AUR submission guidelines](https://wiki.archlinux.org/title/AUR_submission_guidelines)

## Release Checklist

Before creating a new release:

- [ ] Update version in `Cargo.toml`
- [ ] Update `CHANGELOG.md` with new version and date
- [ ] Test all commands work correctly
- [ ] Test on different ecosystems (Rust, Node.js, Python, Go)
- [ ] Commit changes to master
- [ ] Create and push git tag
- [ ] Wait for GitHub Actions to complete
- [ ] Verify binaries work on each platform
- [ ] Update package managers (Cargo, Chocolatey, Scoop, Homebrew, AUR)
- [ ] Announce release (Twitter, Reddit, etc.)
