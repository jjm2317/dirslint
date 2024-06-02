# dirslint

The linter for directory structure convention written in Rust

## Supporting Features
- Directory & File name Linting 🚀
- Wildcard search with glob expressions 😄

## Installation

```shell
# npm
npm install -D dirslint

# yarn
yarn add --dev dirslint

```

## Usage 
```shell

# with npx
npx dirslint --config example.yml

# set dirslint scripts in package.json
yarn dirslint --config example.yml
npm dirslint --config example.yml
```

## Examples
```yml
ds:
  # Rules under specific directory
  src/**/*: ["app", "components", "*.ts" ]

  # Example for Fractal Directory Structure
  "**/hooks/*.ts": ["use*.ts" ]


# Linting Target
target:
- src/* # All files & directories under the src directory

# Ignore specific folders & files
ignore:
- node_modules*
- .env*
- .git*
- .git/**
```
