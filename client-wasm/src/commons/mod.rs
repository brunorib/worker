pub const CONCAT: &str = "|";

#[derive(Serialize, Deserialize, Clone)]
pub struct CommitInfo {
    pub r: Vec<u8>,
    pub u: String,
    pub v: String,
    pub m: Vec<u8>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CommitInfoPayload {
    pub payload: Vec<CommitInfo>,
}
