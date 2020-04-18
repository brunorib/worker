extern crate rand;

use crate::commons::{CommitInfo, CommitInfoPayload, CONCAT};
use crate::cert_parser::parse_x509;

use wasm_bindgen::prelude::*;
use uuid::Uuid;
use rand::Rng;
use rand::distributions::Alphanumeric;
use ring:


#[wasm_bindgen]
pub fn calculate_commit(amount: u32, k: u32, pem: String) -> String {
    let mut vec: Vec<CommitInfo> = Vec::new();
    let key: Rsa<Public> = parse_x509(pem);
    let n = key.n();
    let e = key.e();
    for _i in 0..k {
        let r: BigNum = generate_random_bytes(1024);
        let alfa: String = generate_random_string(32);
        let beta: String = generate_random_string(32);
        let u: String = BigNum::from_u32(amount).unwrap().to_string() + CONCAT + &alfa;
        let id: Uuid = Uuid::new_v4();
        let v: String = id.to_string() + CONCAT + &beta;

        let mut hasher: Sha256 = Sha256::new();
        let to_hash: String = u.clone() + CONCAT + &v;
        hasher.update(&to_hash.as_bytes());

        let output_hash = BigNum::from_slice(&hasher.finish()).unwrap();

        let mut m: BigNum = BigNum::new().unwrap();
        let mut ctx = BigNumContext::new().unwrap();
        m.exp(&r, e, &mut ctx).unwrap();
        let mut m_mod: BigNum = BigNum::new().unwrap();
        m_mod.mod_mul(&m, &output_hash, n, &mut ctx).unwrap();

        let info: CommitInfo = CommitInfo {
            r: r.to_vec(),
            u: u, 
            v: v, 
            m: m.to_vec()
        };
        vec.push(info);
    }
    let infoPayload = CommitInfoPayload {
        payload: vec,
    };
    serde_json::to_string(&infoPayload).unwrap()
}

fn generate_random_bytes(len: u32) -> BigNum {
    let bytes: Vec<u8> = (0..len).map(|_| { rand::random::<u8>() }).collect();
    BigNum::from_slice(&bytes).unwrap()
}

fn generate_random_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .collect::<String>()
}