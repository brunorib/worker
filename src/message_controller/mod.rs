use crate::commons::CommitInfoVerifyPayload;
use crate::commons::SignatureVerifyPayload;
use crate::signer::*;

use openssl::pkcs12::ParsedPkcs12;

use serde_json::Value;
use log::info;

const ACTION: &str = "action";
const PAYLOAD: &str = "payload";

pub fn handle(mut input: Value, keystore: &ParsedPkcs12) -> Result<String, String> {
    let message_type: &str = input[ACTION].as_str().unwrap();
    let rsa_key = keystore.cert.public_key().unwrap().rsa().unwrap();
    let response;
    match message_type {
        "checkFair" => {
            info!("Validating commitments...");
            let commitments: CommitInfoVerifyPayload = serde_json::from_value(input[PAYLOAD].take()).unwrap();
            
            info!("Checking commitments...");
            if check_fair(&commitments, rsa_key) {
                info!("Commitments checked. Signing blind message...");
                response = serde_json::to_string(&sign(&commitments.to_blind_sign, keystore)).unwrap();
            } else {
                response = r#"
                            {
                                "status": "fail",
                                "message": "Provided answers do not match commitments"
                            }"#.to_string();
            }
        },
        "verify" => {
            info!("Verifying signature...");
            let payload: SignatureVerifyPayload = serde_json::from_value(input[PAYLOAD].take()).unwrap();
            if verify_sign(&payload, rsa_key) {
                response = r#"
                {
                    "status": "success"
                }"#.to_string();
            } else {
                response = r#"
                {
                    "status": "fail",
                    "message": "Signature does not match message"
                }"#.to_string();
            }
        },
        _ => return Err("Action not found".to_string()),
    }

    info!("{}", response);
    Ok(response)
}
