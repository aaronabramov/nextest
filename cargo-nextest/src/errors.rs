// Copyright (c) The nextest Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use camino::Utf8PathBuf;
use nextest_filtering::errors::FilterExpressionParseErrors;
use nextest_metadata::NextestExitCode;
use nextest_runner::errors::*;
use owo_colors::{OwoColorize, Stream};
use std::{
    error::{self, Error},
    fmt,
};

#[derive(Debug)]
#[doc(hidden)]
pub enum ReuseBuildKind {
    Normal,
    ReuseWithWorkspaceRemap { workspace_root: Utf8PathBuf },
    Reuse,
}

/// An error occurred in a program that nextest ran, not in nextest itself.
#[derive(Debug)]
#[doc(hidden)]
pub enum ExpectedError {
    CargoMetadataFailed,
    RootManifestNotFound {
        path: Utf8PathBuf,
        reuse_build_kind: ReuseBuildKind,
    },
    ProfileNotFound {
        err: ProfileNotFound,
    },
    CargoConfigsConstructError {
        err: CargoConfigsConstructError,
    },
    ConfigParseError {
        err: ConfigParseError,
    },
    ArgumentFileReadError {
        arg_name: &'static str,
        file_name: Utf8PathBuf,
        err: std::io::Error,
    },
    UnknownArchiveFormat {
        archive_file: Utf8PathBuf,
        err: UnknownArchiveFormat,
    },
    ArchiveCreateError {
        archive_file: Utf8PathBuf,
        err: ArchiveCreateError,
    },
    ArchiveExtractError {
        archive_file: Utf8PathBuf,
        err: ArchiveExtractError,
    },
    PathMapperConstructError {
        arg_name: &'static str,
        err: PathMapperConstructError,
    },
    ArgumentJsonParseError {
        arg_name: &'static str,
        file_name: Utf8PathBuf,
        err: serde_json::Error,
    },
    CargoMetadataParseError {
        file_name: Option<Utf8PathBuf>,
        err: guppy::Error,
    },
    CreateTestListError {
        err: CreateTestListError,
    },
    BuildFailed {
        command: String,
        exit_code: Option<i32>,
    },
    TestRunFailed,
    #[cfg(feature = "self-update")]
    UpdateVersionParseError {
        err: UpdateVersionParseError,
    },
    #[cfg(feature = "self-update")]
    UpdateError {
        err: UpdateError,
    },
    ExperimentalFeatureNotEnabled {
        name: &'static str,
        var_name: &'static str,
    },
    FilterExpressionParseError {
        all_errors: Vec<FilterExpressionParseErrors>,
    },
    TestBinaryArgsParseError {
        reason: &'static str,
        args: Vec<String>,
    },
}

impl ExpectedError {
    pub(crate) fn cargo_metadata_failed() -> Self {
        Self::CargoMetadataFailed
    }

    pub(crate) fn profile_not_found(err: ProfileNotFound) -> Self {
        Self::ProfileNotFound { err }
    }

    pub(crate) fn config_parse_error(err: ConfigParseError) -> Self {
        Self::ConfigParseError { err }
    }

    pub(crate) fn argument_file_read_error(
        arg_name: &'static str,
        file_name: impl Into<Utf8PathBuf>,
        err: std::io::Error,
    ) -> Self {
        Self::ArgumentFileReadError {
            arg_name,
            file_name: file_name.into(),
            err,
        }
    }

    pub(crate) fn argument_json_parse_error(
        arg_name: &'static str,
        file_name: impl Into<Utf8PathBuf>,
        err: serde_json::Error,
    ) -> Self {
        Self::ArgumentJsonParseError {
            arg_name,
            file_name: file_name.into(),
            err,
        }
    }

    pub(crate) fn cargo_metadata_parse_error(
        file_name: impl Into<Option<Utf8PathBuf>>,
        err: guppy::Error,
    ) -> Self {
        Self::CargoMetadataParseError {
            file_name: file_name.into(),
            err,
        }
    }

    pub(crate) fn experimental_feature_error(name: &'static str, var_name: &'static str) -> Self {
        Self::ExperimentalFeatureNotEnabled { name, var_name }
    }

    pub(crate) fn build_failed(
        command: impl IntoIterator<Item = impl AsRef<str>>,
        exit_code: Option<i32>,
    ) -> Self {
        Self::BuildFailed {
            command: shell_words::join(command),
            exit_code,
        }
    }

    pub(crate) fn filter_expression_parse_error(
        all_errors: Vec<FilterExpressionParseErrors>,
    ) -> Self {
        Self::FilterExpressionParseError { all_errors }
    }

    pub(crate) fn test_run_failed() -> Self {
        Self::TestRunFailed
    }

    pub(crate) fn test_binary_args_parse_error(reason: &'static str, args: Vec<String>) -> Self {
        Self::TestBinaryArgsParseError { reason, args }
    }

    /// Returns the exit code for the process.
    pub fn process_exit_code(&self) -> i32 {
        match self {
            Self::CargoMetadataFailed => NextestExitCode::CARGO_METADATA_FAILED,
            Self::ProfileNotFound { .. }
            | Self::RootManifestNotFound { .. }
            | Self::CargoConfigsConstructError { .. }
            | Self::ConfigParseError { .. }
            | Self::ArgumentFileReadError { .. }
            | Self::UnknownArchiveFormat { .. }
            | Self::ArchiveExtractError { .. }
            | Self::PathMapperConstructError { .. }
            | Self::ArgumentJsonParseError { .. }
            | Self::CargoMetadataParseError { .. }
            | Self::TestBinaryArgsParseError { .. } => NextestExitCode::SETUP_ERROR,
            #[cfg(feature = "self-update")]
            Self::UpdateVersionParseError { .. } => NextestExitCode::SETUP_ERROR,
            Self::CreateTestListError { .. } => NextestExitCode::TEST_LIST_CREATION_FAILED,
            Self::BuildFailed { .. } => NextestExitCode::BUILD_FAILED,
            Self::TestRunFailed => NextestExitCode::TEST_RUN_FAILED,
            Self::ArchiveCreateError { .. } => NextestExitCode::ARCHIVE_CREATION_FAILED,
            #[cfg(feature = "self-update")]
            Self::UpdateError { .. } => NextestExitCode::UPDATE_ERROR,
            Self::ExperimentalFeatureNotEnabled { .. } => {
                NextestExitCode::EXPERIMENTAL_FEATURE_NOT_ENABLED
            }
            Self::FilterExpressionParseError { .. } => NextestExitCode::INVALID_FILTER_EXPRESSION,
        }
    }

    /// Displays this error to stderr.
    pub fn display_to_stderr(&self) {
        let mut next_error = match &self {
            Self::CargoMetadataFailed => {
                // The error produced by `cargo metadata` is enough.
                None
            }
            Self::ProfileNotFound { err } => {
                log::error!("{}", err);
                err.source()
            }
            Self::RootManifestNotFound {
                path,
                reuse_build_kind,
            } => {
                let hint_str = match reuse_build_kind {
                    ReuseBuildKind::ReuseWithWorkspaceRemap { workspace_root } => {
                        format!(
                            "\n(hint: ensure that project source is available at {})",
                            workspace_root.if_supports_color(Stream::Stderr, |x| x.bold())
                        )
                    }
                    ReuseBuildKind::Reuse => {
                        "\n(hint: ensure that project source is available for reused build, \
                          using --workspace-remap if necessary)"
                            .to_owned()
                    }
                    ReuseBuildKind::Normal => String::new(),
                };
                log::error!(
                    "workspace root manifest at {} does not exist{hint_str}",
                    path.if_supports_color(Stream::Stderr, |x| x.bold())
                );
                None
            }
            Self::CargoConfigsConstructError { err } => {
                log::error!("{}", err);
                err.source()
            }
            Self::ConfigParseError { err } => {
                log::error!("{}", err);
                err.source()
            }
            Self::ArgumentFileReadError {
                arg_name,
                file_name,
                err,
            } => {
                log::error!(
                    "argument {} specified file `{}` that couldn't be read",
                    format!("--{}", arg_name).if_supports_color(Stream::Stderr, |x| x.bold()),
                    file_name.if_supports_color(Stream::Stderr, |x| x.bold()),
                );
                Some(err as &dyn Error)
            }
            Self::UnknownArchiveFormat { archive_file, err } => {
                log::error!(
                    "failed to autodetect archive format for {}",
                    archive_file.if_supports_color(Stream::Stderr, |x| x.bold())
                );
                Some(err as &dyn Error)
            }
            Self::ArchiveCreateError { archive_file, err } => {
                log::error!(
                    "error creating archive `{}`",
                    archive_file.if_supports_color(Stream::Stderr, |x| x.bold())
                );
                Some(err as &dyn Error)
            }
            Self::ArchiveExtractError { archive_file, err } => {
                log::error!(
                    "error extracting archive `{}`",
                    archive_file.if_supports_color(Stream::Stderr, |x| x.bold())
                );
                Some(err as &dyn Error)
            }
            Self::ArgumentJsonParseError {
                arg_name,
                file_name,
                err,
            } => {
                log::error!(
                    "argument {} specified JSON file `{}` that couldn't be deserialized",
                    format!("--{}", arg_name).if_supports_color(Stream::Stderr, |x| x.bold()),
                    file_name.if_supports_color(Stream::Stderr, |x| x.bold()),
                );
                Some(err as &dyn Error)
            }
            Self::PathMapperConstructError { arg_name, err } => {
                log::error!(
                    "argument {} specified `{}` that couldn't be read",
                    format!("--{}", arg_name).if_supports_color(Stream::Stderr, |x| x.bold()),
                    err.input().if_supports_color(Stream::Stderr, |x| x.bold())
                );
                Some(err as &dyn Error)
            }
            Self::CargoMetadataParseError { file_name, err } => {
                let metadata_source = match file_name {
                    Some(path) => format!(
                        " from file `{}`",
                        path.if_supports_color(Stream::Stderr, |x| x.bold())
                    ),
                    None => "".to_owned(),
                };
                log::error!("error parsing Cargo metadata{}", metadata_source);
                Some(err as &dyn Error)
            }
            Self::CreateTestListError { err } => {
                log::error!("creating test list failed");
                Some(err as &dyn Error)
            }
            Self::BuildFailed { command, exit_code } => {
                let with_code_str = match exit_code {
                    Some(code) => {
                        format!(
                            " with code {}",
                            code.if_supports_color(Stream::Stderr, |x| x.bold())
                        )
                    }
                    None => "".to_owned(),
                };

                log::error!(
                    "command `{}` exited{}",
                    command.if_supports_color(Stream::Stderr, |x| x.bold()),
                    with_code_str,
                );

                None
            }
            Self::TestRunFailed => {
                log::error!("test run failed");
                None
            }
            #[cfg(feature = "self-update")]
            Self::UpdateVersionParseError { err } => {
                log::error!("failed to parse --version");
                Some(err as &dyn Error)
            }
            #[cfg(feature = "self-update")]
            Self::UpdateError { err } => {
                log::error!(
                    "failed to update nextest (please update manually by visiting <{}>)",
                    "https://get.nexte.st".if_supports_color(Stream::Stderr, |x| x.bold())
                );
                Some(err as &dyn Error)
            }
            Self::ExperimentalFeatureNotEnabled { name, var_name } => {
                log::error!(
                    "{} is an experimental feature and must be enabled with {}=1",
                    name,
                    var_name
                );
                None
            }
            Self::FilterExpressionParseError { all_errors } => {
                for errors in all_errors {
                    for single_error in &errors.errors {
                        let report = miette::Report::new(single_error.clone())
                            .with_source_code(errors.input.to_owned());
                        log::error!(target: "cargo_nextest::no_heading", "{:?}", report);
                    }
                }

                log::error!("failed to parse filter expression");
                None
            }
            Self::TestBinaryArgsParseError { reason, args } => {
                log::error!(
                    "failed to parse test binary arguments `{}`: arguments are {reason}",
                    args.join(", "),
                );
                None
            }
        };

        while let Some(err) = next_error {
            log::error!(target: "cargo_nextest::no_heading", "\nCaused by:\n  {}", err);
            next_error = err.source();
        }
    }
}

impl fmt::Display for ExpectedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // This should generally not be called, but provide a stub implementation if it is
        match self {
            Self::CargoMetadataFailed => writeln!(f, "cargo metadata failed"),
            Self::ProfileNotFound { .. } => writeln!(f, "profile not found"),
            Self::RootManifestNotFound { .. } => writeln!(f, "root manifest not found"),
            Self::CargoConfigsConstructError { .. } => writeln!(f, "CargoConfigs construct error"),
            Self::ConfigParseError { .. } => writeln!(f, "config read error"),
            Self::ArgumentFileReadError { .. } => writeln!(f, "argument file error"),
            Self::UnknownArchiveFormat { .. } => writeln!(f, "unknown archive format"),
            Self::ArchiveCreateError { .. } => writeln!(f, "archive create error"),
            Self::ArchiveExtractError { .. } => writeln!(f, "archive extract error"),
            Self::PathMapperConstructError { .. } => writeln!(f, "path mapper construct error"),
            Self::ArgumentJsonParseError { .. } => writeln!(f, "argument json decode error"),
            Self::CargoMetadataParseError { .. } => writeln!(f, "cargo metadata parse error"),
            Self::CreateTestListError { .. } => writeln!(f, "parse test list error"),
            Self::BuildFailed { .. } => writeln!(f, "build failed"),
            Self::TestRunFailed => writeln!(f, "test run failed"),
            #[cfg(feature = "self-update")]
            Self::UpdateVersionParseError { .. } => writeln!(f, "parsing update version failed"),
            #[cfg(feature = "self-update")]
            Self::UpdateError { .. } => writeln!(f, "update failed"),
            Self::ExperimentalFeatureNotEnabled { .. } => {
                writeln!(f, "experimental feature not enabled")
            }
            Self::FilterExpressionParseError { .. } => {
                writeln!(f, "Failed to parse some filter expressions")
            }
            Self::TestBinaryArgsParseError { .. } => {
                writeln!(f, "test binary arguments parse error")
            }
        }
    }
}

impl error::Error for ExpectedError {}
