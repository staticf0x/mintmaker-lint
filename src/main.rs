use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use log::{error, info, LevelFilter};
use std::path::PathBuf;

mod linter;
mod managers;

use linter::{error::LinterError, tests};

const RENOVATE_SCHEMA_URL: &str = "https://docs.renovatebot.com/renovate-schema.json";
const MINTMAKER_CONFIG_URL: &str =
    "https://raw.githubusercontent.com/konflux-ci/mintmaker/main/config/renovate/renovate.json";

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// File name to lint
    file: Option<PathBuf>,
}

/// Fetch the Renovat schema
fn get_renovate_schema() -> Result<serde_json::Value> {
    info!("Fetching Renovate schema");

    Ok(ureq::get(RENOVATE_SCHEMA_URL).call()?.into_json()?)
}

/// Fetch MintMaker's base config
fn get_mintmaker_config() -> Result<serde_json::Value> {
    info!("Fetching MintMaker config");

    Ok(ureq::get(MINTMAKER_CONFIG_URL).call()?.into_json()?)
}

fn read_config_to_json(config: &PathBuf) -> Result<serde_json::Value> {
    info!(
        "Reading Renovate config from {}",
        config.as_path().to_str().unwrap()
    );

    let content: serde_json::Value =
        serde_json::from_str(std::fs::read_to_string(config)?.as_str())?;

    Ok(content)
}

/// JSON-schema validation, should happen before any additional tests
fn validate_schema(schema: serde_json::Value, instance: &serde_json::Value) -> Result<bool> {
    let validator = jsonschema::validator_for(&schema)?;
    let result = validator.validate(&instance);

    match result {
        Ok(_) => Ok(true),
        Err(errors) => {
            for error in errors {
                error!("Validation error: {} at {}", error, error.instance_path);
            }
            Ok(false)
        }
    }
}

fn main() -> Result<()> {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .format_target(false)
        .init();

    let cli = Cli::parse();
    let schema = get_renovate_schema();

    if let Err(e) = schema {
        error!("Cannot download Renovate schema: {}", e.to_string());
        std::process::exit(1);
    }

    let mm_config = get_mintmaker_config();

    if let Err(e) = mm_config {
        error!("Cannot download MintMaker config: {}", e.to_string());
        std::process::exit(2);
    }

    let config_file = cli.file.unwrap_or(PathBuf::from("renovate.json"));

    if !config_file.exists() {
        error!(
            "File {} not found",
            config_file.as_path().to_str().unwrap().bold()
        );
        std::process::exit(3);
    }

    let instance = read_config_to_json(&config_file)?;

    match validate_schema(schema.unwrap(), &instance) {
        Ok(valid) => {
            if valid {
                info!("Config valid according to schema");
            } else {
                error!("Config didn't pass schema validation");
                std::process::exit(128);
            }
        }
        Err(e) => {
            error!("Error validating config: {}", e.to_string());
            std::process::exit(4);
        }
    }

    let tests = vec![
        tests::test_available_managers,
        tests::test_extends_mintmaker,
    ];

    let mut errors: Vec<LinterError> = Vec::new();
    let mm_config = mm_config.unwrap();

    for test in tests {
        let test_errors = test(&mm_config, &instance)?;
        errors.extend(test_errors);
    }

    if !errors.is_empty() {
        for error in errors {
            error!("{}", error);
        }

        std::process::exit(128);
    }

    info!("✨ All checks passed ✨");

    Ok(())
}
