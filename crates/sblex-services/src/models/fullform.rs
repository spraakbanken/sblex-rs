#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Fullform {
    pub id: String,
    pub gf: String,
    pub p: String,
    pub msd: String,
    pub pos: String,
    pub is: Vec<String>,
}
