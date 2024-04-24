use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BasicForm {
    pub full_name: String,
    pub email: String,
    pub programming_languages: Vec<String>,
    pub education_level: String,
}

#[derive(Debug, Deserialize)]
pub struct PDFForm {
    pub cv_string: String,
}
