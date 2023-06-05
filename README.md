# DevOps Metrics Tools

devops-metrics-tools is a CLI tool developed for obtaining DevOps' Four Keys.

CLI command is `devperf`.

This tool is characterized by the ability to store settings for multiple projects in a configuration file,
and it can measure based on GitHub(deployment/pull request) or Heroku's release information.

## Features

### Supports Multiple Projects
You can store settings for multiple projects in a configuration file. This makes it easy to switch between different projects or environments.

### Works with GitHub and Heroku
This tool can fetch and analyze data based on GitHub's deployment, GitHub's pull request or Heroku's release information.

### Command-Line Interface
All functionality is accessible through a user-friendly command-line interface.

## Installation

```bash
brew install shwld/tap/devperf
```

## Usage

Initialize the configuration file.

```bash
devperf init
```

show performance metrics.

```bash
devperf four-keys --project hoge
```

## Configuration

```bash
# Create after init command
cat ~/.config/devperf/default-config.toml
```

## Contributing
[CONTRIBUTING](/CONTRIBUTING.md)

## License
[LICENSE](/LICENSE.md)

## Support
https://twitter.com/shwld
