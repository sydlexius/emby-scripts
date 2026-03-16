// Rating types and functions are consumed by main.rs workflows.
// Remove this allow after wiring workflows in Task 8.
#![allow(dead_code)]

pub mod action;
pub mod scope;

#[cfg(test)]
mod tests;

use crate::server::MediaServerError;

/// Outcome of processing a single track.
#[derive(Debug, Clone, PartialEq)]
pub enum RatingAction {
    /// Rating was applied to the server.
    Set,
    /// Rating was removed (set to empty string).
    Cleared,
    /// Track was skipped (already rated + skip-existing, or no action needed).
    Skipped,
    /// Rating already matches the desired value.
    AlreadyCorrect,
    /// Dry-run: would have set a rating.
    DryRun,
    /// Dry-run: would have cleared a rating.
    DryRunClear,
    /// Server update failed (non-auth error).
    Error(String),
}

impl RatingAction {
    /// CSV-friendly string representation.
    pub fn as_csv_str(&self) -> &str {
        match self {
            Self::Set => "set",
            Self::Cleared => "cleared",
            Self::Skipped => "skipped",
            Self::AlreadyCorrect => "already_correct",
            Self::DryRun => "dry_run",
            Self::DryRunClear => "dry_run_clear",
            Self::Error(_) => "error",
        }
    }
}

/// Why a track was rated.
#[derive(Debug, Clone, PartialEq)]
pub enum Source {
    /// Rating determined by lyrics classification.
    Lyrics,
    /// Rating determined by genre allow-list (G).
    Genre,
    /// Force subcommand or config force_rating.
    Force,
    /// Reset subcommand.
    Reset,
}

impl Source {
    /// CSV-friendly string representation.
    pub fn as_csv_str(&self) -> &str {
        match self {
            Self::Lyrics => "lyrics",
            Self::Genre => "genre",
            Self::Force => "force",
            Self::Reset => "reset",
        }
    }
}

/// Result of processing a single audio track.
#[derive(Debug)]
pub struct ItemResult {
    pub item_id: String,
    pub path: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub tier: Option<String>,
    pub matched_words: Vec<String>,
    pub previous_rating: Option<String>,
    pub action: RatingAction,
    pub source: Source,
    pub server_name: String,
}

/// Resolved library/location scope for a workflow.
#[derive(Debug)]
pub struct LibraryScope {
    /// ParentId for prefetch query (None = all items).
    pub parent_id: Option<String>,
    /// Server-side path prefix for post-prefetch location filtering.
    pub location_path: Option<String>,
    /// Resolved library name (for force_rating lookup).
    pub library_name: Option<String>,
}

/// Errors that abort a workflow.
#[derive(Debug)]
pub enum RatingError {
    /// Server API error (non-auth).
    Server(MediaServerError),
    /// Auth error (401/403) — abort immediately.
    Auth(u16),
    /// Requested library not found.
    LibraryNotFound {
        name: String,
        available: Vec<String>,
    },
    /// Requested location not found.
    LocationNotFound {
        name: String,
        available: Vec<String>,
    },
    /// No music libraries found on server.
    NoMusicLibraries,
    /// Library matched but has no ItemId.
    MissingLibraryId(String),
}

impl std::fmt::Display for RatingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Server(e) => write!(f, "{e}"),
            Self::Auth(status) => write!(f, "auth error (HTTP {status})"),
            Self::LibraryNotFound { name, available } => {
                write!(
                    f,
                    "library '{}' not found. Available: {}",
                    name,
                    available.join(", ")
                )
            }
            Self::LocationNotFound { name, available } => {
                write!(
                    f,
                    "location '{}' not found. Available: {}",
                    name,
                    available.join(", ")
                )
            }
            Self::NoMusicLibraries => write!(f, "no music libraries found on server"),
            Self::MissingLibraryId(name) => {
                write!(f, "library '{}' has no ItemId", name)
            }
        }
    }
}

impl std::error::Error for RatingError {}

impl From<MediaServerError> for RatingError {
    fn from(e: MediaServerError) -> Self {
        match &e {
            MediaServerError::Http { status, .. } if *status == 401 || *status == 403 => {
                Self::Auth(*status)
            }
            _ => Self::Server(e),
        }
    }
}
