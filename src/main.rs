use chrono::Utc;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_yaml::{Mapping, Value};
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Debug, Serialize, Deserialize)]
struct ChartYaml {
    #[serde(rename = "apiVersion")]
    api_version: Option<String>,
    entries: Mapping,
}

impl Default for ChartYaml {
    fn default() -> Self {
        Self {
            api_version: Some("v1".to_string()),
            entries: Mapping::new(),
        }
    }
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

impl Default for Constants {
    fn default() -> Self {
        Self {
            api_version: "v2".to_string(),
            app_version: "1.0.0".to_string(),
            description: "Test Chart".to_string(),
            home: "https://example.com".to_string(),
            icon: "https://example.com/icon.png".to_string(),
            keywords: vec!["test".to_string(), "chart".to_string()],
            maintainers: vec![Maintainer {
                email: "test@example.com".to_string(),
                name: "Abdulrhman Alkhodiry".to_string(),
                url: "https://example.com".to_string(),
            }],
            name: "test-chart".to_string(),
            sources: vec!["https://github.com/test/chart".to_string()],
            entry_type: "application".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Parameters {
    #[serde(rename = "appVersion")]
    app_version: Option<String>,
    digest: String,
    version: String,
    urls: Vec<String>,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            app_version: None,
            digest: "abc123".to_string(),
            version: "0.1.0".to_string(),
            urls: vec!["https://example.com/test-chart-0.1.0.tgz".to_string()],
        }
    }
}

fn update_yaml(
    file_path: &str,
    constants: &Constants,
    parameters: &Parameters,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut data: ChartYaml = if Path::new(file_path).exists() {
        let contents = fs::read_to_string(file_path)?;

        let contents = if contents.trim().is_empty() {
            "apiVersion: v1\nentries: {}\n"
        } else {
            &contents
        };
        serde_yaml::from_str(&contents)?
    } else {
        ChartYaml {
            api_version: Some("v1".to_owned()),
            entries: Mapping::new(),
        }
    };

    let created = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();

    let new_entry = ChartEntry {
        api_version: constants.api_version.clone(),
        app_version: parameters
            .app_version
            .clone()
            .unwrap_or_else(|| constants.app_version.clone()),
        created,
        description: constants.description.clone(),
        digest: parameters.digest.clone(),
        home: constants.home.clone(),
        icon: constants.icon.clone(),
        keywords: constants.keywords.clone(),
        maintainers: constants.maintainers.clone(),
        name: constants.name.clone(),
        sources: constants.sources.clone(),
        entry_type: constants.entry_type.clone(),
        urls: parameters.urls.clone(),
        version: parameters.version.clone(),
    };

    let entries_key = Value::String(constants.name.clone());
    let entries = data
        .entries
        .entry(entries_key)
        .or_insert(Value::Sequence(Vec::new()));

    if let Value::Sequence(ref mut vec) = entries {
        vec.push(serde_yaml::to_value(&new_entry)?);
    } else {
        return Err("Unexpected value type for entries".into());
    }

    serde_yaml::to_string(&data).map_err(Into::into)
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Update the YAML file
    Update {
        /// Path to the YAML file to update
        #[arg(short, long)]
        file: PathBuf,

        /// Path to the constants YAML file
        #[arg(short, long)]
        constants: PathBuf,

        /// Path to the parameters YAML file
        #[arg(short, long)]
        parameters: PathBuf,
    },
    /// Generate a new YAML file templates
    Generate {},
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Update {
            file,
            constants,
            parameters,
        } => {
            let constants: Constants = serde_yaml::from_str(&fs::read_to_string(constants)?)?;
            let parameters: Parameters = serde_yaml::from_str(&fs::read_to_string(parameters)?)?;

            let updated_yaml = update_yaml(file.to_str().unwrap(), &constants, &parameters)?;
            fs::write(file, updated_yaml)?;

            println!("Added new entry to {}", file.display());
        }
        Commands::Generate {} => {
            let mut file = File::create("index.yaml")?;
            let mut constants_file = File::create("constants.yaml")?;
            let mut parameters_file = File::create("parameters.yaml")?;

            let _ = file.write(serde_yaml::to_string(&ChartYaml::default())?.as_bytes());
            let _ =
                constants_file.write(serde_yaml::to_string(&Constants::default())?.as_bytes())?;
            let _ =
                parameters_file.write(serde_yaml::to_string(&Parameters::default())?.as_bytes())?;
            println!("YAML templates generated");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_test_constants() -> Constants {
        Constants {
            api_version: "v2".to_string(),
            app_version: "1.0.0".to_string(),
            description: "Test Chart".to_string(),
            home: "https://example.com".to_string(),
            icon: "https://example.com/icon.png".to_string(),
            keywords: vec!["test".to_string(), "chart".to_string()],
            maintainers: vec![Maintainer {
                email: "test@example.com".to_string(),
                name: "Test Maintainer".to_string(),
                url: "https://example.com".to_string(),
            }],
            name: "test-chart".to_string(),
            sources: vec!["https://github.com/test/chart".to_string()],
            entry_type: "application".to_string(),
        }
    }

    fn create_test_parameters() -> Parameters {
        Parameters {
            app_version: Some("1.0.1".to_string()),
            digest: "abc123".to_string(),
            version: "0.1.0".to_string(),
            urls: vec!["https://example.com/test-chart-0.1.0.tgz".to_string()],
        }
    }

    #[test]
    fn test_update_yaml_new_file() -> Result<(), Box<dyn std::error::Error>> {
        let temp_file = NamedTempFile::new()?;
        let file_path = temp_file.path().to_str().unwrap();

        let constants = create_test_constants();
        let parameters = create_test_parameters();

        let updated_yaml = update_yaml(file_path, &constants, &parameters)?;
        let parsed: ChartYaml = serde_yaml::from_str(&updated_yaml)?;

        assert_eq!(parsed.api_version, Some("v1".to_string()));
        assert_eq!(parsed.entries.len(), 1);

        let entries = parsed
            .entries
            .get(&Value::String("test-chart".to_string()))
            .unwrap();
        let entries: Vec<ChartEntry> = serde_yaml::from_value(entries.clone())?;
        assert_eq!(entries.len(), 1);

        let entry = &entries[0];
        assert_eq!(entry.api_version, "v2");
        assert_eq!(entry.app_version, "1.0.1");
        assert_eq!(entry.description, "Test Chart");
        assert_eq!(entry.digest, "abc123");
        assert_eq!(entry.version, "0.1.0");

        Ok(())
    }

    #[test]
    fn test_update_yaml_existing_file() -> Result<(), Box<dyn std::error::Error>> {
        let mut temp_file = NamedTempFile::new()?;

        let initial_content = r#"
apiVersion: v1
entries:
  test-chart:
    - apiVersion: v2
      appVersion: 1.0.0
      created: "2023-01-01T00:00:00.000Z"
      description: Initial Test Chart
      digest: def456
      home: https://example.com
      icon: https://example.com/icon.png
      keywords:
        - test
        - chart
      maintainers:
        - email: test@example.com
          name: Test Maintainer
          url: https://example.com
      name: test-chart
      sources:
        - https://github.com/test/chart
      type: application
      urls:
        - https://example.com/test-chart-0.0.1.tgz
      version: 0.0.1
"#;
        write!(temp_file, "{}", initial_content)?;
        let file_path = temp_file.path().to_str().unwrap();

        let constants = create_test_constants();
        let parameters = create_test_parameters();

        let updated_yaml = update_yaml(file_path, &constants, &parameters)?;
        let parsed: ChartYaml = serde_yaml::from_str(&updated_yaml)?;

        assert_eq!(parsed.api_version, Some("v1".to_string()));
        assert_eq!(parsed.entries.len(), 1);

        let entries = parsed
            .entries
            .get(&Value::String("test-chart".to_string()))
            .unwrap();
        let entries: Vec<ChartEntry> = serde_yaml::from_value(entries.clone())?;
        assert_eq!(entries.len(), 2);

        let new_entry = &entries[1];
        assert_eq!(new_entry.api_version, "v2");
        assert_eq!(new_entry.app_version, "1.0.1");
        assert_eq!(new_entry.description, "Test Chart");
        assert_eq!(new_entry.digest, "abc123");
        assert_eq!(new_entry.version, "0.1.0");

        Ok(())
    }
}
