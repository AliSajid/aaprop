use axum::extract::Path;
use axum::{http::StatusCode, Json};

use crate::models::AminoAcid;
use crate::responses::{
    AminoAcidAbbreviationResponse, AminoAcidCodonCountResponse, AminoAcidCodonResponse,
    AminoAcidMolecularWeightResponse, AminoAcidNameResponse, AminoAcidResponse,
    AminoAcidShortNameResponse, AminoAcidSideChainResponse, ErrorResponse, RootResponse,
};

fn match_amino_acid(amino_acid: String) -> Option<AminoAcid> {
    let amino_acids = crate::data::amino_acids();
    amino_acids
        .iter()
        .find(|&a| a.get_name().to_lowercase() == amino_acid.to_lowercase())
        .cloned()
}

pub async fn get_root() -> Result<(StatusCode, Json<RootResponse>), Json<ErrorResponse>> {
    let response = RootResponse {
        message: "Welcome to the Amino Acid API".to_string(),
    };
    Ok((StatusCode::OK, Json(response)))
}

pub async fn get_amino_acid(
    Path(amino_acid): Path<String>,
) -> Result<(StatusCode, Json<AminoAcidResponse>), (StatusCode, Json<ErrorResponse>)> {
    let matched: Option<AminoAcid> = match_amino_acid(amino_acid);
    match matched {
        None => {
            let response = ErrorResponse {
                error: "Amino Acid not found".to_string(),
            };
            Err((StatusCode::NOT_FOUND, Json(response)))
        }
        Some(amino_acid) => {
            let response = AminoAcidResponse { amino_acid };
            Ok((StatusCode::OK, Json(response)))
        }
    }
}

pub async fn get_amino_acid_name(
    Path(amino_acid): Path<String>,
) -> Result<(StatusCode, Json<AminoAcidNameResponse>), (StatusCode, Json<ErrorResponse>)> {
    let matched = match_amino_acid(amino_acid);
    match matched {
        None => {
            let response = ErrorResponse {
                error: "Amino Acid not found".to_string(),
            };
            Err((StatusCode::NOT_FOUND, Json(response)))
        }
        Some(amino_acid) => {
            let response = AminoAcidNameResponse {
                name: amino_acid.get_name(),
                short_name: amino_acid.get_short_name(),
                abbreviation: amino_acid.get_abbreviation(),
            };
            Ok((StatusCode::OK, Json(response)))
        }
    }
}

pub async fn get_amino_acid_short_name(
    Path(amino_acid): Path<String>,
) -> Result<(StatusCode, Json<AminoAcidShortNameResponse>), (StatusCode, Json<ErrorResponse>)> {
    let matched = match_amino_acid(amino_acid);
    match matched {
        None => {
            let response = ErrorResponse {
                error: "Amino Acid not found".to_string(),
            };
            Err((StatusCode::NOT_FOUND, Json(response)))
        }
        Some(amino_acid) => {
            let response = AminoAcidShortNameResponse {
                name: amino_acid.get_name(),
                short_name: amino_acid.get_short_name(),
            };
            Ok((StatusCode::OK, Json(response)))
        }
    }
}

pub async fn get_amino_acid_abbreviation(
    Path(amino_acid): Path<String>,
) -> Result<(StatusCode, Json<AminoAcidAbbreviationResponse>), (StatusCode, Json<ErrorResponse>)> {
    let matched = match_amino_acid(amino_acid);
    match matched {
        None => {
            let response = ErrorResponse {
                error: "Amino Acid not found".to_string(),
            };
            Err((StatusCode::NOT_FOUND, Json(response)))
        }
        Some(amino_acid) => {
            let response = AminoAcidAbbreviationResponse {
                name: amino_acid.get_name(),
                abbreviation: amino_acid.get_abbreviation(),
            };
            Ok((StatusCode::OK, Json(response)))
        }
    }
}

pub async fn get_amino_acid_side_chain(
    Path(amino_acid): Path<String>,
) -> Result<(StatusCode, Json<AminoAcidSideChainResponse>), (StatusCode, Json<ErrorResponse>)> {
    let matched = match_amino_acid(amino_acid);
    match matched {
        None => {
            let response = ErrorResponse {
                error: "Amino Acid not found".to_string(),
            };
            Err((StatusCode::NOT_FOUND, Json(response)))
        }
        Some(amino_acid) => {
            let response = AminoAcidSideChainResponse {
                name: amino_acid.get_name(),
                side_chain: amino_acid.get_side_chain().to_string(),
            };
            Ok((StatusCode::OK, Json(response)))
        }
    }
}

pub async fn get_amino_acid_molecular_weight(
    Path(amino_acid): Path<String>,
) -> Result<(StatusCode, Json<AminoAcidMolecularWeightResponse>), (StatusCode, Json<ErrorResponse>)>
{
    let matched = match_amino_acid(amino_acid);
    match matched {
        None => {
            let response = ErrorResponse {
                error: "Amino Acid not found".to_string(),
            };
            Err((StatusCode::NOT_FOUND, Json(response)))
        }
        Some(amino_acid) => {
            let response = AminoAcidMolecularWeightResponse {
                name: amino_acid.get_name(),
                molecular_weight: amino_acid.get_molecular_weight(),
            };
            Ok((StatusCode::OK, Json(response)))
        }
    }
}

pub async fn get_amino_acid_codons(
    Path(amino_acid): Path<String>,
) -> Result<(StatusCode, Json<AminoAcidCodonResponse>), (StatusCode, Json<ErrorResponse>)> {
    let matched = match_amino_acid(amino_acid);
    match matched {
        None => {
            let response = ErrorResponse {
                error: "Amino Acid not found".to_string(),
            };
            Err((StatusCode::NOT_FOUND, Json(response)))
        }
        Some(amino_acid) => {
            let response = AminoAcidCodonResponse {
                name: amino_acid.get_name(),
                codons: amino_acid.get_codons(),
            };
            Ok((StatusCode::OK, Json(response)))
        }
    }
}

pub async fn get_amino_acid_codon_count(
    Path(amino_acid): Path<String>,
) -> Result<(StatusCode, Json<AminoAcidCodonCountResponse>), (StatusCode, Json<ErrorResponse>)> {
    let matched = match_amino_acid(amino_acid);
    match matched {
        None => {
            let response = ErrorResponse {
                error: "Amino Acid not found".to_string(),
            };
            Err((StatusCode::NOT_FOUND, Json(response)))
        }
        Some(amino_acid) => {
            let response = AminoAcidCodonCountResponse {
                name: amino_acid.get_name(),
                codon_count: amino_acid.get_codon_count(),
            };
            Ok((StatusCode::OK, Json(response)))
        }
    }
}
