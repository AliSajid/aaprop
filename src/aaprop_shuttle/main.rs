// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use aaprop::create_router;

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = create_router();

    Ok(router.into())
}

#[allow(dead_code)]
fn server() {
    println!("This is a server.")
}
