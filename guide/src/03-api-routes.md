<!--
SPDX-FileCopyrightText: 2023 - 2025 Ali Sajid Imami

SPDX-License-Identifier: Apache-2.0
SPDX-License-Identifier: MIT
-->
# **API Routes**  

The **AAProp API** provides a structured interface for retrieving amino acid properties. Below is a comprehensive reference for available routes, their usage, and expected responses.  

## **1. Base URL**  

All API requests should be directed to:  
[https://aaprop-c1io.shuttle.app/v1](https://aaprop.shuttleapp.rs/v1)  

## **2. Available Routes**  

### **2.1 Retrieve Amino Acid Information**  

Fetches all available information about a specific amino acid.  

- **Endpoint**: `/amino_acid/{amino_acid}`  
- **Method**: `GET`  
- **URL Parameters**:  
  - `{amino_acid}`–Name, three-letter code, or one-letter code of the amino acid.  
- **Example Request**:  

  ```bash
  curl https://aaprop-c1io.shuttle.app/v1/amino_acid/Alanine
  ```  

- **Example Response**:  

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

### **2.2 Retrieve Side Chain Classification**  

Fetches the side chain property (for example, polar, non-polar, acidic, or basic) of a given amino acid.  

- **Endpoint**: `/amino_acid/{amino_acid}/side_chain`  
- **Method**: `GET`  
- **Example Request**:  

  ```bash
  curl https://aaprop-c1io.shuttle.app/v1/amino_acid/Alanine/side_chain
  ```  

- **Example Response**:  

  ```json
  {
    "amino_acid": {
      "name": "Alanine",
      "side_chain": "Nonpolar"
    }
  }
  ```  

### **2.3 Retrieve Molecular Weight**  

Fetches the molecular weight of a given amino acid.  

- **Endpoint**: `/amino_acid/{amino_acid}/molecular_weight`  
- **Method**: `GET`  
- **Example Request**:  

  ```bash
  curl https://aaprop-c1io.shuttle.app/v1/amino_acid/Alanine/molecular_weight
  ```  

- **Example Response**:  

  ```json
  {
    "amino_acid": {
      "name": "Alanine",
      "molecular_weight": 89.09
    }
  }
  ```  

### **2.4 Retrieve DNA Codons**  

Fetches the codons that encode a given amino acid.  

- **Endpoint**: `/amino_acid/{amino_acid}/codon`  
- **Method**: `GET`  
- **Example Request**:  

  ```bash
  curl https://aaprop-c1io.shuttle.app/v1/amino_acid/Alanine/codon
  ```  

- **Example Response**:  

  ```json
  {
    "amino_acid": {
      "name": "Alanine",
      "codons": [
        "GCT",
        "GCC",
        "GCA",
        "GCG"
      ]
    }
  }
  ```  

### **2.5 Retrieve Abbreviations**  

#### **2.5.1 Retrieve One-Letter Abbreviation**  

Fetches the single-letter abbreviation of a given amino acid.  

- **Endpoint**: `/amino_acid/{amino_acid}/abbreviation`  
- **Method**: `GET`  
- **Example Request**:  

  ```bash
  curl https://aaprop-c1io.shuttle.app/v1/amino_acid/Alanine/abbreviation
  ```  

- **Example Response**:  

  ```json
  {
    "amino_acid": {
      "name": "Alanine",
      "abbreviation": "A"
    }
  }
  ```  

#### **2.5.2 Retrieve Three-Letter Code**  

Fetches the standard three-letter abbreviation for a given amino acid.  

- **Endpoint**: `/amino_acid/{amino_acid}/short_name`  
- **Method**: `GET`  
- **Example Request**:  

  ```bash
  curl https://aaprop-c1io.shuttle.app/v1/amino_acid/Alanine/short_name
  ```  

- **Example Response**:  

  ```json
  {
    "amino_acid": {
      "name": "Alanine",
      "short_name": "Ala"
    }
  }
  ```  

### **2.6 Retrieve Codon Count**  

Fetches the number of codons that encode a given amino acid.  

- **Endpoint**: `/amino_acid/{amino_acid}/codon_count`  
- **Method**: `GET`  
- **Example Request**:  

  ```bash
  curl https://aaprop-c1io.shuttle.app/v1/amino_acid/Alanine/codon_count
  ```  

- **Example Response**:  

  ```json
  {
    "amino_acid": {
      "name": "Alanine",
      "codon_count": 4
    }
  }
  ```  

### **2.7 API Health Check**  

Checks the availability and operational status of the API.  

- **Endpoint**: `/health`  
- **Method**: `GET`  
- **Example Request**:  

  ```bash
  curl https://aaprop-c1io.shuttle.app/v1/health
  ```  

- **Example Response**:  

  ```json
  {}
  ```  

## **3. Summary of Routes**  

| **Route**                            | **Method** | **Description** |
|--------------------------------------|-----------|----------------|
| `/amino_acid/{amino_acid}`           | `GET`     | Retrieve full amino acid details |
| `/amino_acid/{amino_acid}/side_chain` | `GET`     | Retrieve the side chain type |
| `/amino_acid/{amino_acid}/molecular_weight` | `GET` | Retrieve the molecular weight |
| `/amino_acid/{amino_acid}/codon`      | `GET`     | Retrieve the codons for the amino acid |
| `/amino_acid/{amino_acid}/abbreviation` | `GET`    | Retrieve the one-letter abbreviation |
| `/amino_acid/{amino_acid}/short_name`  | `GET`    | Retrieve the three-letter abbreviation |
| `/amino_acid/{amino_acid}/codon_count` | `GET`    | Retrieve the number of codons for the amino acid |
| `/health`                             | `GET`     | Check the API's operational status |
