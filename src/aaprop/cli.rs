// SPDX-FileCopyrightText: 2023 - 2025 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "aaprop-server", version, about)]
pub struct Cli {
    #[arg(
        short,
        long,
        default_value = "127.0.0.1",
        env = "BIND_ADDRESS",
        value_name = "LISTEN_ADDRESS"
    )]
    pub bind: String,
    #[arg(
        short,
        long,
        default_value = "8080",
        env = "BIND_PORT",
        value_name = "LISTEN_PORT"
    )]
    pub port: u16,
    #[arg(
        short,
        long,
        short,
        long,
        default_value = "info",
        env = "LOG_LEVEL",
        value_name = "LOG_LEVEL"
    )]
    pub log: LogLevel,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}
