use reqwest::get;
use reqwest::Url;
use scraper::ElementRef;

#[derive(Debug)]
pub enum DownloadError {
    AttributeNotFound,
    ReqwestError,
    UrlParsing,
    IoError,
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

pub async fn download_file(element: ElementRef<'_>, attribute: String) -> Result<(bytes::Bytes, String), DownloadError> {
    let href = element.value().attr(&attribute).ok_or(DownloadError::AttributeNotFound)?;
    let url = Url::parse(href).map_err(|_| DownloadError::UrlParsing)?;
    let filename = url
        .path_segments()
        .ok_or(DownloadError::UrlParsing)?
        .last()
        .ok_or(DownloadError::UrlParsing)?;
    let response = get(url.to_owned()).await?;
    let bytes = response.bytes().await?;
    Ok((bytes, filename.to_owned()))
}
