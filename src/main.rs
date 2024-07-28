use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct ChartYaml {
    #[serde(rename = "apiVersion")]
    api_version: String,
    entries: Vec<ChartEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChartEntry {
    #[serde(rename = "apiVersion")]
    api_version: String,
    #[serde(rename = "appVersion")]
    app_version: String,
    created: String,
    description: String,
    digest: String,
    home: String,
    icon: String,
    keywords: Vec<String>,
    maintainers: Vec<Maintainer>,
    name: String,
    sources: Vec<String>,
    #[serde(rename = "type")]
    entry_type: String,
    urls: Vec<String>,
    version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Maintainer {
    email: String,
    name: String,
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Constants {
    #[serde(rename = "apiVersion")]
    api_version: String,
    #[serde(rename = "appVersion")]
    app_version: String,
    created: String,
    description: String,
    home: String,
    icon: String,
    keywords: Vec<String>,
    maintainers: Vec<Maintainer>,
    name: String,
    sources: Vec<String>,
    #[serde(rename = "type")]
    entry_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Parameters {
    #[serde(rename = "appVersion")]
    app_version: Option<String>,
    digest: String,
    version: String,
    urls: String,
}

fn update_yaml(
    file_path: &str,
    constants: &Constants,
    parameters: &Parameters,
) -> Result<String, Box<dyn std::error::Error>> {
    // Read the main YAML file
    let contents = fs::read_to_string(file_path)?;
    let content = serde_yaml::from_str(&contents);
    let mut data: ChartYaml = match content {
        Ok(data) => data,
        Err(_) => ChartYaml {
            api_version: "v1".to_owned(),
            entries: Vec::new(),
        },
    };

    let created = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
    let urls = Vec::from([parameters.urls.to_owned()]);

    // Create new entry
    let new_entry = ChartEntry {
        api_version: constants.api_version.to_owned(),
        app_version: parameters
            .app_version
            .to_owned()
            .unwrap_or(constants.app_version.to_owned()),
        created: created.to_owned(),
        description: constants.description.to_owned(),
        digest: parameters.digest.to_owned(),
        home: constants.home.to_owned(),
        icon: constants.icon.to_owned(),
        keywords: constants.keywords.to_owned(),
        maintainers: constants.maintainers.to_owned(),
        name: constants.name.to_owned(),
        sources: constants.sources.to_owned(),
        entry_type: constants.entry_type.to_owned(),
        urls,
        version: parameters.version.to_owned(),
    };

    // Add new entry to the keydb list
    data.entries.push(new_entry);

    // Write the updated YAML back to the file
    let updated_yaml = serde_yaml::to_string(&data)?;

    Ok(updated_yaml)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!(
            "Usage: {} <file_path> <constants_path> <parameters_path>",
            args[0]
        );
        std::process::exit(1);
    }

    let file_path = &args[1];
    let constants_path = &args[2];
    let parameters_path = &args[3];

    // Read constants
    let constants_content = fs::read_to_string(constants_path)?;
    let constants: Constants = serde_yaml::from_str(&constants_content)?;

    // Read parameters
    let parameters_content = fs::read_to_string(parameters_path)?;
    let parameters: Parameters = serde_yaml::from_str(&parameters_content)?;

    let updated_yaml = update_yaml(file_path, &constants, &parameters)?;

    fs::write(file_path, updated_yaml)?;
    println!("Added new entry to {}", file_path);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::DateTime;

    #[test]
    fn test_update_yaml() {
        // Create a sample input YAML
        let input_yaml = r#"
apiVersion: v1
entries:
  - apiVersion: v2
    appVersion: "6.3.4"
    created: "2024-04-05T16:03:22.72761377Z"
    description: "Initial description"
    digest: "initial_digest"
    home: "https://initial.com"
    icon: "https://initial.com/icon.png"
    keywords: ["initial"]
    maintainers:
      - email: "initial@example.com"
        name: "Initial Maintainer"
        url: "https://initial.com"
    name: "initial"
    sources: ["https://initial.com/source"]
    type: "application"
    urls: ["https://initial.com/chart.tgz"]
    version: "0.1.0"
"#;

        // Write input YAML to a temporary file
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test.yaml");
        fs::write(&file_path, input_yaml).unwrap();

        // Create sample constants and parameters
        let constants = Constants {
            api_version: "v2".to_string(),
            app_version: "6.3.5".to_string(),
            created: "".to_string(), // This will be overwritten
            description: "Updated description".to_string(),
            home: "https://updated.com".to_string(),
            icon: "https://updated.com/icon.png".to_string(),
            keywords: vec!["updated".to_string()],
            maintainers: vec![Maintainer {
                email: "updated@example.com".to_string(),
                name: "Updated Maintainer".to_string(),
                url: "https://updated.com".to_string(),
            }],
            name: "updated".to_string(),
            sources: vec!["https://updated.com/source".to_string()],
            entry_type: "application".to_string(),
        };

        let parameters = Parameters {
            app_version: Some("6.3.6".to_string()),
            digest: "updated_digest".to_string(),
            version: "0.2.0".to_string(),
            urls: "https://updated.com/chart-0.2.0.tgz".to_string(),
        };

        // Call update_yaml
        let result = update_yaml(file_path.to_str().unwrap(), &constants, &parameters).unwrap();

        // Parse the result
        let updated_yaml: ChartYaml = serde_yaml::from_str(&result).unwrap();

        // Assertions
        assert_eq!(updated_yaml.api_version, "v1");
        assert_eq!(updated_yaml.entries.len(), 2);

        let new_entry = &updated_yaml.entries[1];
        assert_eq!(new_entry.api_version, "v2");
        assert_eq!(new_entry.app_version, "6.3.6");
        assert_eq!(new_entry.description, "Updated description");
        assert_eq!(new_entry.digest, "updated_digest");
        assert_eq!(new_entry.home, "https://updated.com");
        assert_eq!(new_entry.icon, "https://updated.com/icon.png");
        assert_eq!(new_entry.keywords, vec!["updated"]);
        assert_eq!(new_entry.maintainers[0].email, "updated@example.com");
        assert_eq!(new_entry.name, "updated");
        assert_eq!(new_entry.sources, vec!["https://updated.com/source"]);
        assert_eq!(new_entry.entry_type, "application");
        assert_eq!(new_entry.urls, vec!["https://updated.com/chart-0.2.0.tgz"]);
        assert_eq!(new_entry.version, "0.2.0");

        // Check that the created field is a valid timestamp
        assert!(DateTime::parse_from_rfc3339(&new_entry.created).is_ok());
    }
}
