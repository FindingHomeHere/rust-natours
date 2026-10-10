use salvo::{Request, Response, handler, http::StatusCode};

#[handler]
pub async fn get_all_users(_req: &mut Request, res: &mut Response) {
    res.render_with_status(StatusCode::INTERNAL_SERVER_ERROR, "Route not defined");
}

#[handler]
pub async fn get_single_user(_req: &mut Request, res: &mut Response) {
    res.render_with_status(StatusCode::INTERNAL_SERVER_ERROR, "Route not defined");
}

#[handler]
pub async fn create_user(_req: &mut Request, res: &mut Response) {
    res.render_with_status(StatusCode::INTERNAL_SERVER_ERROR, "Route not defined");
}

#[handler]
pub async fn update_user(_req: &mut Request, res: &mut Response) {
    res.render_with_status(StatusCode::INTERNAL_SERVER_ERROR, "Route not defined");
}

#[handler]
pub async fn delete_user(_req: &mut Request, res: &mut Response) {
    res.render_with_status(StatusCode::INTERNAL_SERVER_ERROR, "Route not defined");
}
