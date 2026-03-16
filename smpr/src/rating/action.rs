use crate::rating::{RatingAction, RatingError};
use crate::server::MediaServerError;

/// Decide what action to take for setting a rating.
///
/// Pure logic — no server calls. Returns the action to take.
/// When `Set` is returned, the caller must perform the server round-trip.
pub fn decide_rating_action(
    tier: &str,
    current_rating: Option<&str>,
    overwrite: bool,
    dry_run: bool,
) -> RatingAction {
    // Already at the desired rating?
    if current_rating.is_some_and(|r| r == tier) {
        return RatingAction::AlreadyCorrect;
    }
    // Skip if has existing rating and overwrite is false
    if !overwrite && current_rating.is_some_and(|r| !r.is_empty()) {
        return RatingAction::Skipped;
    }
    if dry_run {
        return RatingAction::DryRun;
    }
    RatingAction::Set
}

/// Decide what action to take for clearing a rating.
///
/// Used when lyrics are clean but a track has an existing rating (overwrite mode).
pub fn decide_clear_action(
    current_rating: Option<&str>,
    overwrite: bool,
    dry_run: bool,
) -> RatingAction {
    // No rating to clear
    if current_rating.is_none() || current_rating.is_some_and(|r| r.is_empty()) {
        return RatingAction::Skipped;
    }
    // Skip-existing mode: don't touch rated tracks
    if !overwrite {
        return RatingAction::Skipped;
    }
    if dry_run {
        return RatingAction::DryRunClear;
    }
    RatingAction::Cleared
}

/// GET-then-POST round-trip to set OfficialRating on an item.
///
/// Returns `Ok(RatingAction)` on success or non-auth failure.
/// Returns `Err(RatingError::Auth)` on 401/403 so the caller can abort the workflow.
pub fn apply_rating(
    client: &crate::server::MediaServerClient,
    item_id: &str,
    rating: &str,
    label: &str,
) -> Result<RatingAction, RatingError> {
    match apply_rating_inner(client, item_id, rating) {
        Ok(()) => {
            if rating.is_empty() {
                log::info!("cleared rating from {}", label);
                Ok(RatingAction::Cleared)
            } else {
                log::info!("set {} on {}", rating, label);
                Ok(RatingAction::Set)
            }
        }
        Err(MediaServerError::Http { status, .. }) if status == 401 || status == 403 => {
            Err(RatingError::Auth(status))
        }
        Err(e) => {
            log::error!("failed to update {}: {}", label, e);
            Ok(RatingAction::Error(e.to_string()))
        }
    }
}

fn apply_rating_inner(
    client: &crate::server::MediaServerClient,
    item_id: &str,
    rating: &str,
) -> Result<(), MediaServerError> {
    let mut item = client.get_item(item_id)?;
    item["OfficialRating"] = serde_json::Value::String(rating.to_string());
    client.update_item(item_id, &item)?;
    Ok(())
}
