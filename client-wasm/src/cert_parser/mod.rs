use openssl::rsa::Rsa;
use openssl::pkey::Public;

pub fn parse_x509(x509pem: String) -> Rsa<Public> {
    Rsa::public_key_from_pem(x509pem.as_bytes()).unwrap()
}