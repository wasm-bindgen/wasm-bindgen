//! Diagnostic messages for unsupported or partially-supported TS constructs.

use std::path::{Path, PathBuf};

/// Source location for a diagnostic.
#[derive(Clone, Debug)]
pub struct SourceLocation {
    pub file: PathBuf,
    pub line: u32,
    pub col: u32,
}

impl std::fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.file.display(), self.line, self.col)
    }
}

/// A diagnostic emitted during parsing or code generation.
#[derive(Clone, Debug)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub message: String,
    /// Optional source location.
    pub location: Option<SourceLocation>,
    /// Optional: the TS source text that triggered this diagnostic.
    pub source_text: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiagnosticLevel {
    /// A type could not be resolved — the output may be incorrect.
    Error,
    /// Something was skipped or simplified.
    Warning,
    /// Informational — the tool made a decision the user should know about.
    Info,
}

/// Collects diagnostics during a parse/codegen session.
///
/// Call `set_file` before processing each file. `warn_at`/`error_at` accept
/// byte offsets and automatically compute line:col from the source.
#[derive(Clone, Debug, Default)]
pub struct DiagnosticCollector {
    pub diagnostics: Vec<Diagnostic>,
    /// The file currently being processed, with its source text.
    current_file: Option<PathBuf>,
    current_source: Option<String>,
    /// Pre-computed byte offsets of each line start (0-indexed) for O(log n) line lookup.
    line_offsets: Vec<u32>,
}

impl DiagnosticCollector {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the current file and its source text for subsequent diagnostics.
    /// Builds a line offset table for O(log n) offset-to-line lookup.
    pub fn set_file(&mut self, path: &Path, source: &str) {
        self.current_file = Some(path.to_path_buf());
        // Build line offset table: line_offsets[i] = byte offset of start of line i+1.
        let mut offsets = vec![0u32];
        for (i, b) in source.bytes().enumerate() {
            if b == b'\n' {
                offsets.push((i + 1) as u32);
            }
        }
        self.line_offsets = offsets;
        self.current_source = Some(source.to_string());
    }

    /// Emit a warning with optional line/column.
    pub fn warn(&mut self, message: impl Into<String>) {
        self.diagnostics.push(Diagnostic {
            level: DiagnosticLevel::Warning,
            message: message.into(),
            location: None,
            source_text: None,
        });
    }

    /// Emit a warning at a byte offset in the current file.
    pub fn warn_at(&mut self, message: impl Into<String>, offset: u32) {
        let location = self.make_location(offset);
        self.diagnostics.push(Diagnostic {
            level: DiagnosticLevel::Warning,
            message: message.into(),
            location,
            source_text: None,
        });
    }

    pub fn warn_with_source(&mut self, message: impl Into<String>, source: impl Into<String>) {
        self.diagnostics.push(Diagnostic {
            level: DiagnosticLevel::Warning,
            message: message.into(),
            location: None,
            source_text: Some(source.into()),
        });
    }

    pub fn info(&mut self, message: impl Into<String>) {
        self.diagnostics.push(Diagnostic {
            level: DiagnosticLevel::Info,
            message: message.into(),
            location: None,
            source_text: None,
        });
    }

    pub fn error(&mut self, message: impl Into<String>) {
        self.diagnostics.push(Diagnostic {
            level: DiagnosticLevel::Error,
            message: message.into(),
            location: None,
            source_text: None,
        });
    }

    /// Emit an error at a byte offset in the current file.
    pub fn error_at(&mut self, message: impl Into<String>, offset: u32) {
        let location = self.make_location(offset);
        self.diagnostics.push(Diagnostic {
            level: DiagnosticLevel::Error,
            message: message.into(),
            location,
            source_text: None,
        });
    }

    fn make_location(&self, offset: u32) -> Option<SourceLocation> {
        let file = self.current_file.as_ref()?;
        let (line, col) = self.offset_to_line_col(offset);
        Some(SourceLocation {
            file: file.clone(),
            line,
            col,
        })
    }

    /// Print all diagnostics to stderr, deduplicating identical messages.
    pub fn emit(&self) {
        let mut seen = std::collections::HashSet::new();
        for diag in &self.diagnostics {
            // Dedupe key: level + message (ignore location for dedup)
            let key = format!("{:?}:{}", diag.level, diag.message);
            if !seen.insert(key) {
                continue;
            }
            let prefix = match diag.level {
                DiagnosticLevel::Error => "error",
                DiagnosticLevel::Warning => "warning",
                DiagnosticLevel::Info => "info",
            };
            if let Some(ref loc) = diag.location {
                eprintln!("[ts-gen {prefix}]: {loc}: {}", diag.message);
            } else {
                eprintln!("[ts-gen {prefix}]: {}", diag.message);
            }
            if let Some(ref src) = diag.source_text {
                eprintln!("  source: {src}");
            }
        }
    }

    /// Compute line and column from a byte offset using the pre-built line offset table.
    /// Returns (line, col) where both are 1-indexed.
    /// Uses binary search for O(log n) lookup.
    fn offset_to_line_col(&self, offset: u32) -> (u32, u32) {
        if self.line_offsets.is_empty() {
            return (1, 1);
        }
        // Binary search: find the last line_offset <= offset
        let line_idx = match self.line_offsets.binary_search(&offset) {
            Ok(i) => i,
            Err(i) => i.saturating_sub(1),
        };
        let line = (line_idx as u32) + 1;
        let col = offset - self.line_offsets[line_idx] + 1;
        (line, col)
    }

    pub fn has_warnings(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.level == DiagnosticLevel::Warning)
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.level == DiagnosticLevel::Error)
    }
}
