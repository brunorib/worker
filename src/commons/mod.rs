pub const CONCAT: &str = "|";

#[derive(Serialize, Deserialize, Clone)]
pub struct AnswerInfo {
    pub blinding: String,
    pub amount: String,
    pub id: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CommitInfoVerifyPayload {
    pub to_blind_sign: String,
    pub to_verify: Vec<AnswerInfo>,
    pub m_commitments: Vec<String>
}


#[derive(Serialize, Deserialize, Clone)]
pub struct BlindSignature {
    pub blind_signature: String,
}