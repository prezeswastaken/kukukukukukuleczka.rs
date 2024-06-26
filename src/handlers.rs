use std::{fs::File, io::Read, path::Path, sync::Arc};

use axum::{debug_handler, extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use serde_json::json;
use tokio::sync::RwLock;

use crate::{
    config::Config,
    cv_builder::CVBuilder,
    forms::{BasicForm, PDFForm},
    jobs::{get_jobs, Job, JobCheckRequest, JobCheckResult},
    pdf_builder::PdfBuilder,
    utils::clean_storage,
};

pub async fn basic_create(Json(form): Json<BasicForm>) -> Result<impl IntoResponse, StatusCode> {
    let cv_string = CVBuilder::from(form).create_cv_string();
    match cv_string {
        Ok(cv) => Ok(Json(json!({"message": cv}))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[debug_handler]
pub async fn pdf(
    State(config): State<Arc<RwLock<Config>>>,
    Json(pdf_form): Json<PDFForm>,
) -> Result<impl IntoResponse, StatusCode> {
    let config = config.read().await;
    let storage_path = config.get_storage_path();
    let cv_string = pdf_form.cv_string;
    let mut pdf_builder = PdfBuilder::new(cv_string, storage_path.to_string());

    pdf_builder
        .create_latex_file()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let pdf_file_name = pdf_builder
        .create_pdf()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let app_url = config.get_app_url();
    let pdf_link = format!("{}/storage/{}", app_url, pdf_file_name);

    clean_storage(storage_path.to_string()).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({"pdf_link": pdf_link})))
}

pub async fn show_jobs() -> Json<Vec<Job>> {
    let jobs = get_jobs();
    Json(jobs)
}

#[derive(Serialize, Clone)]
pub struct JobResponse {
    pub job: Job,
    pub result: JobCheckResult,
}

#[debug_handler]
pub async fn check_job(Json(job_request): Json<JobCheckRequest>) -> Json<Vec<JobResponse>> {
    let jobs = get_jobs();
    let results = jobs
        .clone()
        .into_iter()
        .filter_map(|job| job.get_score(&job_request).ok())
        .enumerate()
        .map(|(i, score)| JobResponse {
            job: jobs[i].clone(),
            result: score,
        })
        .collect::<Vec<_>>();
    Json(results)
}
