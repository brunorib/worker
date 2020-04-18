pub const CONCAT: &str = "|";

#[derive(Serialize, Deserialize, Clone)]
pub struct CommitInfoVerify {
    pub index: usize,
    pub r: Vec<u8>,
    pub u: String,
    pub v: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CommitInfoVerifyPayload {
    pub to_blind_sign: Vec<u8>,
    pub to_verify: Vec<CommitInfoVerify>,
    pub m_commitments: Vec<Vec<u8>>
}


#[derive(Serialize, Deserialize, Clone)]
pub struct BlindSignature {
    pub blind_signature: Vec<u8>,
}