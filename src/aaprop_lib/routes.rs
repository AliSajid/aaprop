// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use axum::{
    extract::Path,
    http::StatusCode,
    Json,
};
use tracing::{
    event,
    instrument,
    Level,
};

use crate::{
    find_by_abbreviation,
    find_by_name,
    find_by_short_name,
    responses::{
        AminoAcidDetailResponse,
        ErrorResponse,
        RootResponse,
    },
    AminoAcid,
};

#[instrument]
fn match_amino_acid(amino_acid: &str) -> Option<AminoAcid> {
    event!(Level::INFO, "Matching amino acid: {}", &amino_acid);
    find_by_name(amino_acid)
        .or_else(|| find_by_short_name(amino_acid))
        .or_else(|| find_by_abbreviation(amino_acid))
}

#[instrument]
pub async fn get_root() -> Result<(StatusCode, Json<RootResponse>), Json<ErrorResponse>> {
    event!(Level::INFO, "GET / called");
    let response = RootResponse {
        message: "Welcome to the Amino Acid API".to_string(),
    };
    event!(Level::DEBUG, "Response: {:?}", response);
    Ok((StatusCode::OK, Json(response)))
}

#[instrument]
pub async fn get_amino_acid(
    Path(amino_acid): Path<String>,
) -> Result<(StatusCode, Json<AminoAcidDetailResponse>), (StatusCode, Json<ErrorResponse>)> {
    event!(Level::INFO, "GET /amino_acid/{} called", &amino_acid);
    match_amino_acid(&amino_acid).map_or_else(
        || {
            let response = ErrorResponse {
                error: "Amino Acid not found".to_string(),
            };
            event!(Level::INFO, "Amino Acid {} not found", &amino_acid);
            event!(Level::DEBUG, "Response: {:?}", &response);
            Err((StatusCode::NOT_FOUND, Json(response)))
        },
        |amino_acid| {
            event!(Level::INFO, "Amino Acid {} found", &amino_acid.get_name());
            let response = AminoAcidDetailResponse {
                name:             Some(amino_acid.get_name().to_owned()),
                short_name:       Some(amino_acid.get_short_name().to_owned()),
                abbreviation:     Some(amino_acid.get_abbreviation().to_owned()),
                side_chain:       Some(amino_acid.get_side_chain().to_string()),
                molecular_weight: Some(amino_acid.get_molecular_weight()),
                codons:           Some(
                    amino_acid
                        .get_codons()
                        .into_iter()
                        .filter(|s| !s.is_empty())
                        .map(std::string::ToString::to_string)
                        .collect(),
                ),
                codon_count:      Some(amino_acid.get_codon_count()),
            };
            event!(Level::DEBUG, "Response: {:?}", &response);
            Ok((StatusCode::OK, Json(response)))
        },
    )
}

#[instrument]
pub async fn get_amino_acid_name(
    Path(amino_acid): Path<String>,
) -> Result<(StatusCode, Json<AminoAcidDetailResponse>), (StatusCode, Json<ErrorResponse>)> {
    event!(Level::INFO, "GET /amino_acid/{}/name called", &amino_acid);
    match_amino_acid(&amino_acid).map_or_else(
        || {
            let response = ErrorResponse {
                error: "Amino Acid not found".to_string(),
            };
            event!(Level::INFO, "Amino Acid {} not found", &amino_acid);
            event!(Level::DEBUG, "Response: {:?}", &response);
            Err((StatusCode::NOT_FOUND, Json(response)))
        },
        |amino_acid| {
            event!(Level::INFO, "Amino Acid {} found", &amino_acid.get_name());
            let response = AminoAcidDetailResponse {
                name:             Some(amino_acid.get_name().to_owned()),
                short_name:       Some(amino_acid.get_short_name().to_owned()),
                abbreviation:     Some(amino_acid.get_abbreviation().to_owned()),
                side_chain:       None,
                molecular_weight: None,
                codons:           None,
                codon_count:      None,
            };
            event!(Level::DEBUG, "Response: {:?}", &response);
            Ok((StatusCode::OK, Json(response)))
        },
    )
}

#[instrument]
pub async fn get_amino_acid_short_name(
    Path(amino_acid): Path<String>,
) -> Result<(StatusCode, Json<AminoAcidDetailResponse>), (StatusCode, Json<ErrorResponse>)> {
    event!(
        Level::INFO,
        "GET /amino_acid/{}/short_name called",
        &amino_acid
    );
    match_amino_acid(&amino_acid).map_or_else(
        || {
            let response = ErrorResponse {
                error: "Amino Acid not found".to_string(),
            };
            event!(Level::INFO, "Amino Acid {} not found", &amino_acid);
            event!(Level::DEBUG, "Response: {:?}", &response);
            Err((StatusCode::NOT_FOUND, Json(response)))
        },
        |amino_acid| {
            event!(Level::INFO, "Amino Acid {} found", &amino_acid.get_name());
            let response = AminoAcidDetailResponse {
                name:             Some(amino_acid.get_name().to_owned()),
                short_name:       Some(amino_acid.get_short_name().to_owned()),
                abbreviation:     Some(amino_acid.get_abbreviation().to_owned()),
                side_chain:       None,
                molecular_weight: None,
                codons:           None,
                codon_count:      None,
            };
            event!(Level::DEBUG, "Response: {:?}", &response);
            Ok((StatusCode::OK, Json(response)))
        },
    )
}

#[instrument]
pub async fn get_amino_acid_abbreviation(
    Path(amino_acid): Path<String>,
) -> Result<(StatusCode, Json<AminoAcidDetailResponse>), (StatusCode, Json<ErrorResponse>)> {
    event!(
        Level::INFO,
        "GET /amino_acid/{}/abbreviation called",
        &amino_acid
    );
    match_amino_acid(&amino_acid).map_or_else(
        || {
            let response = ErrorResponse {
                error: "Amino Acid not found".to_string(),
            };
            event!(Level::INFO, "Amino Acid {} not found", &amino_acid);
            event!(Level::DEBUG, "Response: {:?}", &response);
            Err((StatusCode::NOT_FOUND, Json(response)))
        },
        |amino_acid| {
            event!(Level::INFO, "Amino Acid {} found", &amino_acid.get_name());
            let response = AminoAcidDetailResponse {
                name:             Some(amino_acid.get_name().to_owned()),
                short_name:       Some(amino_acid.get_short_name().to_owned()),
                abbreviation:     Some(amino_acid.get_abbreviation().to_owned()),
                side_chain:       None,
                molecular_weight: None,
                codons:           None,
                codon_count:      None,
            };
            event!(Level::DEBUG, "Response: {:?}", &response);
            Ok((StatusCode::OK, Json(response)))
        },
    )
}

#[instrument]
pub async fn get_amino_acid_side_chain(
    Path(amino_acid): Path<String>,
) -> Result<(StatusCode, Json<AminoAcidDetailResponse>), (StatusCode, Json<ErrorResponse>)> {
    event!(
        Level::INFO,
        "GET /amino_acid/{}/side_chain called",
        &amino_acid
    );
    match_amino_acid(&amino_acid).map_or_else(
        || {
            let response = ErrorResponse {
                error: "Amino Acid not found".to_string(),
            };
            event!(Level::INFO, "Amino Acid {} not found", &amino_acid);
            event!(Level::DEBUG, "Response: {:?}", &response);
            Err((StatusCode::NOT_FOUND, Json(response)))
        },
        |amino_acid| {
            event!(Level::INFO, "Amino Acid {} found", &amino_acid.get_name());
            let response = AminoAcidDetailResponse {
                name:             Some(amino_acid.get_name().to_owned()),
                side_chain:       Some(amino_acid.get_side_chain().to_string()),
                short_name:       Some(amino_acid.get_short_name().to_owned()),
                abbreviation:     Some(amino_acid.get_abbreviation().to_owned()),
                molecular_weight: None,
                codons:           None,
                codon_count:      None,
            };
            event!(Level::DEBUG, "Response: {:?}", &response);
            Ok((StatusCode::OK, Json(response)))
        },
    )
}

#[instrument]
pub async fn get_amino_acid_molecular_weight(
    Path(amino_acid): Path<String>,
) -> Result<(StatusCode, Json<AminoAcidDetailResponse>), (StatusCode, Json<ErrorResponse>)> {
    event!(
        Level::INFO,
        "GET /amino_acid/{}/molecular_weight called",
        &amino_acid
    );
    match_amino_acid(&amino_acid).map_or_else(
        || {
            let response = ErrorResponse {
                error: "Amino Acid not found".to_string(),
            };
            event!(Level::INFO, "Amino Acid {} not found", &amino_acid);
            event!(Level::DEBUG, "Response: {:?}", &response);
            Err((StatusCode::NOT_FOUND, Json(response)))
        },
        |amino_acid| {
            event!(Level::INFO, "Amino Acid {} found", &amino_acid.get_name());
            let response = AminoAcidDetailResponse {
                name:             Some(amino_acid.get_name().to_owned()),
                molecular_weight: Some(amino_acid.get_molecular_weight()),
                short_name:       Some(amino_acid.get_short_name().to_owned()),
                abbreviation:     Some(amino_acid.get_abbreviation().to_owned()),
                side_chain:       None,
                codons:           None,
                codon_count:      None,
            };
            event!(Level::DEBUG, "Response: {:?}", &response);
            Ok((StatusCode::OK, Json(response)))
        },
    )
}

#[instrument]
pub async fn get_amino_acid_codons(
    Path(amino_acid): Path<String>,
) -> Result<(StatusCode, Json<AminoAcidDetailResponse>), (StatusCode, Json<ErrorResponse>)> {
    event!(Level::INFO, "GET /amino_acid/{}/codons called", &amino_acid);
    match_amino_acid(&amino_acid).map_or_else(
        || {
            let response = ErrorResponse {
                error: "Amino Acid not found".to_string(),
            };
            event!(Level::INFO, "Amino Acid {} not found", &amino_acid);
            event!(Level::DEBUG, "Response: {:?}", &response);
            Err((StatusCode::NOT_FOUND, Json(response)))
        },
        |amino_acid| {
            event!(Level::INFO, "Amino Acid {} found", &amino_acid.get_name());
            let response = AminoAcidDetailResponse {
                name:             Some(amino_acid.get_name().to_owned()),
                codons:           Some(
                    amino_acid
                        .get_codons()
                        .iter()
                        .map(std::string::ToString::to_string)
                        .collect(),
                ),
                short_name:       Some(amino_acid.get_short_name().to_owned()),
                abbreviation:     Some(amino_acid.get_abbreviation().to_owned()),
                side_chain:       None,
                molecular_weight: None,
                codon_count:      None,
            };
            event!(Level::DEBUG, "Response: {:?}", &response);
            Ok((StatusCode::OK, Json(response)))
        },
    )
}

#[instrument]
pub async fn get_amino_acid_codon_count(
    Path(amino_acid): Path<String>,
) -> Result<(StatusCode, Json<AminoAcidDetailResponse>), (StatusCode, Json<ErrorResponse>)> {
    event!(
        Level::INFO,
        "GET /amino_acid/{}/codon_count called",
        &amino_acid
    );
    match_amino_acid(&amino_acid).map_or_else(
        || {
            let response = ErrorResponse {
                error: "Amino Acid not found".to_string(),
            };
            event!(Level::INFO, "Amino Acid {} not found", &amino_acid);
            event!(Level::DEBUG, "Response: {:?}", &response);
            Err((StatusCode::NOT_FOUND, Json(response)))
        },
        |amino_acid| {
            event!(Level::INFO, "Amino Acid {} found", &amino_acid.get_name());
            let response = AminoAcidDetailResponse {
                name:             Some(amino_acid.get_name().to_owned()),
                codon_count:      Some(amino_acid.get_codon_count()),
                short_name:       Some(amino_acid.get_short_name().to_owned()),
                abbreviation:     Some(amino_acid.get_abbreviation().to_owned()),
                side_chain:       None,
                molecular_weight: None,
                codons:           None,
            };
            event!(Level::DEBUG, "Response: {:?}", &response);
            Ok((StatusCode::OK, Json(response)))
        },
    )
}
