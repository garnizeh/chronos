# Changelog

## [0.5.1](https://github.com/garnizeh/chronos/compare/chronos-daemon-v0.5.0...chronos-daemon-v0.5.1) (2026-03-21)

## [0.5.0](https://github.com/garnizeh/chronos/compare/chronos-daemon-v0.4.0...chronos-daemon-v0.5.0) (2026-03-21)


### Features

* **chronos-capture:** achieve 100% logic coverage with mocks ([3d98c6f](https://github.com/garnizeh/chronos/commit/3d98c6f39610d00bb0f28446c07693d299b1cdee))

## [0.4.0](https://github.com/garnizeh/chronos/compare/chronos-daemon-v0.3.0...chronos-daemon-v0.4.0) (2026-03-21)


### Features

* **chronos-core:** add InvalidInput error and validate database limits ([e7c3625](https://github.com/garnizeh/chronos/commit/e7c3625d7a52368f948db98ecebfb48298886c68))
* **chronos-daemon:** implement sqlite database layer for semantic logs ([f2ca3d9](https://github.com/garnizeh/chronos/commit/f2ca3d96ce2cadd6d4933707a60f1fe9adf0b993))
* **chronos-daemon:** scaffold binary crate with sqlx and clap ([527af29](https://github.com/garnizeh/chronos/commit/527af29e4f497345c3f25d7816f8e5d4ba359211))
* **database:** add CHECK constraint for confidence_score and verify with tests ([1004889](https://github.com/garnizeh/chronos/commit/10048899ef07e003e83827a35f6583588985a4f8))
* implement pragmatic CI/CD and developer tooling improvements ([5cd00de](https://github.com/garnizeh/chronos/commit/5cd00de1efffd8aa6f91c0974d8c9a7ab222dbd2))


### Bug Fixes

* **chronos-daemon:** repair corrupted database.rs and improve in-memory pool ([b7b18b7](https://github.com/garnizeh/chronos/commit/b7b18b7a0fd77bae7ac40794c07dea183b1deff4))
* **chronos-daemon:** use unique in-memory URI for test isolation ([95c218e](https://github.com/garnizeh/chronos/commit/95c218e6bcc7bb61d03ad052efbce6f7b1b90465))
* **workspace:** revert version.workspace inheritance to fix release-please parsing ([706fd7c](https://github.com/garnizeh/chronos/commit/706fd7c4574710624255557c73315cb5d461b479))

## [0.3.0](https://github.com/garnizeh/chronos/compare/chronos-daemon-v0.2.0...chronos-daemon-v0.3.0) (2026-03-21)


### Features

* **chronos-core:** add InvalidInput error and validate database limits ([e7c3625](https://github.com/garnizeh/chronos/commit/e7c3625d7a52368f948db98ecebfb48298886c68))
* **chronos-daemon:** implement sqlite database layer for semantic logs ([f2ca3d9](https://github.com/garnizeh/chronos/commit/f2ca3d96ce2cadd6d4933707a60f1fe9adf0b993))
* **database:** add CHECK constraint for confidence_score and verify with tests ([1004889](https://github.com/garnizeh/chronos/commit/10048899ef07e003e83827a35f6583588985a4f8))


### Bug Fixes

* **chronos-daemon:** repair corrupted database.rs and improve in-memory pool ([b7b18b7](https://github.com/garnizeh/chronos/commit/b7b18b7a0fd77bae7ac40794c07dea183b1deff4))
* **chronos-daemon:** use unique in-memory URI for test isolation ([95c218e](https://github.com/garnizeh/chronos/commit/95c218e6bcc7bb61d03ad052efbce6f7b1b90465))

## [0.2.0](https://github.com/garnizeh/chronos/compare/chronos-daemon-v0.1.0...chronos-daemon-v0.2.0) (2026-03-21)


### Features

* implement pragmatic CI/CD and developer tooling improvements ([5cd00de](https://github.com/garnizeh/chronos/commit/5cd00de1efffd8aa6f91c0974d8c9a7ab222dbd2))

## 0.1.0 (2026-03-21)


### Features

* **chronos-daemon:** scaffold binary crate with sqlx and clap ([527af29](https://github.com/garnizeh/chronos/commit/527af29e4f497345c3f25d7816f8e5d4ba359211))


### Bug Fixes

* **workspace:** revert version.workspace inheritance to fix release-please parsing ([706fd7c](https://github.com/garnizeh/chronos/commit/706fd7c4574710624255557c73315cb5d461b479))
