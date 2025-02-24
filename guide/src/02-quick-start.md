<!--
SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami

SPDX-License-Identifier: Apache-2.0
SPDX-License-Identifier: MIT
-->

# **Quick Start Guide**  

The **AAProp API** provides a straightforward way to retrieve detailed information about amino acids via a RESTful interface. This guide will help you get started with accessing the API efficiently.  

## **API Base URL**  

The current version of the API is available at:  
[https://aaprop-c1io.shuttle.app/v1](https://aaprop-c1io.shuttle.app/v1)  

All API requests should be directed to this endpoint.  

## **Retrieving Amino Acid Information**  

To fetch data for a specific amino acid, send a **GET request** to the following endpoint:  

```plaintext
/amino_acid/{amino_acid}
```  

where `{amino_acid}` can be:  

- The **full name** (for example, `Alanine`)  
- The **three-letter code** (for example, `Ala`)  
- The **one-letter code** (for example, `A`)  

### **Example Request**  

You can use `curl` or any HTTP client to make a request:  

```bash
curl https://aaprop-c1io.shuttle.app/v1/amino_acid/Alanine
```  

### **Example Response**  

A successful request returns a JSON object containing amino acid details:  

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

## **Next Steps**  

- Explore the API by querying different amino acids using their names or abbreviations.  
- Integrate `aaprop` into your applications for bioinformatics, computational biology, or educational tools.  
- For additional features and advanced usage, refer to the [**API Documentation**](03-api-routes.md).  
