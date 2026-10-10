use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use salvo::{Depot, Request, Response, handler, prelude::Json};

use crate::{
    controllers::{ResponseData, SingleResponseData, SingleTourResponse, TourResponse},
    models::{
        establish_connection,
        tour_model::{NewTourSimple, Tour},
    },
};

// Get all tours
#[handler]
pub async fn get_all_tours(_req: &Request, res: &mut Response) {
    use crate::schema::tours::dsl::*;
    let conn = &mut establish_connection();
    let results = tours
        .select(Tour::as_select())
        .load(conn)
        .expect("Error loading tours.");

    res.render(Json(TourResponse {
        status: "success".to_string(),
        results: results.len(),
        data: ResponseData { tours: results },
    }));
}

// Create Tour
#[handler]
pub async fn create_tour(req: &mut Request, res: &mut Response) {
    // let body: NewTourSimple = req.parse_json().await.expect("Failed to parse JSON.");

    // let tour = TourSimple::new()
    //     .id(ObjectId::new())
    //     .name(body.name.to_string())
    //     .duration(body.duration)
    //     .description(body.description)
    //     .difficulty(body.difficulty)
    //     .max_group_size(body.max_group_size)
    //     .ratings_average(body.ratings_average)
    //     .ratings_quantity(body.ratings_quantity)
    //     .price(body.price)
    //     .summary(body.summary)
    //     .image_cover(body.image_cover)
    //     .images(body.images)
    //     .start_dates(body.start_dates);

    // tour.save().await.unwrap();

    // res.status_code(StatusCode::CREATED)
    //     .render(Json(SingleTourResponse {
    //         status: "Success".to_string(),
    //         data: SingleResponseData { tour },
    //     }))
}

// Get single tour
#[handler]
pub async fn get_single_tour(req: &mut Request, res: &mut Response) {
    // let tour_id: ObjectId = req.param("id").unwrap();

    // let Ok(Some(tour)) = TourSimple::query()
    //     .filter(|item| item._id.eq(tour_id))
    //     .first()
    //     .await
    // else {
    //     res.render_with_status(StatusCode::NOT_FOUND, "Cannot find tour with that ID");
    //     return;
    // };

    // res.render(Json(SingleTourResponse {
    //     status: "success".to_string(),
    //     data: SingleResponseData { tour },
    // }))
}

// Update Tour
#[handler]
pub async fn update_tour(req: &mut Request, res: &mut Response) {
    // let tour_id: ObjectId = req.param("id").unwrap();
    // let update: TourSimple = req.parse_json().await.expect("Failed to parse JSON.");

    // let Ok(Some(updated_tour)) = TourSimple::query()
    //     .filter(|item| item._id.eq(tour_id))
    //     .update_one(|tour| {
    //         tour.description.set(update.description)
    //             & tour.difficulty.set(update.difficulty)
    //             & tour.duration.set(update.duration)
    //             & tour.image_cover.set(update.image_cover)
    //             & tour.images.set(update.images)
    //             & tour.max_group_size.set(update.max_group_size)
    //             & tour.name.set(update.name)
    //             & tour.price.set(update.price)
    //             & tour.ratings_average.set(update.ratings_average)
    //             & tour.ratings_quantity.set(update.ratings_quantity)
    //             & tour.start_dates.set(update.start_dates)
    //             & tour.summary.set(update.summary)
    //     })
    //     .await
    // else {
    //     res.render_with_status(StatusCode::BAD_REQUEST, "Failed to update tour");
    //     return;
    // };

    // res.render_with_status(
    //     StatusCode::OK,
    //     Json(SingleTourResponse {
    //         status: "success".to_string(),
    //         data: SingleResponseData { tour: updated_tour },
    //     }),
    // );
}

// Delete Tour
#[handler]
pub async fn delete_tour(_req: &mut Request, depot: &mut Depot, res: &mut Response) {
    // let tour_id = depot.get::<TourSimple>("tour").unwrap();

    // let _tour: TourSimple = tour_id.clone();

    // res.render_with_status(StatusCode::NO_CONTENT, "Successfully deleted tour");
}
