extern crate base64;

use openssl::sha::Sha256;
use openssl::pkcs12::ParsedPkcs12;
use openssl::rsa::Rsa;
use openssl::pkey::{Public, Private};
use openssl::bn::{BigNum, BigNumContext};
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
        let r: BigNum = base64_to_bignum(&elem.blinding);
        m.mod_exp(&r, e, n, &mut ctx).unwrap();
        let mut m_mod: BigNum = BigNum::new().unwrap();
        m_mod.mod_mul(&m, &output_hash, n, &mut ctx).unwrap();

        let m_commited = base64_to_bignum(&m_to_verify[i]);
        if m_mod != m_commited {
            error!("{} != {}", m_mod, m_commited);
            return false;
        }
    }
    true
}

pub fn sign(blinded: &String, keystore: &ParsedPkcs12) -> BlindSignature {
    let blinded_message: BigNum = base64_to_bignum(blinded);
    let private: Rsa<Private> = keystore.pkey.rsa().unwrap();
    let d = private.d();
    let n = private.n();
   
    let mut s: BigNum = BigNum::new().unwrap();
    let mut ctx = BigNumContext::new().unwrap();
    s.mod_exp(&blinded_message, d, n, &mut ctx).unwrap();
    
    BlindSignature {
        blind_signature: bignum_to_base64(s)
    }
}

pub fn verify_sign(payload: &SignatureVerifyPayload, public: Rsa<Public>) -> bool {
    let signature = base64_to_bignum(&payload.signature);
    let to_hash = payload.amount.clone() + CONCAT + &payload.id;
    let e = public.e();
    let n = public.n();

    let mut hasher: Sha256 = Sha256::new();
    hasher.update(&to_hash.as_bytes());
    let out_hash: BigNum = BigNum::from_slice(&hasher.finish()).unwrap();

    let mut s_decrypt: BigNum = BigNum::new().unwrap();
    let mut ctx = BigNumContext::new().unwrap();
    s_decrypt.mod_exp(&signature, e, n, &mut ctx).unwrap();

    if out_hash == s_decrypt {
        return true
    }
    
    false
}

fn base64_to_bignum(base64: &String) -> BigNum {
    BigNum::from_slice(&base64::decode(base64).unwrap()).unwrap()
}

fn bignum_to_base64(num: BigNum) -> String {
    base64::encode(num.to_vec())
}