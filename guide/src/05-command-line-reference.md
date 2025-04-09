<!--
SPDX-FileCopyrightText: 2023 - 2025 Ali Sajid Imami

SPDX-License-Identifier: Apache-2.0
SPDX-License-Identifier: MIT
-->

# Command Line Reference

This section provides a reference for the command-line interface (CLI) of the `AAProp` API.

## Usage

```bash
aaprop [FLAGS] [OPTIONS]
```

## Flags

- `-h`, `--help`: Prints help information.
- `-V`, `--version`: Prints version information.

## Options

- `-b`, `--bind LISTEN_ADDRESS`: The IP address to bind the server to. Default: `127.0.0.1`.
- `-p`, `--port LISTEN_PORT`: The port number to listen on. Default: `8080`.
- `-l`, `--log LOG_LEVEL`: The log level. Default: `info`. Possible values: `trace`, `debug`, `info`, `warn`, `error`.

## Examples

### Start the `AAProp` API on the default address and port

This command starts the `AAProp` API on `localhost:8080`. You can access the API at [`http://localhost:8080`](http://localhost:8080).

```bash
aaprop
```

### Start the `AAProp` API on a custom address and port

You can customize one, both, or none of the address and port options.

#### Custom Address, Custom Port

```bash
aaprop --bind 0.0.0.0 --port 8081
```

#### Custom Address. Default Port

```bash
aaprop --bind 0.0.0.0
```

#### Default Address, Custom Port

```bash
aaprop --port 8081
```

### Start the `AAProp` API with a custom log level

You can set the log level to `trace`, `debug`, `info`, `warn`, or `error`.

```bash
aaprop --log debug
```
