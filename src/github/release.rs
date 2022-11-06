use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Asset {
    pub name: String,
    #[serde(rename(deserialize = "browser_download_url"))]
    pub download_url: String,
    #[serde(skip)]
    pub display_name: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Release {
    #[serde(rename(deserialize = "tag_name"))]
    pub tag: String,
    #[serde(rename(deserialize = "tarball_url"))]
    pub tarball: String,
    #[serde(rename(deserialize = "zipball_url"))]
    pub zipball: String,
    pub assets: Vec<Asset>,
}
