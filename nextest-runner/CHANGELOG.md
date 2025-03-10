# Changelog

## [0.12.0] - 2022-06-17

### Added

- On Unix, tests that fail due to a signal (e.g. SIGSEGV) will print out the name of the signal rather than the generic "FAIL".

### Changed

- Progress bars now take up the entire width of the screen. This prevents issues with the bar wrapping around on terminals that aren't wide enough.

## [0.11.1] - 2022-06-13

### Fixed

- Account for skipped tests when determining the length of the progress bar.

## [0.11.0] - 2022-06-13

### Added

- Nextest can now update itself! Once this version is installed, simply run `cargo nextest self update` to update to the latest version.
    > Note to distributors: you can disable self-update by building cargo-nextest with `--no-default-features`.
- Partial, emulated support for test binary arguments passed in after `cargo nextest run --` ([#265], thanks [@tabokie](https://github.com/tabokie) for your contribution!).

  For example, `cargo nextest run -- my_test --ignored` will run ignored tests containing `my_test`, similar to `cargo test -- my_test --ignored`.

  Support is limited to test names, `--ignored` and `--include-ignored`.

  > Note to integrators: to reliably disable all argument parsing, pass in `--` twice. For example, `cargo nextest run -- -- my-filter`.

### Fixed

- Better detection for cross-compilation -- now look through the `CARGO_BUILD_TARGET` environment variable, and Cargo configuration as well. The `--target` option is still preferred.
- Slow and flaky tests are now printed out properly in the final status output ([#270]).

[#265]: https://github.com/nextest-rs/nextest/pull/265
[#270]: https://github.com/nextest-rs/nextest/issues/270

## [0.10.0] - 2022-06-08

### Added

- Support for terminating tests if they take too long, via the configuration parameter `slow-timeout.terminate-after`. For example, to time out after 120 seconds:

    ```toml
    slow-timeout = { period = "60s", terminate-after = 2 }
    ```

    Thanks [steveeJ](https://github.com/steveeJ) for your contribution ([#214])!

[#214]: https://github.com/nextest-rs/nextest/pull/214

### Fixed

- Improved support for [reusing builds](https://nexte.st/book/reusing-builds): produce better error messages if the workspace's source is missing.

### Changed

- Errors are now defined with [thiserror](https://docs.rs/thiserror). Some minor API changes were required for the migration.

## [0.9.0] - 2022-06-07

This release contains a number of user experience improvements.

### Added

- If producing output to an interactive terminal, nextest now prints out its status as a progress bar. This makes it easy to see the status of a test run at a glance.
- Nextest's configuration has a new `final-status-level` option which can be used to print out some statuses at the end of a run (defaults to `none`). On the command line, this can be overridden with the `--final-status-level` argument or `NEXTEST_FINAL_STATUS_LEVEL` in the environment.
- If a [target runner](https://nexte.st/book/target-runners) is in use, nextest now prints out its name and the environment variable or config file the definition was obtained from.

### Changed

- If the creation of a test list fails, nextest now prints a more descriptive error message, and exits with the exit code 104 ([`TEST_LIST_CREATION_FAILED`]).

[`TEST_LIST_CREATION_FAILED`]: https://docs.rs/nextest-metadata/latest/nextest_metadata/enum.NextestExitCode.html#associatedconstant.TEST_LIST_CREATION_FAILED

## [0.8.1] - 2022-06-02

### Added

- Nextest now [sets `NEXTEST_LD_*` and `NEXTEST_DYLD_*` environment
  variables](https://nexte.st/book/env-vars.html#environment-variables-nextest-sets) to work around
  macOS System Integrity Protection sanitization.

### Fixed

- While [archiving build artifacts](https://nexte.st/book/reusing-builds), work around some libraries producing linked paths that don't exist ([#247]). Print a warning for those paths instead of failing.

[#247]: https://github.com/nextest-rs/nextest/issues/247

### Changed

- Build artifact archives no longer recurse into linked path subdirectories. This is not a behavioral change because `LD_LIBRARY_PATH` and other similar variables do not recurse into subdirectories either.

## [0.8.0] - 2022-05-31

### Added

- Support for creating and running archives of test binaries.
  - Most of the new logic is within a new `reuse_build` module.
- Non-test binaries and dynamic libraries are now recorded in `BinaryList`.

### Fixed

Fix for experimental feature [filter expressions](https://nexte.st/book/filter-expressions.html):
- Fix test filtering when expression filters are set but name-based filters aren't.

### Changed

- MSRV bumped to Rust 1.59.

## [0.7.0] - 2022-04-18

### Fixed

- `PathMapper` now canonicalizes the remapped workspace and target directories (and returns an error if that was unsuccessful).
- If the workspace directory is remapped, `CARGO_MANIFEST_DIR` in tests' runtime environment is set to the new directory.

## [0.6.0] - 2022-04-16

### Added

- Experimental support for [filter expressions](https://nexte.st/book/filter-expressions).

## [0.5.0] - 2022-03-22

### Added

- `BinaryList` and `TestList` have a new member called `rust_build_meta`, which returns Rust build-related metadata for a binary list or test list. This currently contains the target directory, the base output directories, and paths to [search for dynamic libraries in](https://nexte.st/book/env-vars#dynamic-library-paths) relative to the target directory.

### Changed

- MSRV bumped to Rust 1.56.

## [0.4.0] - 2022-03-07

Thanks to [Guiguiprim](https://github.com/Guiguiprim) for their contributions to this release!

### Added

- Filter test binaries by the build platform they're for (target or host).
- Experimental support for reusing build artifacts between the build and run steps.
- Nextest executions done as a separate process per test (currently the only supported method, though this might change in the future) set the environment variable `NEXTEST_PROCESS_MODE=process-per-test`.

### Changed

- `TargetRunner` now has separate handling for the target and host platforms. As part of this, a new struct `PlatformRunner` represents a target runner for a single platform.

## [0.3.0] - 2022-02-23

### Fixed

- Target runners of the form `runner = ["bin-name", "--arg1", ...]` are now parsed correctly ([#75]).
- Binary IDs for `[[bin]]` and `[[example]]` tests are now unique, in the format `<crate-name>::bin/<binary-name>` and `<crate-name>::test/<binary-name>` respectively ([#76]).

[#75]: https://github.com/nextest-rs/nextest/pull/75
[#76]: https://github.com/nextest-rs/nextest/pull/76

## [0.2.1] - 2022-02-23

- Improvements to `TargetRunnerError` message display: source errors are no longer displayed directly, only in "caused by".

## [0.2.0] - 2022-02-22

### Added

- Support for [target runners](https://nexte.st/book/target-runners).

## [0.1.2] - 2022-02-20

### Added

- In test output, module paths are now colored cyan ([#42]).

[#42]: https://github.com/nextest-rs/nextest/pull/42

## [0.1.1] - 2022-02-14

### Changed

- Updated quick-junit to 0.1.5, fixing builds on Rust 1.54.

## [0.1.0] - 2022-02-14

- Initial version.

[0.12.0]: https://github.com/nextest-rs/nextest/releases/tag/nextest-runner-0.12.0
[0.11.1]: https://github.com/nextest-rs/nextest/releases/tag/nextest-runner-0.11.1
[0.11.0]: https://github.com/nextest-rs/nextest/releases/tag/nextest-runner-0.11.0
[0.10.0]: https://github.com/nextest-rs/nextest/releases/tag/nextest-runner-0.10.0
[0.9.0]: https://github.com/nextest-rs/nextest/releases/tag/nextest-runner-0.9.0
[0.8.1]: https://github.com/nextest-rs/nextest/releases/tag/nextest-runner-0.8.1
[0.8.0]: https://github.com/nextest-rs/nextest/releases/tag/nextest-runner-0.8.0
[0.7.0]: https://github.com/nextest-rs/nextest/releases/tag/nextest-runner-0.7.0
[0.6.0]: https://github.com/nextest-rs/nextest/releases/tag/nextest-runner-0.6.0
[0.5.0]: https://github.com/nextest-rs/nextest/releases/tag/nextest-runner-0.5.0
[0.4.0]: https://github.com/nextest-rs/nextest/releases/tag/nextest-runner-0.4.0
[0.3.0]: https://github.com/nextest-rs/nextest/releases/tag/nextest-runner-0.3.0
[0.2.1]: https://github.com/nextest-rs/nextest/releases/tag/nextest-runner-0.2.1
[0.2.0]: https://github.com/nextest-rs/nextest/releases/tag/nextest-runner-0.2.0
[0.1.2]: https://github.com/nextest-rs/nextest/releases/tag/nextest-runner-0.1.2
[0.1.1]: https://github.com/nextest-rs/nextest/releases/tag/nextest-runner-0.1.1
[0.1.0]: https://github.com/nextest-rs/nextest/releases/tag/nextest-runner-0.1.0
