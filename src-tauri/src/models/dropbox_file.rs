use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct FolderResponse {
    pub cursor: String,
}

#[derive(Deserialize, Debug)]
pub struct LongpollResponse {
    pub backoff: Option<u32>,
    pub changes: bool,
}
