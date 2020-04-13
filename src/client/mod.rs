extern crate rand;

use crate::commons::{CommitmentParams, CommitInfo, CommitInfoPayload, CONCAT};

use num_bigint::BigUint;
use num_traits::pow::Pow;
use num_integer::Integer;
use uuid::Uuid;
use openssl::sha::Sha256;
use rand::Rng;
use rand::distributions::Alphanumeric;

pub fn calculate_commit(amount: BigUint, params: CommitmentParams) -> CommitInfoPayload {
    let mut vec: Vec<CommitInfo> = Vec::new();
    for _i in 0..params.k {
        let r: BigUint = generate_random_bytes(1024);
        let alfa: String = generate_random_string(32);
        let beta: String = generate_random_string(32);
        let u: String = amount.to_string() + CONCAT + &alfa;
        let id: Uuid = Uuid::new_v4();
        let v: String = id.to_string() + CONCAT + &beta;

        let mut hasher: Sha256 = Sha256::new();
        let to_hash: String = u.clone() + CONCAT + &v;
        hasher.update(&to_hash.as_bytes());

        let output_hash = BigUint::from_bytes_be(&hasher.finish());

        let m: BigUint = r.pow(&params.e)*output_hash;
        let m_mod = m.mod_floor(&params.n);

        let info: CommitInfo = CommitInfo {
            r: r,
            u: u, 
            v: v, 
            m: m_mod
        };
        vec.push(info);
    }
    CommitInfoPayload {
        payload: vec,
    }
}

fn generate_random_bytes(len: u32) -> BigUint {
    let bytes: Vec<u8> = (0..len).map(|_| { rand::random::<u8>() }).collect();
    BigUint::from_bytes_be(&bytes)
}

fn generate_random_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .collect::<String>()
}