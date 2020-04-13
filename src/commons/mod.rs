use num_bigint::BigUint;

pub const CONCAT: &str = "|";

#[derive(Serialize, Deserialize, Clone)]
pub struct CommitmentParams {
    pub k: u32,
    pub e: BigUint,
    pub n: BigUint
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CommitInfoVerify {
    pub index: usize,
    pub r: BigUint,
    pub u: String,
    pub v: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CommitInfoVerifyPayload {
    pub to_blind_sign: BigUint,
    pub to_verify: Vec<CommitInfoVerify>,
    pub m_commitments: Vec<BigUint>,
    pub params: CommitmentParams
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CommitInfo {
    pub r: BigUint,
    pub u: String,
    pub v: String,
    pub m: BigUint
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CommitInfoPayload {
    pub payload: Vec<CommitInfo>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BlindSignature {
    pub blind_signature: BigUint,
}