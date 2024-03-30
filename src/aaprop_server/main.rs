// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

mod cli;

use aaprop::create_router;
use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let bind_addr = format!("{}:{}", cli.bind, cli.port);

    let router = create_router();

    let listener = match tokio::net::TcpListener::bind(&bind_addr).await {
        Ok(listener) => {
            println!("Server started at http://{}", &bind_addr);
            listener
        }
        Err(e) => {
            eprintln!("Failed to start server: {}", e);
            std::process::exit(1);
        }
    };

    match axum::serve(listener, router).await {
        Ok(_) => {
            println!("Listening on: http://{}", &bind_addr);
        }
        Err(e) => {
            eprintln!("Server error: {}", e);
            std::process::exit(2);
        }
    }
}
