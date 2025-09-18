# Contributing to gigachat-rust

## Development Setup

### Prerequisites
- Rust 1.70+ (MSRV)
- Git

### Local Development
1. Clone the repository
2. Run tests: `cargo test`
3. Check formatting: `cargo fmt --check`
4. Run lints: `cargo clippy --all-targets --all-features`

## CI/CD Setup

### Required GitHub Secrets

The following secrets must be configured in your GitHub repository settings:

#### For Publishing to Crates.io
- `CRATES_IO_TOKEN` - Your crates.io API token
  - Go to https://crates.io/settings/tokens
  - Create a new token with publish permissions
  - Add it to GitHub repository secrets

#### For Security Scanning (Optional)
- `SEMGREP_APP_TOKEN` - Semgrep App token for enhanced security scanning
  - Sign up at https://semgrep.dev/
  - Get your app token from settings
  - Add it to GitHub repository secrets

#### For Code Coverage (Optional)
- `CODECOV_TOKEN` - Codecov token for code coverage reports
  - Sign up at https://codecov.io/
  - Add your repository and get the token
  - Add it to GitHub repository secrets

### How to Release

The release process is manual and triggered via GitHub Actions:

1. Go to the "Actions" tab in your GitHub repository
2. Select the "Release" workflow
3. Click "Run workflow"
4. Enter the version number (e.g., `0.1.1`, `1.0.0-alpha.1`)
5. Optionally check "Dry run" to test without publishing

#### Release Process Steps
1. **Validation** - Checks version format and builds the project
2. **GitHub Release** - Creates a GitHub release with changelog
3. **Crates.io Publishing** - Publishes to crates.io
4. **Version Bump** - Commits version changes back to repository

#### Version Format
Use semantic versioning: `MAJOR.MINOR.PATCH[-PRERELEASE]`
- `1.0.0` - Major release
- `0.1.1` - Patch release
- `1.0.0-alpha.1` - Pre-release (marked as prerelease on GitHub)

### Workflows

#### CI (`ci.yml`)
Runs on every push and pull request:
- Tests on stable, beta, and nightly Rust
- Code formatting check
- Clippy linting
- Documentation check
- Code coverage (if Codecov token configured)

#### Release (`release.yml`)
Manual workflow for publishing:
- Version validation
- GitHub release creation
- Crates.io publishing
- Automatic version bump commit

#### Security (`security.yml`)
Runs on push/PR and weekly:
- Dependency security audit
- Supply chain security checks
- Static analysis with Semgrep
- Dependency review on PRs

## Pull Request Guidelines

1. Ensure CI passes
2. Add tests for new functionality
3. Update documentation if needed
4. Follow Rust conventions and idioms
5. Keep commits focused and well-described

## Code Style

- Use `cargo fmt` for formatting
- Follow Clippy suggestions
- Write comprehensive documentation
- Add tests for new features
- Handle errors appropriately