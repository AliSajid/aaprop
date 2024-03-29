// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

mod data;
mod models;
mod responses;
mod routes;

use axum::{
    http::StatusCode,
    routing::get,
    Router,
};
pub use data::{
    find_by_abbreviation,
    find_by_name,
    find_by_short_name,
    AMINO_ACID_DATA,
};
pub use models::{
    AminoAcid,
    CodonSet,
    SideChain,
};

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(routes::get_root))
        .route("/amino_acid", get(routes::get_root))
        .route("/amino_acid/:amino_acid", get(routes::get_amino_acid))
        .route(
            "/amino_acid/:amino_acid/name",
            get(routes::get_amino_acid_name),
        )
        .route(
            "/amino_acid/:amino_acid/short_name",
            get(routes::get_amino_acid_short_name),
        )
        .route(
            "/amino_acid/:amino_acid/abbreviation",
            get(routes::get_amino_acid_abbreviation),
        )
        .route(
            "/amino_acid/:amino_acid/side_chain",
            get(routes::get_amino_acid_side_chain),
        )
        .route(
            "/amino_acid/:amino_acid/molecular_weight",
            get(routes::get_amino_acid_molecular_weight),
        )
        .route(
            "/amino_acid/:amino_acid/codon",
            get(routes::get_amino_acid_codons),
        )
        .route(
            "/amino_acid/:amino_acid/codon_count",
            get(routes::get_amino_acid_codon_count),
        )
        .route("/health", get(|| async { StatusCode::OK }))
        .route("/coffeemaker", get(|| async { StatusCode::GONE }))
}
