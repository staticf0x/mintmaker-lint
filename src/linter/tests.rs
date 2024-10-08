use crate::linter::error::LinterError;
use crate::managers;
use anyhow::Result;

/// Test that the instance is not using any managers that are not
/// enabled in MintMaker
pub fn test_available_managers(
    mm_config: &serde_json::Value,
    instance: &serde_json::Value,
) -> Result<Vec<LinterError>> {
    let available_managers = managers::get_available_managers();
    let enabled_managers: Vec<String> = mm_config["enabledManagers"]
        .as_array()
        .unwrap()
        .into_iter()
        .map(|value| value.as_str().unwrap().to_string())
        .collect();

    let mut errors: Vec<LinterError> = Vec::new();

    for key in instance.as_object().unwrap().keys() {
        if available_managers.contains(&key) {
            if !enabled_managers.contains(&key) {
                errors.push(LinterError::UseOfUnsupportedManager {
                    manager: key.clone(),
                });
            }
        }
    }
    Ok(errors)
}

pub fn test_extends_mintmaker(
    mm_config: &serde_json::Value,
    instance: &serde_json::Value,
) -> Result<Vec<LinterError>> {
    let expected_extend =
        String::from("github>konflux-ci/mintmaker//config/renovate/renovate.json");

    match instance.get("extends") {
        Some(value) => {
            let extend_list: Vec<String> = value
                .as_array()
                .unwrap()
                .into_iter()
                .map(|value| value.as_str().unwrap().to_string())
                .collect();

            if !extend_list.contains(&expected_extend) {
                return Ok(vec![LinterError::DoesntExtendMintMaker]);
            }
        }
        None => return Ok(vec![LinterError::DoesntExtendMintMaker]),
    }

    Ok(vec![])
}
