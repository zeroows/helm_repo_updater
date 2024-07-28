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

```bash
cargo run -- path/to/chart.yaml path/to/constants.yaml path/to/parameters.yaml
```
