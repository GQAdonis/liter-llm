use std::fs;
use std::path::PathBuf;

use clap::Args;
use liter_llm_proxy::config::ProxyConfig;
use serde_json::json;

#[derive(Args)]
pub struct ConfigCheckArgs {
    /// Path to the proxy configuration to validate.
    #[arg(long, short)]
    pub config: PathBuf,
}

#[expect(clippy::print_stdout, reason = "machine-readable CLI command result")]
pub fn run(args: ConfigCheckArgs) -> Result<(), String> {
    let result = match fs::read_to_string(&args.config) {
        Ok(source) => match ProxyConfig::from_toml_str(&source) {
            Ok(_) => json!({ "valid": true }),
            Err(error) => json!({
                "valid": false,
                "code": "invalid_config",
                "message": safe_error_summary(&error),
            }),
        },
        Err(error) => json!({
            "valid": false,
            "code": "read_failed",
            "message": error.kind().to_string(),
        }),
    };

    println!("{result}");
    Ok(())
}

fn safe_error_summary(error: &str) -> &str {
    error.lines().next().unwrap_or("invalid liter-llm configuration")
}
