pub mod auth;
pub mod general;
pub mod user;

/*
 * DB Models:
 *
    1. users
        -- id
        -- name
        -- username
        -- email
        -- country
        -- password
        -- phone (an Option<String>)
        -- dob (i.e the date of birth. an Option<String>)
        -- is_verified
        -- device_token

        -- referral_code (i.e the user's referral code, used for inviting friends/users)
        -- referred_by (i.e the person who referred this user. an Option<String>)

        -- profile_image (an Option<String>)

        -- access_role (one of "user", "agent", "merchant", "expert", "admin" or "super_admin")

        -- is_enabled (i.e can be used by the admin to temporarily disable this user, until kyc or other due diligence is completed)

        -- settings (a JSONB object)
            {
                --  notifications: {
                        notify_me_on_transactions: bool,
                        notify_me_on_new_login: Option<{
                            is_enabled: bool,
                            last_login_device: Option<String> (should also be set on signup)
                        }>
                    },
                --  security: {
                        two_fa: Option<{
                            -- is_enabled: bool,
                            -- secret_key: String (used for validating the supplied code from the user's 2FA app like google authenticator)
                        }>
                    }
            }

    2. blockchains (i.e the tokens supported by the platform)  (possible routes: 1. "get_supported_blockchains" an array of ["bep20","bep2","erc20","trc20"])
        -- id
        -- name (e.g "bep20", or "erc20" etc.)
        -- web3_url (e.g "https://data-seed-prebsc-1-s1.binance.org:8545". i.e the rpc url for initializing the web3 api)
        -- scanner_url (e.g "https://api-testnet.bscscan.com" i.e the scanning api of the blockchain)
        -- convert_chain_base_token_to_token_url (e.g "https://api.binance.com/api/v3/ticker/price?symbol=BNB${token}") // url for converting the base token (i.e BNB) of that blockchain to another token running in that same chain

        -- swap_router (e.g "0xD99D1c33F9fC3444f8101754aBC46c52416550D1" i.e for e.g, the pancake swap router for bep20)
        -- swap_base_token_address (e.g "0xae13d989daC2f0dEbFf460aC112a837C89BAa7cd" i.e for e.g, "WBNB" the pancake swap base conversion token for bep20)
        -- swap_abi (i.e the abi for swapping tokens on this chain)


        -- max_uint (e.g "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff" i.e the max. uint)
        -- min_gas_limit (e.g usize with value 21_000)
        -- gas_limit (e.g usize with value 25_000)
        -- highest_gas_limit (e.g usize with value 60000)
        -- highest_swap_gas_limit (e.g usize with value 150000)
        -- max_gas_price (e.g usize with value 10_000_000_000) (i.e 10 GWEi. 1 gwei = 1_000_000_000 i.e 1^9; then the amount of BNB needed can be calculated as ===> ( (maxGas * maxGasPrice) / 10^18 ) BNB)

        -- is_enabled (i.e a bool used for enabling/disabling a particular chain and all of its tokens on the platform)

    3. tokens (i.e the tokens supported by the platform)  (possible routes: 1. "get_supported_blockchains" an array of ["bep20","bep2","erc20","trc20"])
        -- id
        -- name
        -- symbol
        -- icon
        -- icon_dominant_color
        -- blockchain_id (i.e the id of the blockchain above this token is available in)
        -- contract_address
        -- abi

        -- decimals
        -- minimum_transfer
        -- minimum_swap

        -- charges (i.e a list of charges for this token)
            [
                {
                    -- charge_percentage
                    -- charge_cap
                    -- charge_recipient_address (i.e the address of the platform owners to send the charges to)
                }
            ]

        -- is_enabled (i.e bool.  Used for disabling/enabling tokens on the platform. Only is_enabled tokens would be shown in the app and site)

    4. banners (i.e the mobile ad banners, just like the one shown in binance)
        -- id
        -- image_url
        -- url (i.e url to take the user, to on-click. an Option<String>)
        -- title (i.e Option<String>)
        -- description (i.e Option<String>)
        -- is_enabled (i.e bool)

    5. countries (i.e the countries supported by the platform. Only users within these countries can use the platform. NOTE: countries that have been interacted with cannot be removed after being used)
        -- id
        -- country_name (e.g "United States")
        -- country_code (e.g "234")
        -- icon (i.e the url of the country flag)

        -- currency_name (e.g "USD")
        -- currency_code (e.g "$")

        -- is_enabled (i.e bool)

    6. faqs
        -- id
        -- question
        -- answer

    7. feedbacks (i.e a system that allows the users to reach out to the admin on how good/bad the platform has been so far, and possible improvements/suggestions they'd like the platform to add up)
        -- id
        -- user_id
        -- content (i.e the feedback)

    8. kycs  (i.e the kycs for each country, based on the level)
        -- id
        -- kyc_level (e.g 1 or 2 or 3 etc. an i32 value)
        -- countries [
            {
                -- country_id (i.e the id of the country that has the kyc questions/documents below)
                -- requirements: [
                    {
                        -- name (e.g "Profile Image")
                        -- description (e.g "Please take a selfie of yourself")
                        -- content_type (e.g "file_url" i.e an image file url)
                    },
                    {
                        -- name (e.g "Date of Birth")
                        -- description (e.g "Please provide your date of birth certificate")
                        -- content_type (e.g "file_url" i.e a pdf file url)
                    },
                    {
                        -- name (e.g "Valid Identification Document")
                        -- description (e.g "Please provide a valid identification document. Supported document types for your country are NIN, Voter Card, Passport, Driver's License, etc")
                        -- content_type (e.g "file_url" i.e a pdf file url)
                    }
                    {
                        -- name (e.g "Phone Number")
                        -- description (e.g "Please provide your home phone number")
                        -- content_type (e.g "text" i.e a text content)
                    }
                    ... add other documents/data
                ]
            }
        ]

    9. kyc_submissions  (i.e the actual submission from the user)
        -- id
        -- kyc_id (the id of the kyc above)
        -- user_id (the id of the user that submitted the kyc information)
        -- submitted_data [
            {
                -- name (e.g "Profile Image")
                -- content (e.g "https://the_profile_image_url")
            },
            {
                -- name (e.g "Phone Number")
                -- content (e.g "090583748...")
            }
        ],
        -- approved (a bool)

    10. transactions  ("transfer_in" means the user's wallet address was sent some token, while "transfer_out" means some token was sent out from the user's wallet address )
        -- id
        -- details (one of "crypto" or "fiat")
            -- crypto Option<{
                -- swap Option<{
                    -- transaction_hash (i.e the hash of the transaction on chain)

                    -- from {
                        -- token_id (i.e the token that was swapped from)
                        -- address (i.e the address initiating the swap)
                        -- amount (i.e the amount of token swapped from)
                    }
                    -- to {
                        -- token_id (i.e the token that was swapped to)
                        -- address (i.e the address receiving the swap)
                        -- amount (i.e the amount of token swapped to)
                    }

                    -- fees [
                        {
                            -- recipient_type: "platform" (i.e the platform fee for this transaction)
                            -- token_id (i.e the token earned from this transaction)
                            -- address (i.e the address receiving the fee)
                            -- amount (i.e the amount of token earned)
                        },
                        {
                            -- recipient_type: "gas" (i.e the gas fee spent for executing this transaction)
                            -- token_id (i.e the base gass fee token used for the swapping)
                            -- address (i.e the address receiving the fee)
                            -- amount (i.e the amount of token used to perform the transaction)
                        }
                    ]

                    -- status {
                        --  fulfilled Option<{
                                -- reason (i.e "N/A")
                            }>
                        --  pending Option<{
                                -- reason (i.e "N/A")
                            }>
                        --  failed Option<{
                                -- reason (i.e reason why the transaction failed)
                            }>
                    }
                }>
                -- transfer Option<{
                    -- transaction_hash (i.e the hash of the transaction on chain)
                    -- transfer_type (one of "out" or "in")
                    -- token_id (i.e the token that was transferred from)
                    -- from_address (i.e the address initiating the transfer)
                    -- to_address (i.e the address receiving the transfer)
                    -- amount (i.e the amount of token swapped from)
                    -- status {
                        --  fulfilled Option<{
                                -- reason (i.e "N/A")
                            }>
                        --  pending Option<{
                                -- reason (i.e "N/A")
                            }>
                        --  failed Option<{
                                -- reason (i.e reason why the transaction failed)
                            }>
                    }
                    -- fees [
                        {
                            -- recipient_type: "platform" (i.e the platform fee for this transaction)
                            -- token_id (i.e the token earned from this transaction)
                            -- address (i.e the address receiving the fee)
                            -- amount (i.e the amount of token earned)
                        },
                        {
                            -- recipient_type: "gas" (i.e the gas fee spent for executing this transaction)
                            -- token_id (i.e the base gass fee token used for the transfer)
                            -- address (i.e the address receiving the fee. an Option<String>)
                            -- amount (i.e the amount of token used to perform the transaction)
                        }
                    ]
                }>

                -- exchange (i.e exchanging from crypto to fiat. Helpful for P2P) Option<{
                    -- transaction_hash
                    -- p2p_order_id (i.e the id of the P2P order i.e the p2p_orders collection)
                    -- from_crypto {
                        -- token_id (i.e the token that was swapped from)
                        -- amount (i.e the amount of token received from the exchange)
                    }
                    -- to_fiat {
                        -- country_id (i.e contains the currency)
                        -- amount (i.e the amount of fiat exchanged for crypto)
                    }
                    -- status (one of "fulfilled", "pending", "failed")
                    -- fees [
                        {
                            -- recipient_type: "platform" (i.e the platform fee for this transaction)
                            -- token_id (i.e the token earned from this transaction)
                            -- address (i.e the address receiving the fee. an Option<String>)
                            -- amount (i.e the amount of token earned)
                        },
                        {
                            -- recipient_type: "gas" (i.e the gas fee spent for executing this transaction)
                            -- token_id (i.e the base gass fee token used for the p2p transfer)
                            -- address (i.e the address receiving the fee. an Option<String>)
                            -- amount (i.e the amount of token used to perform the transaction)
                        }
                    ]
                }>
            }>
        --  fiat Option<{
                -- transfer Option<{
                    -- transfer_type (one of "out" or "in")
                    -- from {
                        -- bank_name (e.g "WALLET" if sending from their wallet, or "the_actual_bank_name")
                        -- account_number
                        -- account_name
                        ... add other options like "routing number" etc
                    }
                    -- to {
                        -- bank_name (e.g "WALLET" if sent to their wallet, or "the_actual_bank_name")
                        -- account_number
                        -- account_name
                        ... add other options like "routing number" etc
                    }
                    -- country_id (i.e contains the currency)
                    -- amount (i.e the amount of fiat sent out)
                    -- status (one of "fulfilled", "pending", "failed")
                }>
                -- exchange (i.e exchanging from fiat to crypto. Helpful for P2P) Option<{
                    -- p2p_order_id (i.e the id of the P2P order i.e the p2p_orders collection)
                    -- from_fiat {
                        -- country_id (i.e contains the currency)
                        -- amount (i.e the amount of fiat exchanged for crypto)
                    }
                    -- to_crypto {
                        -- token_id (i.e the token that was swapped from)
                        -- amount (i.e the amount of token received from the exchange)
                    }
                    -- status (one of "fulfilled", "pending", "failed")
                }>
            }>

    11. broadcasts
        -- id
        -- image (i.e image for push notifications. an Option<String>)
        -- title
        -- message
        -- country_group (i.e the users in the country to broadcast to. if empty, broadcasts to all users. an Option<String>)
        -- broadcast_type (one of "push_notification" or "email")

    12. web3_wallets  (one address is used for all the tokens held by a user, in a particular protocol. Helps the user to copy once for receiving any kind of tokens)
        -- id
        -- user_id
        -- blockchain_id i.e, the id of the blockchain whose name is one of ("bep20", "bep2", "erc20", etc)
        -- wallet_address (i.e the actual wallet address they can share with users)
        -- public_key (i.e the pub key for generating the wallet_address)
        -- secret_key (i.e the secret key with which the pk can be derived again)
        -- is_enabled (i.e bool)

    13. fiat_wallets  (i.e the real money account/wallet for each user)
        -- id
        -- user_id
        -- currencies: [
            {
                -- country_id (i.e the id of the country above. Since the countries above already has the currency codes. so no need repeating them)
                -- balance (e.g 4.6; an f64. Endeavour to employ mutex for this balance)
                -- is_enabled (i.e bool. if enabled, the user can transact with the balance, else they can't)
            }
        ]

    14. p2p_orders  (i.e token/fiat pairs for p2p, for each user)
        -- id
        -- user_id
        -- rates_pair (e.g USDT/USD, BNB/NGN) {
            -- cryto {
                -- token_id (i.e id of the token)
                -- unit_value (e.g 1 i.e 1 USDT == 1000 NGN, for e.g)
            }

            fiat {
                -- country_id (i.e the id of the country containing the currency)
                -- unit_value (e.g 1000 i.e 1000 NGN == 1 USDT, for e.g)
            }
        }
 *
*/
