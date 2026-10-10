use serde::Serialize;

use crate::models::tour_model::Tour;

pub mod tour_controller;
pub mod user_controller;

#[derive(Serialize)]
pub struct TourResponse {
    status: String,
    results: usize,
    data: ResponseData,
}

#[derive(Serialize)]
pub struct ResponseData {
    tours: Vec<Tour>,
}

#[derive(Serialize)]
pub struct SingleTourResponse {
    status: String,
    data: SingleResponseData,
}

#[derive(Serialize)]
struct SingleResponseData {
    tour: Tour,
}
