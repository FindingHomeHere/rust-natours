use salvo::prelude::*;

use crate::controllers::{
    tour_controller::{create_tour, delete_tour, get_all_tours, get_single_tour, update_tour},
    user_controller::{create_user, delete_user, get_all_users, get_single_user, update_user},
};

pub fn tour_routes() -> Router {
    Router::with_path("tours")
        .get(get_all_tours)
        .post(create_tour)
        .push(
            Router::with_path("{id}")
                .get(get_single_tour)
                .patch(update_tour)
                .delete(delete_tour),
        )
}

pub fn user_routes() -> Router {
    Router::with_path("users")
        .get(get_all_users)
        .post(create_user)
        .push(
            Router::with_path("{id}")
                .get(get_single_user)
                .patch(update_user)
                .delete(delete_user),
        )
}

// pub fn static_router() -> Router {
//     Router::with_path("{*path}")
// }
