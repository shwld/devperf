# DevOps Metrics Tools

devops-metrics-tools is a CLI tool developed for obtaining DevOps' Four Keys.

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

```bash
devperf four-keys --project hoge
```

## TODO: Configuration
Explain the structure and options of the configuration file in detail. Explain how users can configure multiple projects and set up the tool to use either GitHub or Heroku.

## TODO: Examples
Include a few examples of how the tool can be used, such as fetching data for a project, switching between projects, or analyzing fetched data.

## TODO: Contributing
If you're open to contributions, provide instructions on how others can contribute to the project.

## TODO: License
Specify the license under which your project is distributed.

## TODO: Support
Provide information on how users can get help with the tool if they encounter issues. This could include linking to an issue tracker, providing your email, etc.

## TODO: Roadmap
If you have plans for future updates or features, you can list them here.
