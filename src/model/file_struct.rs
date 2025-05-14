use serde::Deserialize;


#[derive(Clone)]

pub struct UploadedFile {
    pub name: String,
    pub contents: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct GeneratedResponse {
    pub message: String,
    pub data: Vec<FileResponse>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct FileResponse {
    pub filename: String,
    pub response: String,
}

