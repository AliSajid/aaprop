<!--
SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami

SPDX-License-Identifier: Apache-2.0
SPDX-License-Identifier: MIT
-->

# Amino Acid Properties

![GitHub Release (w/pre-release)](https://img.shields.io/github/v/release/AliSajid/aaprop?include_prereleases&logo=semantic-release)
![GitHub Release](https://img.shields.io/github/v/release/AliSajid/aaprop?logo=semantic-release)
[![Continuous integration](https://github.com/AliSajid/aaprop/actions/workflows/ci.yaml/badge.svg)](https://github.com/AliSajid/aaprop/actions/workflows/ci.yaml)
[![Security Audit](https://github.com/AliSajid/aaprop/actions/workflows/audit.yaml/badge.svg?branch=main)](https://github.com/AliSajid/aaprop/actions/workflows/audit.yaml)

![GitHub issues](https://img.shields.io/github/issues/AliSajid/aaprop)
![REUSE Compliance](https://img.shields.io/reuse/compliance/github.com%2FAliSajid%2Faaprop)

## Introduction

This project is a small server app that provides a REST API to access the properties of amino acids. The project is written in Rust and uses the `actix-web` framework to handle the HTTP requests. The project is in its initial stages of development and is not yet ready for production use. The project is also designed with dual-deployment in mind. It can either be deployed to [Shuttle](https://shuttle.dev) or to a traditional cloud provider. For traditional cloud providers, we provide both a binary and a Docker image.

## Build Status

| Rust Version  | Build Status                                                                                                                                                       |
| ------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Stable        | ![Ubuntu x Stable Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/625c1d8a63a7cbb926f8828d97d850b1/raw/ubuntu-stable.json)   |
| Beta          | ![Ubuntu x Beta Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/625c1d8a63a7cbb926f8828d97d850b1/raw/ubuntu-beta.json)       |
| Nightly       | ![Ubuntu x Nightly Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/625c1d8a63a7cbb926f8828d97d850b1/raw/ubuntu-nightly.json) |
| MSRV (1.78.0) | ![Ubuntu x MSRV Rust](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/AliSajid/625c1d8a63a7cbb926f8828d97d850b1/raw/ubuntu-msrv.json)       |

**Note: We have stopped testing builds on Windows and macos for expediency. We recommend you use the docker image should you need to run the project on these platforms.**

## Roadmap

- [x] Create the standard project
- [x] Add the necessary meta files
- [x] Create the models for the data structures
- [x] Add the Amino Acid Properties Data
- [x] Set up the appropriate routes
- [x] Restructure the project
- [x] Add the tracing
- [x] Add a CLI interface

## Contributing

Contributions to the project are welcome. Please see the [Contributing Guidelines](CONTRIBUTING.md) for more information.

## License

This project is dual-licensed under the [MIT License](LICENSES/MIT.txt) and the [Apache License (Version 2.0)](LICENSES/Apache-2.0.txt). You may choose to use this project under either license, at your discretion. Other, ancillary files are under the [CC0 License](LICENSES/CC0-1.0.txt) and are dedicated to the Public Domain. Please see the [LICENSES](LICENSES) directory for more information.

This project is REUSE compliant. You can find more information about REUSE [here](https://reuse.software/).

## Code of Conduct

[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](CODE_OF_CONDUCT.md)

<!-- vale write-good.Passive = NO -->
<!-- vale Google.Passive = NO -->

This project adheres to the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, You are expected to uphold this code.

<!-- vale Google.Passive = YES -->
<!-- vale write-good.Passive = YES -->
