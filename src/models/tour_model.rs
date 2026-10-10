use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::tours)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tour {
    pub id: i32,
    pub name: String,
    pub duration: Option<i32>,
    pub max_group_size: Option<i32>,
    pub difficulty: Option<String>,
    pub ratings_avg: Option<f64>,
    pub ratings_qty: Option<i32>,
    pub price: Option<i32>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub image_cover: Option<String>,
    pub images: Option<Vec<Option<String>>>,
    pub start_dates: Option<Vec<Option<time::Date>>>,
    pub created_at: Option<time::OffsetDateTime>,
    pub updated_at: Option<time::OffsetDateTime>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct NewTourSimple {
    pub name: String,
    pub duration: i64,
    #[serde(rename = "maxGroupSize")]
    pub max_group_size: i64,
    pub difficulty: String,
    #[serde(rename = "ratingsAverage")]
    pub ratings_average: f64,
    #[serde(rename = "ratingsQuantity")]
    pub ratings_quantity: i64,
    pub price: i64,
    pub summary: String,
    pub description: String,
    #[serde(rename = "imageCover")]
    pub image_cover: String,
    pub images: Vec<String>,
    #[serde(rename = "startDates")]
    pub start_dates: Vec<String>,
}
