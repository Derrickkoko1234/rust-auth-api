// contains re-usable constant variables
pub static APP_VERSION: f64 = 1.1;
pub static THREE_MILI_MINUTES: usize = 60 * 3 * 1000; // in milliseconds
pub static THIRTY_MINUTES: usize = 60 * 30; // in seconds
pub static ONE_HOUR:usize = 60 * 60; // in seconds
pub static TWENTY_FOUR_HOURS: usize = 24 * ONE_HOUR;
pub static THREE_DAYS_IN_SECONDS: usize = TWENTY_FOUR_HOURS * 3; // in seconds

pub static CODE_MIN: usize = 0;
pub static CODE_MAX: usize = 15;
pub static CODE_LEN: usize = 4;
pub static CUSTOM_CODE_LEN: usize = 12;
pub static MAX_FILES_PER_UPLOAD: usize = 5;
pub static FETCH_LIMIT: i32 = 20;
pub static BROADCAST_LIMIT: i32 = 1_000_000;
pub static NIL: usize = 0;
pub static HUNDRED: usize = 100;

pub static SINGLETON: i32 = 1;
pub static UNDERGOING_MAINTENANCE: bool = false; // used for controlling when the platform is undergoing maintenance or not

pub static N_WORKERS: usize = 200; // no. of worker threads

pub static ONE_MB: u64 = 1024 * 1024;
pub static FIVE_MB: u64 = 5 * ONE_MB;
pub static MAX_FILE_SIZE: u64 = 20 * ONE_MB; // 20mb

pub static SERVER_N_WORKERS: usize = 4; // server no. of worker threads
pub static EMPTY_STR: &str = "";
pub static APP_NAME: &str = "CryptoDemo"; 
pub static APP_DOMAIN_NAME: &str = "crypto-demo.com"; 

pub static UNAUTHORIZED: &str = "Unauthorized!";
pub static SUCCESS: &str = "success";
pub static BROADCAST_CACHE_KEY: &str = "broadcast_cache";
pub static BANNER_CACHE_KEY: &str = "banner_cache";
pub static FEEDBACK_CACHE_KEY: &str = "feedback_cache";
pub static FAQ_CACHE_KEY: &str = "faq_cache";
pub static COUNTRY_CACHE_KEY: &str = "country_cache";
pub static TOKEN: &str = "token";
pub static NOT_FOUND: &str = "Not Found!";
pub static NOW: &str = "now";
pub static MAINTENANCE_MODE_ACTIVATED: &str = "We're currently undergoing maintenance at the moment! You'll be notified when we're done";

pub static X_ACCESS_TOKEN: &str = "x-access-token";
pub static ORIGIN: &str = "origin";

pub static LAMBA_API: &str = "https://api.lambahq.com/v1";
pub static LOW_MAIL: &str = "low_mail";
pub static LOW_SMS: &str = "low_sms";
pub static ANY: &str = "any";

pub static BROADCAST_SMS: &str = "sms";
pub static BROADCAST_EMAIL: &str = "email";
pub static BROADCAST_PUSH_NOTIFICATION: &str = "push_notification";

pub static CLOUDFLARE_FILE_BASE_URL: &str = "https://pub-e2d9929e46e1478b844cf73ec6d50278.r2.dev";
pub static CLOUDFLARE_BUCKET: &str = "crypto-demo";
pub static CLOUDFLARE_REGION: &str = "weur";

pub static CRYPTO_FCM: &str = "./src/credentials/crypto-fcm.json";

pub static USER_VERIFICATION_DATA: &str = "user_verification_data_";

pub static FILE_DOWNLOAD_PATH: &str = "./src/public/temp/";
pub static EMAILTEMPLATE_PATH: &str = "./src/public/templates/";
pub static FOOTER: &str = "footer";
pub static CODE: &str = "code";
pub static MESSAGE: &str = "message";
pub static BROADCAST: &str = "broadcast";
pub static WELCOME: &str = "welcome";
pub static VERIFICATION_YOUR_ACCOUNT: &str = "Verify Your Account";
pub static WELCOME_ABOARD: &str = "Welcome Aboard!";
pub static TWO_FA_ENABLED: &str = "2 Factor Authentication Enabled";
pub static TWO_FA_DISABLED: &str = "2 Factor Authentication Disabled";
pub static NEW_LOGIN_DEVICE_DETECTED: &str = "New Login Device Detected";
pub static PROVIDE_PHONE_OR_EMAIL: &str = "One of email or phone number must be provided";
pub static INVALID_CREDENTIALS: &str = "Invalid credentials";
pub static PROVIDE_YOUR_2FA_CODE: &str = "Please provide your 2FA code to continue";
pub static OPERATION_FAILED: &str = "Operation failed";

// collection names
pub static USERS_COLLECTION: &str = "users";
pub static BLOCKCHAINS_COLLECTION: &str = "blockchains";
pub static TOKENS_COLLECTION: &str = "tokens";
pub static BANNERS_COLLECTION: &str = "banners";
pub static COUNTRIES_COLLECTION: &str = "countries";
pub static FAQS_COLLECTION: &str = "faqs";
pub static FEEDBACKS_COLLECTION: &str = "feedbacks";
pub static KYCS_COLLECTION: &str = "kycs";
pub static KYC_SUBMISSIONS: &str = "kyc_submissions";
pub static TRANSACTIONS_COLLECTION: &str = "transactions";
pub static BROADCASTS_COLLECTION: &str = "broadcasts";
pub static WEB3_WALLETS_COLLECTION: &str = "web3_wallets";
pub static FIAT_WALLETS_COLLECTION: &str = "fiat_wallets";
pub static P2P_ORDERS_COLLECTION: &str = "p2p_orders";

// possible push notifications "take_to" values
pub static APP_HOME: &str = "home";


pub static AUTHORIZED_ROUTES: [&str;6] = [
    "/auth/sign-up",
    "/auth/sign-in",
    "/auth/init-account-recovery",
    "/auth/verify-account",
    "/auth/change-password",
    "/user/verify-2fa",
];

pub static FULL_TEXT_SEARCH_FIELDS: [&str;2] = [
    "name",
    "email"
];

pub static ACCESS_ROLES: [&str;6] = [
    "user", 
    "agent", 
    "merchant", 
    "expert", 
    "admin", 
    "super_admin"
];

pub static ADMIN_ROLES: [&str;2] = [
    "admin",
    "super_admin"
];

pub static DB_COLLECTIONS: [&str;14] = [
    USERS_COLLECTION,
    BLOCKCHAINS_COLLECTION,
    TOKENS_COLLECTION,
    BANNERS_COLLECTION,
    COUNTRIES_COLLECTION,
    FAQS_COLLECTION,
    FEEDBACKS_COLLECTION,
    KYCS_COLLECTION,
    KYC_SUBMISSIONS,
    TRANSACTIONS_COLLECTION,
    BROADCASTS_COLLECTION,
    WEB3_WALLETS_COLLECTION,
    FIAT_WALLETS_COLLECTION,
    P2P_ORDERS_COLLECTION
];

// indexes for all the collections
pub static USERS_COLLECTION_INDEXES: [&str;4] = [
    "name",
    "email",
    "phone",
    "country",
];

pub static BLOCKCHAINS_COLLECTION_INDEXES: [&str;1] = [
    "name",
];

pub static TOKENS_COLLECTION_INDEXES: [&str;3] = [
    "name",
    "symbol",
    "blockchain_id",
];

pub static BANNERS_COLLECTION_INDEXES: [&str;1] = [
    "title",
];

pub static COUNTRIES_COLLECTION_INDEXES: [&str;3] = [
    "country_name",
    "country_code",
    "currency_name",
];

pub static FEEDBACKS_COLLECTION_INDEXES: [&str;1] = [
    "user_id",
];

pub static KYC_SUBMISSIONS_INDEXES: [&str;2] = [
    "kyc_id",
    "user_id",
];

pub static BROADCASTS_COLLECTION_INDEXES: [&str;3] = [
    "title",
    "country_group",
    "broadcast_type",
];

pub static FIAT_WALLETS_COLLECTION_INDEXES: [&str;1] = [
    "user_id",
];

pub static P2P_ORDERS_COLLECTION_INDEXES: [&str;1] = [
    "user_id",
];

pub static ALLOWED_UPLOAD_FILE_FORMATS: [&str;27] = [
    "image/png",
    "image/webp",
    "image/jpg",
    "image/jpeg",
    "image/gif",
    "image/avif",
    "image/svg+xml",
    "video/mp4",
    "video/mov",
    "video/avi",
    "application/msword",
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
    "application/vnd.ms-excel",
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
    "application/vnd.ms-powerpoint",
    "application/vnd.openxmlformats-officedocument.presentationml.presentation",
    "application/pdf",
    "text/plain",
    "application/rtf",
    "application/vnd.oasis.opendocument.text",
    "application/vnd.oasis.opendocument.spreadsheet",
    "application/vnd.oasis.opendocument.presentation",
    "application/x-iwork-pages-sffpages",
    "application/x-iwork-keynote-sffkey",
    "application/x-iwork-numbers-sffnumbers",
    "text/html",
    "application/xml",
];


// WEB3 contants
// pub static WEB3_URL: &str = "https://data-seed-prebsc-1-s1.binance.org:8545"; // live url is: "https://bsc-dataseed1.binance.org:443" ("https://bsc-dataseed.binance.org")
// pub static BSC_BASE_API: &str = "https://api-testnet.bscscan.com"; // live url is: "https://api.bscscan.com"

// pub static MAX_UINT256: &str = "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";

// pub static PANCAKE_SWAP_V2_ROUTER: &str = "0xD99D1c33F9fC3444f8101754aBC46c52416550D1"; // live address is: "0x10ED43C718714eb63d5aA57B78B54704E256024E"
// pub static WBNB_ADDRESS: &str = "0xae13d989daC2f0dEbFf460aC112a837C89BAa7cd"; // live address is: "0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c"