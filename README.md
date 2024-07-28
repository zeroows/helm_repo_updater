# YAML Chart Updater

This Rust application updates YAML files containing chart entries. It's designed
to add new entries to an existing YAML file based on provided constants and
parameters.

## Features

- Read and parse existing YAML files
- Add new entries to the YAML structure
- Combine constant values with dynamic parameters
- Automatically set creation timestamp

## Usage

To use this application, you need to provide:

1. Path to the YAML file to be updated
2. Constants file path
3. Parameters file path

Example:

to update a file `index.yaml` with constants from `constants.yaml` and
parameters from `parameters.yaml`:

```bash
helm_repo_updater update --file index.yaml --constants constants.yaml --parameters parameters.yaml
```

to generate a template for the files needed

```bash
helm_repo_updater generate
```
