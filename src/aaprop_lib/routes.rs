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
    models::AminoAcid,
    responses::{
        AminoAcidAbbreviationResponse,
        AminoAcidCodonCountResponse,
        AminoAcidCodonResponse,
        AminoAcidMolecularWeightResponse,
        AminoAcidNameResponse,
        AminoAcidResponse,
        AminoAcidShortNameResponse,
        AminoAcidSideChainResponse,
        ErrorResponse,
        RootResponse,
    },
};

#[instrument]
fn match_amino_acid(amino_acid: &String) -> Option<AminoAcid> {
    event!(Level::INFO, "Matching amino acid: {}", amino_acid);
    let amino_acids = crate::data::amino_acids();
    amino_acids
        .iter()
        .find(|&a| a.get_name().to_lowercase() == amino_acid.to_lowercase())
        .cloned()
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
) -> Result<(StatusCode, Json<AminoAcidResponse>), (StatusCode, Json<ErrorResponse>)> {
    event!(Level::INFO, "GET /amino_acid/{} called", &amino_acid);
    let matched: Option<AminoAcid> = match_amino_acid(&amino_acid);
    match matched {
        None => {
            let response = ErrorResponse {
                error: "Amino Acid not found".to_string(),
            };
            event!(Level::INFO, "Amino Acid {} not found", &amino_acid);
            event!(Level::DEBUG, "Response: {:?}", &response);
            Err((StatusCode::NOT_FOUND, Json(response)))
        }
        Some(amino_acid) => {
            event!(Level::INFO, "Amino Acid {} found", &amino_acid.get_name());
            let response = AminoAcidResponse { amino_acid };
            event!(Level::DEBUG, "Response: {:?}", &response);
            Ok((StatusCode::OK, Json(response)))
        }
    }
}

#[instrument]
pub async fn get_amino_acid_name(
    Path(amino_acid): Path<String>,
) -> Result<(StatusCode, Json<AminoAcidNameResponse>), (StatusCode, Json<ErrorResponse>)> {
    event!(Level::INFO, "GET /amino_acid/{}/name called", &amino_acid);
    let matched = match_amino_acid(&amino_acid);
    match matched {
        None => {
            let response = ErrorResponse {
                error: "Amino Acid not found".to_string(),
            };
            event!(Level::INFO, "Amino Acid {} not found", &amino_acid);
            Err((StatusCode::NOT_FOUND, Json(response)))
        }
        Some(amino_acid) => {
            event!(Level::INFO, "Amino Acid {} found", &amino_acid.get_name());
            let response = AminoAcidNameResponse {
                name:         amino_acid.get_name(),
                short_name:   amino_acid.get_short_name(),
                abbreviation: amino_acid.get_abbreviation(),
            };
            event!(Level::DEBUG, "Response: {:?}", &response);
            Ok((StatusCode::OK, Json(response)))
        }
    }
}

#[instrument]
pub async fn get_amino_acid_short_name(
    Path(amino_acid): Path<String>,
) -> Result<(StatusCode, Json<AminoAcidShortNameResponse>), (StatusCode, Json<ErrorResponse>)> {
    event!(
        Level::INFO,
        "GET /amino_acid/{}/short_name called",
        &amino_acid
    );
    let matched = match_amino_acid(&amino_acid);
    match matched {
        None => {
            let response = ErrorResponse {
                error: "Amino Acid not found".to_string(),
            };
            event!(Level::INFO, "Amino Acid {} not found", &amino_acid);
            event!(Level::DEBUG, "Response: {:?}", &response);
            Err((StatusCode::NOT_FOUND, Json(response)))
        }
        Some(amino_acid) => {
            event!(Level::INFO, "Amino Acid {} found", &amino_acid.get_name());
            let response = AminoAcidShortNameResponse {
                name:       amino_acid.get_name(),
                short_name: amino_acid.get_short_name(),
            };
            event!(Level::DEBUG, "Response: {:?}", &response);
            Ok((StatusCode::OK, Json(response)))
        }
    }
}

#[instrument]
pub async fn get_amino_acid_abbreviation(
    Path(amino_acid): Path<String>,
) -> Result<(StatusCode, Json<AminoAcidAbbreviationResponse>), (StatusCode, Json<ErrorResponse>)> {
    event!(
        Level::INFO,
        "GET /amino_acid/{}/abbreviation called",
        &amino_acid
    );
    let matched = match_amino_acid(&amino_acid);
    match matched {
        None => {
            let response = ErrorResponse {
                error: "Amino Acid not found".to_string(),
            };
            event!(Level::INFO, "Amino Acid {} not found", &amino_acid);
            event!(Level::DEBUG, "Response: {:?}", &response);
            Err((StatusCode::NOT_FOUND, Json(response)))
        }
        Some(amino_acid) => {
            event!(Level::INFO, "Amino Acid {} found", &amino_acid.get_name());
            let response = AminoAcidAbbreviationResponse {
                name:         amino_acid.get_name(),
                abbreviation: amino_acid.get_abbreviation(),
            };
            event!(Level::DEBUG, "Response: {:?}", &response);
            Ok((StatusCode::OK, Json(response)))
        }
    }
}

#[instrument]
pub async fn get_amino_acid_side_chain(
    Path(amino_acid): Path<String>,
) -> Result<(StatusCode, Json<AminoAcidSideChainResponse>), (StatusCode, Json<ErrorResponse>)> {
    event!(
        Level::INFO,
        "GET /amino_acid/{}/side_chain called",
        &amino_acid
    );
    let matched = match_amino_acid(&amino_acid);
    match matched {
        None => {
            let response = ErrorResponse {
                error: "Amino Acid not found".to_string(),
            };
            event!(Level::INFO, "Amino Acid {} not found", &amino_acid);
            event!(Level::DEBUG, "Response: {:?}", &response);
            Err((StatusCode::NOT_FOUND, Json(response)))
        }
        Some(amino_acid) => {
            event!(Level::INFO, "Amino Acid {} found", &amino_acid.get_name());
            let response = AminoAcidSideChainResponse {
                name:       amino_acid.get_name(),
                side_chain: amino_acid.get_side_chain().to_string(),
            };
            event!(Level::DEBUG, "Response: {:?}", &response);
            Ok((StatusCode::OK, Json(response)))
        }
    }
}

#[instrument]
pub async fn get_amino_acid_molecular_weight(
    Path(amino_acid): Path<String>,
) -> Result<(StatusCode, Json<AminoAcidMolecularWeightResponse>), (StatusCode, Json<ErrorResponse>)>
{
    event!(
        Level::INFO,
        "GET /amino_acid/{}/molecular_weight called",
        &amino_acid
    );
    let matched = match_amino_acid(&amino_acid);
    match matched {
        None => {
            let response = ErrorResponse {
                error: "Amino Acid not found".to_string(),
            };
            event!(Level::INFO, "Amino Acid {} not found", &amino_acid);
            event!(Level::DEBUG, "Response: {:?}", &response);
            Err((StatusCode::NOT_FOUND, Json(response)))
        }
        Some(amino_acid) => {
            event!(Level::INFO, "Amino Acid {} found", &amino_acid.get_name());
            let response = AminoAcidMolecularWeightResponse {
                name:             amino_acid.get_name(),
                molecular_weight: amino_acid.get_molecular_weight(),
            };
            event!(Level::DEBUG, "Response: {:?}", &response);
            Ok((StatusCode::OK, Json(response)))
        }
    }
}

#[instrument]
pub async fn get_amino_acid_codons(
    Path(amino_acid): Path<String>,
) -> Result<(StatusCode, Json<AminoAcidCodonResponse>), (StatusCode, Json<ErrorResponse>)> {
    event!(Level::INFO, "GET /amino_acid/{}/codons called", &amino_acid);
    let matched = match_amino_acid(&amino_acid);
    match matched {
        None => {
            let response = ErrorResponse {
                error: "Amino Acid not found".to_string(),
            };
            event!(Level::INFO, "Amino Acid {} not found", &amino_acid);
            event!(Level::DEBUG, "Response: {:?}", &response);
            Err((StatusCode::NOT_FOUND, Json(response)))
        }
        Some(amino_acid) => {
            event!(Level::INFO, "Amino Acid {} found", &amino_acid.get_name());
            let response = AminoAcidCodonResponse {
                name:   amino_acid.get_name(),
                codons: amino_acid.get_codons(),
            };
            event!(Level::DEBUG, "Response: {:?}", &response);
            Ok((StatusCode::OK, Json(response)))
        }
    }
}

#[instrument]
pub async fn get_amino_acid_codon_count(
    Path(amino_acid): Path<String>,
) -> Result<(StatusCode, Json<AminoAcidCodonCountResponse>), (StatusCode, Json<ErrorResponse>)> {
    event!(
        Level::INFO,
        "GET /amino_acid/{}/codon_count called",
        &amino_acid
    );
    let matched = match_amino_acid(&amino_acid);
    match matched {
        None => {
            let response = ErrorResponse {
                error: "Amino Acid not found".to_string(),
            };
            event!(Level::INFO, "Amino Acid {} not found", &amino_acid);
            event!(Level::DEBUG, "Response: {:?}", &response);
            Err((StatusCode::NOT_FOUND, Json(response)))
        }
        Some(amino_acid) => {
            event!(Level::INFO, "Amino Acid {} found", &amino_acid.get_name());
            let response = AminoAcidCodonCountResponse {
                name:        amino_acid.get_name(),
                codon_count: amino_acid.get_codon_count(),
            };
            event!(Level::DEBUG, "Response: {:?}", &response);
            Ok((StatusCode::OK, Json(response)))
        }
    }
}
