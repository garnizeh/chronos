# Changelog

## [0.7.0](https://github.com/garnizeh/chronos/compare/chronos-daemon-v0.6.0...chronos-daemon-v0.7.0) (2026-03-22)


### Features

* **chronos-daemon:** achieve full orchestration coverage via run_orchestrator ([7f5598b](https://github.com/garnizeh/chronos/commit/7f5598bae562f42df20b621f0375c3c665bec5ed))
* **chronos-daemon:** implement cli and query interface ([72cfb4a](https://github.com/garnizeh/chronos/commit/72cfb4ab22c8e7c7e1b57407dd66bb6036d904df))
* **chronos-daemon:** implement graceful shutdown with ctrl_c ([937c411](https://github.com/garnizeh/chronos/commit/937c41188264e29c5bd6ad1d8fb79ec9d94c29ad))
* **chronos-daemon:** improve main.rs coverage by extracting run_app ([cc47fc2](https://github.com/garnizeh/chronos/commit/cc47fc22ae4c0529beee21582b335b633ef08c09))
* **chronos-daemon:** improve testability and increase coverage ([2a40e4d](https://github.com/garnizeh/chronos/commit/2a40e4dd757006c86490dfff898e7b75f3d13bb4))


### Bug Fixes

* address multiple security, usability, and test findings ([35c4398](https://github.com/garnizeh/chronos/commit/35c439860bbba5d8017d3c66352d3945f365a8fd))
* address multiple security, usability, and test findings ([574c61b](https://github.com/garnizeh/chronos/commit/574c61bf04daae2602c05946634a795adce390ca))
* **chronos-daemon:** enforce limit in date-range queries ([a703afe](https://github.com/garnizeh/chronos/commit/a703afec05efaf3ae9fbaf2ac8977ae393a72fc5))
* **chronos-daemon:** ensure proper task monitoring and exit code propagation ([460431c](https://github.com/garnizeh/chronos/commit/460431cb7392477104e6fb3150d5ff2d097cdef4))
* **chronos-daemon:** ensure truncate is UTF-8 safe ([ef5e9c5](https://github.com/garnizeh/chronos/commit/ef5e9c5a9f241525dfc78e15460e56bb36d0b3ef))
* improve inference resiliency and query normalization ([5f521c3](https://github.com/garnizeh/chronos/commit/5f521c33b9c44eca58c20e73727d21f423b8afaa))

## [0.6.0](https://github.com/garnizeh/chronos/compare/chronos-daemon-v0.5.1...chronos-daemon-v0.6.0) (2026-03-21)


### Features

* **chronos-daemon:** implement pipeline integration (Capture -&gt; Vision -&gt; DB) ([bb69a1f](https://github.com/garnizeh/chronos/commit/bb69a1ffe714f146438a1805510df40d96e80fff))
* **chronos-daemon:** refine pipeline resilience and sync task docs ([b8d5419](https://github.com/garnizeh/chronos/commit/b8d54191f216bdf0e3f985216be4c70c9312d30f))

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
