use super::{WizardError, from_inquire};
use crate::server;

pub fn prompt_auth(url: &str, verbose: bool) -> Result<String, WizardError> {
    println!("\n── Authentication ──\n");

    let options = vec![
        "Username & password (obtain API key automatically)",
        "Manual API key (paste from server dashboard)",
    ];
    let choice = inquire::Select::new("How to authenticate:", options)
        .prompt()
        .map_err(from_inquire)?;

    let api_key = if choice.starts_with("Username") {
        prompt_username_password(url, verbose)?
    } else {
        prompt_manual_api_key()?
    };

    // Validate the key works
    if verbose {
        eprintln!("Validating API key...");
    }
    let server_type = server::detect_server_type(url)
        .map_err(|e| WizardError::ServerUnreachable(e.to_string()))?;
    let client = server::MediaServerClient::new(url.to_string(), api_key.clone(), server_type);
    match client.get_user_id() {
        Ok(_) => println!("  Authentication successful."),
        Err(e) => {
            return Err(WizardError::AuthFailed(format!(
                "API key validation failed (GET /Users): {e}"
            )));
        }
    }

    Ok(api_key)
}

fn prompt_username_password(url: &str, verbose: bool) -> Result<String, WizardError> {
    const MAX_RETRIES: usize = 3;

    for attempt in 1..=MAX_RETRIES {
        let username = inquire::Text::new("Username:")
            .prompt()
            .map_err(from_inquire)?;

        let password = inquire::Password::new("Password:")
            .without_confirmation()
            .prompt()
            .map_err(from_inquire)?;

        if verbose {
            eprintln!("Authenticating as '{username}'...");
        }

        match server::authenticate_by_name(url, &username, &password) {
            Ok(token) => {
                println!("  Obtained API key via authentication.");
                return Ok(token);
            }
            Err(e) => {
                eprintln!("  Authentication failed (attempt {attempt}/{MAX_RETRIES}): {e}");
                if attempt == MAX_RETRIES {
                    eprintln!("  Falling back to manual API key entry.");
                    return prompt_manual_api_key();
                }
            }
        }
    }
    unreachable!()
}

fn prompt_manual_api_key() -> Result<String, WizardError> {
    let key = inquire::Text::new("API key:")
        .with_help_message("Find this in your server's dashboard under API Keys")
        .prompt()
        .map_err(from_inquire)?;

    let key = key.trim().to_string();
    if key.is_empty() {
        return Err(WizardError::AuthFailed(
            "API key cannot be empty".to_string(),
        ));
    }
    Ok(key)
}
