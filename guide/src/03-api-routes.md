<!--
SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami

SPDX-License-Identifier: Apache-2.0
SPDX-License-Identifier: MIT
-->

# API Routes

This section provides a reference for the API routes of the `AAProp` API.

## Routes

### Get Amino Acid Information

This route returns information about an amino acid.

- **URL**: `/amino_acid/{amino_acid}`
- **Method**: `GET`
- **URL Parameters**:
  - `amino_acid`: The name, three-letter code, or one-letter code of the amino acid.
- **Example**:

```bash
curl https://aaprop.shuttleapp.rs/amino_acid/Alanine
```

```json
{
  "amino_acid": {
    "name": "Alanine",
    "short_name": "Ala",
    "abbreviation": "A",
    "side_chain": "Nonpolar",
    "molecular_weight": 89.09,
    "codons": [
      "GCT",
      "GCC",
      "GCA",
      "GCG"
    ]
  }
}
```

### Get Amino Acid Side Chain Information

This route returns information about the side chain of an amino acid.

- **URL**: `/amino_acid/{amino_acid}/side_chain`
- **Method**: `GET`
- **URL Parameters**:
  - `amino_acid`: The name, three-letter code, or one-letter code of the amino acid.
- **Example**:

```bash
curl https://aaprop.shuttleapp.rs/amino_acid/Alanine/side_chain
```

```json
{
  "amino_acid": {
    "name": "Alanine",
    "side_chain": "Nonpolar",
  }
}
```

### Get Amino Acid Molecular Weight

This route returns the molecular weight of an amino acid.

- **URL**: `/amino_acid/{amino_acid}/molecular_weight`
- **Method**: `GET`
- **URL Parameters**:
  - `amino_acid`: The name, three-letter code, or one-letter code of the amino acid.
- **Example**:

```bash
curl https://aaprop.shuttleapp.rs/amino_acid/Alanine/molecular_weight
```

```json
{
  "amino_acid": {
    "name": "Alanine",
    "short_name": "Ala",
    "abbreviation": "A",
    "molecular_weight": 89.09,
  }
}
```

### Get Amino Acid Codons

This route returns the codons that code for an amino acid.

- **URL**: `/amino_acid/{amino_acid}/codon`
- **Method**: `GET`
- **URL Parameters**:
  - `amino_acid`: The name, three-letter code, or one-letter code of the amino acid.
- **Example**:

```bash
curl https://aaprop.shuttleapp.rs/amino_acid/Alanine/codon
```

```json
{
  "amino_acid": {
    "name": "Alanine",
    "short_name": "Ala",
    "abbreviation": "A",
    "codons": [
      "GCT",
      "GCC",
      "GCA",
      "GCG"
    ]
  }
}
```

### Get Amino Acid Abbreviation

This route returns the abbreviation of an amino acid.

- **URL**: `/amino_acid/{amino_acid}/abbreviation`
- **Method**: `GET`
- **URL Parameters**:
  - `amino_acid`: The name, three-letter code, or one-letter code of the amino acid.
- **Example**:

```bash
curl https://aaprop.shuttleapp.rs/amino_acid/Alanine/abbreviation
```

```json
{
  "amino_acid": {
    "name": "Alanine",
    "short_name": "Ala",
    "abbreviation": "A",
  }
}
```

### Get Amino Acid Short Name

This route returns the short name of an amino acid.

- **URL**: `/amino_acid/{amino_acid}/short_name`
- **Method**: `GET`
- **URL Parameters**:
  - `amino_acid`: The name, three-letter code, or one-letter code of the amino acid.
- **Example**:

```bash
curl https://aaprop.shuttleapp.rs/amino_acid/Alanine/short_name
```

```json
{
  "amino_acid": {
    "name": "Alanine",
    "short_name": "Ala",
    "abbreviation": "A",
  }
}
```

### Get Amino Acid Codons Count

This route returns the number of codons that code for an amino acid.

- **URL**: `/amino_acid/{amino_acid}/codon_count`
- **Method**: `GET`
- **URL Parameters**:
  - `amino_acid`: The name, three-letter code, or one-letter code of the amino acid.
- **Example**:

```bash
curl https://aaprop.shuttleapp.rs/amino_acid/Alanine/codon_count
```

```json
{
  "amino_acid": {
    "name": "Alanine",
    "short_name": "Ala",
    "abbreviation": "A",
    "codon_count": 4,
  }
}
```

### Check for the health of the API

This route returns the health status of the API.

- **URL**: `/health`
- **Method**: `GET`
- **Example**:

```bash
curl https://aaprop.shuttleapp.rs/health
```

```json
{}
```
