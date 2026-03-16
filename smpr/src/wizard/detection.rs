use super::{WizardError, from_inquire};
use crate::config::defaults;

/// Result of the detection rules step.
pub struct DetectionAdditions {
    pub extra_r_stems: Vec<String>,
    pub extra_r_exact: Vec<String>,
    pub extra_pg13_stems: Vec<String>,
    pub extra_pg13_exact: Vec<String>,
    pub extra_false_positives: Vec<String>,
}

pub fn prompt_detection(_verbose: bool) -> Result<DetectionAdditions, WizardError> {
    println!("\n── Detection Rules ──\n");

    println!("  R-tier stems:      {}", defaults::R_STEMS.join(", "));
    println!("  R-tier exact:      {}", defaults::R_EXACT.join(", "));
    println!("  PG-13 stems:       {}", defaults::PG13_STEMS.join(", "));
    println!("  PG-13 exact:       {}", defaults::PG13_EXACT.join(", "));
    println!(
        "  False positives:   {}",
        defaults::FALSE_POSITIVES.join(", ")
    );
    println!();

    let use_defaults = inquire::Confirm::new("Use these defaults?")
        .with_default(true)
        .prompt()
        .map_err(from_inquire)?;

    if use_defaults {
        return Ok(DetectionAdditions {
            extra_r_stems: vec![],
            extra_r_exact: vec![],
            extra_pg13_stems: vec![],
            extra_pg13_exact: vec![],
            extra_false_positives: vec![],
        });
    }

    println!("\n  Enter additional words (comma-separated, Enter to skip).\n");

    let extra_r_stems = prompt_csv("Additional R-tier stems:")?;
    let extra_r_exact = prompt_csv("Additional R-tier exact words:")?;
    let extra_pg13_stems = prompt_csv("Additional PG-13 stems:")?;
    let extra_pg13_exact = prompt_csv("Additional PG-13 exact words:")?;
    let extra_false_positives = prompt_csv("Additional false positives:")?;

    Ok(DetectionAdditions {
        extra_r_stems,
        extra_r_exact,
        extra_pg13_stems,
        extra_pg13_exact,
        extra_false_positives,
    })
}

fn prompt_csv(message: &str) -> Result<Vec<String>, WizardError> {
    let input = inquire::Text::new(message)
        .with_help_message("Comma-separated, or press Enter to skip")
        .with_default("")
        .prompt()
        .map_err(from_inquire)?;

    let items: Vec<String> = input
        .split(',')
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect();

    Ok(items)
}
