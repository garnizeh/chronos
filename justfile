# ==============================================================================
# Chronos Workspace - Justfile
# Run `just` to see all available commands.
# ==============================================================================

# Sets the default shell to bash for better compatibility
set shell := ["bash", "-c"]

# The default recipe lists all available commands
@default:
    just --list

# ==============================================================================
# 🛠️ 1. SETUP & DEPENDENCIES
# ==============================================================================

# Install all required Cargo binaries for the development workflow
@setup:
    echo "Installing required Cargo tools..."
    # Reason: cargo-binstall downloads pre-compiled binaries instead of
    # compiling from source, turning a 10-minute setup into ~15 seconds.
    # We use the official bootstrap script for maximum speed, falling back
    # to cargo install if necessary.
    # Go Equivalent: Using pre-built binaries instead of building from source.
    command -v cargo-binstall >/dev/null || \
      (curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash) || \
      cargo install cargo-binstall
    # Reason: sqlx-cli needs --no-default-features/--features flags, which
    # cargo-binstall does not support. We keep `cargo install` for this one.
    cargo install sqlx-cli --no-default-features --features rustls,sqlite
    cargo binstall -y cargo-llvm-cov
    cargo binstall -y cargo-nextest --locked
    cargo binstall -y cargo-deny
    cargo binstall -y cargo-outdated
    cargo binstall -y cargo-release
    echo "Setup complete!"

# ==============================================================================
# 🧹 2. GIT HELPERS
# ==============================================================================

# Fetch from origin and prune local branches that were deleted on the remote
@git-prune:
    echo "Fetching origin and pruning deleted branches..."
    git fetch -p
    if [ -n "$(git branch -vv | grep ': gone]' | awk '{print $1}')" ]; then \
        git branch -vv | grep ': gone]' | awk '{print $1}' | xargs git branch -D; \
    fi
    echo "Clean up finished!"

# ==============================================================================
# 🗄️ 3. DATABASE MANAGEMENT (SQLite / sqlx)
# ==============================================================================

# Create the SQLite database file.
# Automatically creates a .env with DATABASE_URL if one doesn't exist.
@db-setup:
    [ -f .env ] || echo 'DATABASE_URL=sqlite://chronos.db' > .env
    mkdir -p migrations
    sqlx database setup

# Run pending migrations
@db-migrate:
    sqlx migrate run

# Create a new migration file. Usage: `just db-add create_users_table`
@db-add name:
    sqlx migrate add -r {{name}}

# Drop the database, recreate it, and run all migrations from scratch
@db-reset:
    [ -f .env ] || echo 'DATABASE_URL=sqlite://chronos.db' > .env
    sqlx database drop -y
    sqlx database setup
    sqlx migrate run

# Prepare sqlx metadata for offline builds (commit .sqlx/ to git)
@sqlx-prepare:
    cargo sqlx prepare --workspace

# Check if sqlx metadata is up to date (for CI and QA)
@sqlx-check:
    cargo sqlx prepare --workspace --check

# ==============================================================================
# 🚀 4. BUILD & RUN
# ==============================================================================

# Quickly check if the code compiles without producing binaries
@check:
    cargo check --workspace

# Build the entire workspace in debug mode
@build:
    cargo build --workspace

# Build the entire workspace in release mode (optimized)
@build-release:
    cargo build --workspace --release

# Run a specific crate with optional arguments. Usage: `just run chronos-daemon start`
# Reason: The `*args` variadic captures all trailing arguments and forwards them
# after `--` to the target binary. Without this, `just run chronos-daemon start`
# would silently drop the `start` argument.
# Go Equivalent: This is like `go run ./cmd/daemon start` — Go naturally forwards args.
@run crate *args:
    cargo run -p {{crate}} -- {{args}}

# ==============================================================================
# 🧪 5. QUALITY ASSURANCE (QA)
# ==============================================================================

# Format the codebase
@fmt:
    cargo fmt --all

# Run the linter with zero-tolerance for warnings
@lint:
    cargo clippy --workspace --all-targets -- -D warnings

# Check for security vulnerabilities and unmaintained dependencies
@audit:
    cargo deny check

# Run all tests in the workspace (using nextest)
@test:
    cargo nextest run --workspace

# Run tests with LLVM coverage and generate lcov.info (using nextest)
@coverage:
    cargo llvm-cov nextest --workspace --lcov --output-path lcov.info
    echo "Coverage report generated at lcov.info"

# Generate and open HTML coverage report locally
@coverage-html:
    echo "Generating HTML coverage report..."
    cargo llvm-cov nextest --workspace --html --open

# Verify that the code compiles with the declared MSRV (1.94)
@msrv:
    rustup toolchain list | grep -q "1.94" || rustup toolchain install 1.94
    cargo +1.94 check --workspace

# Generate and open workspace documentation locally
@doc:
    cargo doc --workspace --no-deps --open

# ==============================================================================
# 🛡️ 6. LOCAL CI REPLICA
# ==============================================================================

# Run the exact CI checks locally before committing to catch errors early.
# This is the single source of truth for pre-commit validation.
@ci-local:
    echo "Running local CI pipeline (Privacy-First Strict Checks)..."
    echo -e "\n1. Checking formatting..."
    cargo fmt --all -- --check
    echo -e "\n2. Running security audit (cargo-deny)..."
    cargo deny check
    echo -e "\n3. Running strict linter (includes offline SQLx verification)..."
    cargo clippy --workspace --all-targets -- -D warnings
    echo -e "\n4. Running test suite with nextest..."
    cargo nextest run --workspace
    echo -e "\n5. Generating test coverage summary..."
    cargo llvm-cov nextest --workspace
    echo -e "\n✅ All local CI checks passed! Safe to commit."

# ==============================================================================
# 📦 7. RELEASE & MAINTENANCE
# ==============================================================================

# Check for outdated dependencies in the workspace
@outdated:
    cargo outdated

# Trigger a release (updates Cargo.toml, creates a git tag, and commits)
# Usage: `just release minor` or `just release patch`
@release level:
    cargo release {{level}} --execute

# Clean the target directory (free up disk space)
@clean:
    cargo clean