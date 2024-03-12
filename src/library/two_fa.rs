use base32;
use rand::Rng;
use totp_rs::{Algorithm, Secret, TOTP};
use super::constant;

use qrcode_generator::QrCodeEcc;

pub fn enroll_2fa(email: &str)->(String,String){
    let mut rng = rand::thread_rng();
    let data_byte: [u8; 21] = rng.gen();
    let base32_string = base32::encode(base32::Alphabet::RFC4648 { padding: false }, &data_byte);

    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        Secret::Encoded(base32_string).to_bytes().unwrap(),
    )
    .unwrap();

    let otp_base32 = totp.get_secret_base32();
    let issuer = constant::APP_NAME;
    let otp_auth_url = format!("otpauth://totp/{issuer}:{email}?secret={otp_base32}&issuer={issuer}");

    // generate qrcode for otp_auth_url
    let auth_url_img = qrcode_generator::to_svg_to_string(otp_auth_url, QrCodeEcc::Low, 200, None::<&str>).unwrap();

    (otp_base32.to_owned(),auth_url_img)
}

pub fn verify_2fa(secret_key: &str, code: &str)->bool{
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        Secret::Encoded(secret_key.to_owned()).to_bytes().unwrap(),
    )
    .unwrap();

    let is_valid = totp.check_current(code).unwrap();
    is_valid
}