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

fn create_router_v1() -> Router {
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
}

pub fn create_router() -> Router {
    Router::new().nest("/v1", create_router_v1())
}

#[cfg(test)]
mod tests {
    use axum_test::TestServer;
    use rstest::{
        fixture,
        rstest,
    };
    use shuttle_runtime::tokio;

    use super::*;

    #[fixture]
    fn test_server_with_versioning() -> TestServer {
        TestServer::new(create_router()).unwrap()
    }

    #[fixture]
    fn test_server_without_versioning() -> TestServer {
        TestServer::new(create_router_v1()).unwrap()
    }
    #[rstest]
    #[case::root("/")]
    #[case::amino_acid("/amino_acid")]
    #[case::amino_acid_name("/amino_acid/Alanine")]
    #[case::amino_acid_short_name("/amino_acid/Ala")]
    #[case::amino_acid_abbreviation_name("/amino_acid/A")]
    #[case::amino_acid_side_chain("/amino_acid/Alanine/side_chain")]
    #[case::amino_acid_molecular_weight("/amino_acid/Alanine/molecular_weight")]
    #[case::amino_acid_codons("/amino_acid/Alanine/codon")]
    #[case::amino_acid_codon_count("/amino_acid/Alanine/codon_count")]
    #[tokio::test]
    async fn test_create_router(test_server_without_versioning: TestServer, #[case] url: &str) {
        let response = test_server_without_versioning.get(url).await;
        assert_eq!(response.status_code(), StatusCode::OK);
    }

    #[rstest]
    #[case::amino_acid("/v1/amino_acid")]
    #[case::amino_acid_name("/v1/amino_acid/Alanine")]
    #[case::amino_acid_short_name("/v1/amino_acid/Ala")]
    #[case::amino_acid_abbreviation_name("/v1/amino_acid/A")]
    #[case::amino_acid_side_chain("/v1/amino_acid/Alanine/side_chain")]
    #[case::amino_acid_molecular_weight("/v1/amino_acid/Alanine/molecular_weight")]
    #[case::amino_acid_codons("/v1/amino_acid/Alanine/codon")]
    #[case::amino_acid_codon_count("/v1/amino_acid/Alanine/codon_count")]
    #[tokio::test]
    async fn test_create_router_with_version(
        test_server_with_versioning: TestServer,
        #[case] url: &str,
    ) {
        let response = test_server_with_versioning.get(url).await;
        assert_eq!(response.status_code(), StatusCode::OK);
    }
}
