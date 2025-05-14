use reqwest::Client;
use reqwest::multipart::{ Form, Part };
use crate::model::{ GeneratedResponse, UploadedFile };
use crate::global::DOMAIN;

pub async fn send_files_to_python(
    files: Vec<UploadedFile>
) -> Result<GeneratedResponse, Box<dyn std::error::Error>> {

    let client = Client::new();
    let mut form = Form::new();

    for file in files {
        let part = Part::bytes(file.contents.into_bytes())
            .file_name(file.name.clone())
            .mime_str("application/octet-stream")?;

        form = form.part("files", part);
    }

    let localhost = "http://localhost:8000";
    
    let response = client.post(format!("{}/files/",localhost)).multipart(form).send().await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("HTTP Error {}: {}", status, error_text).into());
    }

    let file_response = response.json::<GeneratedResponse>().await?;

    Ok(file_response)
}
