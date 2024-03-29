// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

// use aaprop::create_router;

use aaprop::create_router;
use shuttle_axum::ShuttleAxum;

#[shuttle_runtime::main]
async fn axum() -> ShuttleAxum {
    let router = create_router();

    Ok(router.into())
}
