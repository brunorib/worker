use openssl::sha::Sha256;
use openssl::pkcs12::ParsedPkcs12;
use openssl::sign::Signer;
use num_bigint::BigUint;
use num_traits::pow::Pow;
use num_integer::Integer;


use crate::commons::*;

pub fn check_fair(payload: &CommitInfoVerifyPayload) -> bool {
    let params = &payload.params;
    let info = &payload.to_verify;
    let m_to_verify = &payload.m_commitments;

    if m_to_verify.len() != info.len() {
        return false;
    }

    for i in 0..info.len() {
        let elem = &info[i];

        let mut hasher: Sha256 = Sha256::new();
        let to_hash: String = elem.u.clone() + CONCAT + &elem.v;
        hasher.update(&to_hash.as_bytes());

        let output_hash = BigUint::from_bytes_be(&hasher.finish());

        let m: BigUint = elem.r.pow(&params.e)*output_hash;
        let m_mod = m.mod_floor(&params.n);

        if m_mod != m_to_verify[i] {
            return false;
        }
    }
    true
}

pub fn sign(blinded: &BigUint, keystore: &ParsedPkcs12) -> BlindSignature {
    let mut signer: Signer = Signer::new_without_digest(&keystore.pkey).unwrap();
    signer.update(&blinded.to_bytes_be()).unwrap();
    BlindSignature {
        blind_signature: BigUint::from_bytes_be(&signer.sign_to_vec().unwrap())
    }
}