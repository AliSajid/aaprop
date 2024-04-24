<!--
SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami

SPDX-License-Identifier: Apache-2.0
SPDX-License-Identifier: MIT
-->

# Quick Start

The AAProp API is available at [https://aaprop.shuttleapp.rs](https://aaprop.shuttleapp.rs). You can access the API by sending a GET request to `/amino_acid/{amino_acid}` where `{amino_acid}` is the name, three-letter code, or one-letter code of the amino acid you want to get information about.

## Using the API

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
