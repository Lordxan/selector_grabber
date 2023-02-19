use reqwest::get;
use reqwest::Url;
use scraper::ElementRef;

#[derive(Debug)]
pub enum DownloadError {
    NotFound,
    ReqwestError,
    IoError,
    Other,
}

impl From<reqwest::Error> for DownloadError {
    fn from(_: reqwest::Error) -> Self {
        DownloadError::ReqwestError
    }
}

impl From<std::io::Error> for DownloadError {
    fn from(_: std::io::Error) -> Self {
        DownloadError::IoError
    }
}

pub async fn download_file(element: ElementRef<'_>) -> Result<(bytes::Bytes, String), DownloadError> {
    let href = element.value().attr("src").ok_or(DownloadError::NotFound)?;
    let url = Url::parse(href).map_err(|_| DownloadError::Other)?;
    let filename = url
        .path_segments()
        .ok_or(DownloadError::Other)?
        .last()
        .ok_or(DownloadError::Other)?;
    let response = get(url.to_owned()).await?;
    let bytes = response.bytes().await?;
    Ok((bytes, filename.to_owned()))
}
