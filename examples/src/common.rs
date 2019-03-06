#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum ClientToServer {
    Say(String),
    Leave,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum ServerToClient {
    Answer(String),
}
