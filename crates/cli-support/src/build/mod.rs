use crate::build::progressbar::ProgressOutput;

pub mod binary;
pub mod emoji;
pub mod lockfile;
pub mod manifest;
mod progressbar;
pub mod target;
pub mod utils;
pub mod versions;

/// The global progress bar and user-facing message output.
pub static PBAR: ProgressOutput = ProgressOutput::new();

/// The build profile controls whether optimizations, debug info, and assertions
/// are enabled or disabled.
#[derive(Clone, Debug)]
pub enum BuildProfile {
    /// Enable assertions and debug info. Disable optimizations.
    Dev,
    /// Enable optimizations. Disable assertions and debug info.
    Release,
    /// Enable optimizations and debug info. Disable assertions.
    Profiling,
    /// User-defined profile with --profile flag
    Custom(String),
}
