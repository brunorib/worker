extern crate base64;

use openssl::sha::Sha256;
use openssl::pkcs12::ParsedPkcs12;
use openssl::sign::Signer;
use openssl::rsa::Rsa;
use openssl::sign::Verifier;
use openssl::pkey::Public;
use openssl::bn::{BigNum, BigNumContext};
use openssl::rsa::Padding;
use log::{error};

use crate::commons::*;

pub fn check_fair(payload: &CommitInfoVerifyPayload, p_key: Rsa<Public>) -> bool {
    let info = &payload.to_verify;
    let m_to_verify = &payload.m_commitments;
    let e = p_key.e();
    let n = p_key.n();

    if m_to_verify.len() != info.len() {
        return false;
    }

    for i in 0..info.len() {
        let elem = &info[i];

        let mut hasher: Sha256 = Sha256::new();
        let to_hash: String = elem.amount.clone() + CONCAT + &elem.id;
        hasher.update(&to_hash.as_bytes());

        let output_hash = BigNum::from_slice(&hasher.finish()).unwrap();

        let mut m: BigNum = BigNum::new().unwrap();
        let mut ctx = BigNumContext::new().unwrap();
        let r: BigNum = BigNum::from_slice(&base64::decode(&elem.blinding).unwrap()).unwrap();
        m.mod_exp(&r, e, n, &mut ctx).unwrap();
        let mut m_mod: BigNum = BigNum::new().unwrap();
        m_mod.mod_mul(&m, &output_hash, n, &mut ctx).unwrap();

        let m_commited = BigNum::from_slice(&base64::decode(&m_to_verify[i]).unwrap()).unwrap();
        if m_mod != m_commited {
            error!("{} != {}", m_mod, m_commited);
            return false;
        }
    }
    true
}

pub fn sign(blinded: &String, keystore: &ParsedPkcs12) -> BlindSignature {
    let mut signer: Signer = Signer::new_without_digest(&keystore.pkey).unwrap();
    signer.set_rsa_padding(Padding::NONE).unwrap();
    
    signer.update(&base64::decode(blinded).unwrap()).unwrap();
    BlindSignature {
        blind_signature: base64::encode(signer.sign_to_vec().unwrap())
    }
}

pub fn verify_sign(payload: &SignatureVerifyPayload, keystore: &ParsedPkcs12) -> bool {
    let signature = &payload.signature;
    let to_hash = payload.amount.clone() + CONCAT + &payload.id;
    let public = keystore.cert.public_key().unwrap();
    let mut verifier = Verifier::new_without_digest(&public).unwrap();
    
    let mut hasher: Sha256 = Sha256::new();
    hasher.update(&to_hash.as_bytes());
    
    verifier.update(&hasher.finish()).unwrap();

    verifier.verify(&base64::decode(&signature).unwrap()).unwrap()
}