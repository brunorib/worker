use openssl::sha::Sha256;
use openssl::pkcs12::ParsedPkcs12;
use openssl::sign::Signer;
use openssl::rsa::Rsa;
use openssl::pkey::Public;
use openssl::bn::{BigNum, BigNumContext};


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
        let to_hash: String = elem.u.clone() + CONCAT + &elem.v;
        hasher.update(&to_hash.as_bytes());

        let output_hash = BigNum::from_slice(&hasher.finish()).unwrap();

        let mut m: BigNum = BigNum::new().unwrap();
        let mut ctx = BigNumContext::new().unwrap();
        let r: BigNum = BigNum::from_slice(&elem.r).unwrap();
        m.exp(&r, e, &mut ctx).unwrap();
        let mut m_mod: BigNum = BigNum::new().unwrap();
        m_mod.mod_mul(&m, &output_hash, n, &mut ctx).unwrap();

        if m_mod != BigNum::from_slice(&m_to_verify[i]).unwrap() {
            return false;
        }
    }
    true
}

pub fn sign(blinded: &Vec<u8>, keystore: &ParsedPkcs12) -> BlindSignature {
    let mut signer: Signer = Signer::new_without_digest(&keystore.pkey).unwrap();
    signer.update(&blinded).unwrap();
    BlindSignature {
        blind_signature: signer.sign_to_vec().unwrap()
    }
}