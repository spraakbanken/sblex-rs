#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct FullformLex {
    pub id: String,
    pub fm: String,
    pub fp: Vec<String>,
    pub l: String,
    pub gf: String,
    pub p: String,
}
