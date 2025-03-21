// SPDX-FileCopyrightText: 2023 - 2025 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use cfg_block::cfg_block;

cfg_block! {

    if #[cfg(all(feature = "standalone", not(feature = "shuttle")))] {
        mod cli;

        use aaprop::create_router;
        use clap::Parser;
        use cli::Cli;
        use tracing;
        use tracing_subscriber::FmtSubscriber;

        #[tokio::main]
        pub async fn main() {

            let subscriber = FmtSubscriber::builder()
                .with_max_level(tracing::Level::INFO)
                .finish();

            if tracing::subscriber::set_global_default(subscriber).is_err() {
                eprintln!("Failed to set subscriber");
                std::process::exit(3);
            }

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
    } else {
        use aaprop::create_router;
        use shuttle_axum::ShuttleAxum;


        #[allow(clippy::unused_async)]
        #[shuttle_runtime::main]
        pub async fn app() -> ShuttleAxum {
            //! This is a server.
            //!
            //! # Errors
            //!
            //! If the server fails to start, an error will be returned.
            let router = create_router();

            Ok(router.into())
        }

    }

}
