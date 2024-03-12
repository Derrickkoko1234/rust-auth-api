

ngrok http --host-header=rewrite 8080

# Database Model

This document provides a comprehensive overview of the database models used in the project, detailing the structure and data types of each model:

---

# Project Database Models

This README outlines the structure of the database models used in the project.

## Table of Contents
- [1. Users](#1-users)
- [2. Tokens](#2-tokens)
- [3. Banners](#3-banners)
- [4. Countries](#4-countries)
- [5. FAQs](#5-faqs)
- [6. Feedbacks](#6-feedbacks)
- [7. KYCs](#7-kycs)
- [8. KYC Submissions](#8-kyc-submissions)
- [9. Transactions](#9-transactions)
- [10. Broadcasts](#10-broadcasts)
- [11. Web3 Wallets](#11-web3-wallets)
- [12. Fiat Wallets](#12-fiat-wallets)
- [13. P2P Orders](#13-p2p-orders)

## 1. Users
- `id`: Integer
- `name`: String
- `username`: String
- `email`: String
- `country`: String
- `password`: String
- `phone`: Option<String>
- `dob`: Option<String> (Date of Birth)
- `is_verified`: Boolean
- `device_token`: String
- `referral_code`: String
- `referred_by`: Option<String>
- `profile_image`: Option<String>
- `access_role`: Enum ("user", "agent", "merchant", "expert", "admin", "super_admin")
- `is_enabled`: Boolean
- `settings`: JSONB

### Settings JSON Structure
```json
{
  "notifications": {
    "notify_me_on_transactions": Boolean,
    "notify_me_on_new_login": {
      "is_enabled": Boolean,
      "last_login_device": Option<String>
    }
  },
  "security": {
    "two_fa": {
      "is_enabled": Boolean,
      "secret_key": String
    }
  }
}
```

## 2. Tokens
- `id`: Integer
- `name`: String
- `symbol`: String
- `icon`: String
- `chain`: String
- `contract_address`: String
- `abi`: String
- `decimals`: Integer
- `minimum_transfer`: Integer
- `minimum_swap`: Integer
- `charges`: Array
- `is_enabled`: Boolean

### Charges Structure
```json
[
  {
    "charge_percentage": Float,
    "charge_cap": Float,
    "charge_recipient_address": String
  }
]
```

## 3. Banners
- `id`: Integer
- `image_url`: String
- `url`: Option<String>
- `title`: Option<String>
- `description`: Option<String>
- `is_enabled`: Boolean

## 4. Countries
- `id`: Integer
- `country_name`: String
- `country_code`: String
- `icon`: String
- `currency_name`: String
- `currency_code`: String
- `is_enabled`: Boolean

## 5. FAQs
- `id`: Integer
- `question`: String
- `answer`: String

## 6. Feedbacks
- `id`: Integer


- `user_id`: Integer
- `content`: String

## 7. KYCs
- `id`: Integer
- `kyc_level`: Integer
- `countries`: Array

### Countries Structure
```json
[
  {
    "country_id": Integer,
    "requirements": [
      {
        "name": String,
        "description": String,
        "content_type": String
      }
      // ... other documents/data
    ]
  }
]
```

## 8. KYC Submissions
- `id`: Integer
- `kyc_id`: Integer
- `user_id`: Integer
- `submitted_data`: Array
- `approved`: Boolean

### Submitted Data Structure
```json
[
  {
    "name": String,
    "content": String
  }
  // ... other submitted data
]
```

## 9. Transactions
- `id`: Integer
- `details`: Enum ("crypto", "fiat")

### Crypto Transactions Structure
```yaml
crypto: {
  swap: {
    transaction_id: String
    // ... additional fields
  }
  transfer: {
    // ... similar structure as swap
  }
  exchange: {
    // ... similar structure with additional fields
  }
}
```

### Fiat Transactions Structure
```yaml
fiat: {
  transfer: {
    // ... structure for fiat transfers
  }
  exchange: {
    // ... structure for fiat to crypto exchange
  }
}
```

## 10. Broadcasts
- `id`: Integer
- `image`: Option<String>
- `title`: String
- `message`: String
- `country_group`: Option<String>
- `broadcast_type`: Enum ("push_notification", "email")

## 11. Web3 Wallets
- `id`: Integer
- `user_id`: Integer
- `protocol`: String
- `public_key`: String
- `secret_key`: String
- `is_enabled`: Boolean

## 12. Fiat Wallets
- `id`: Integer
- `user_id`: Integer
- `currencies`: Array

### Currencies Structure
```json
[
  {
    "country_id": Integer,
    "balance": Float,
    "is_enabled": Boolean
  }
]
```

## 13. P2P Orders
- `id`: Integer
- `user_id`: Integer
- `rates_pair`: {
    "crypto": {
      "token_id": Integer,
      "unit_value": Float
    },
    "fiat": {
      "country_id": Integer,
      "unit_value": Float
    }
  }

---

For more info about this model, refer to the file at: `/src/api/v1/models/mod.rs`