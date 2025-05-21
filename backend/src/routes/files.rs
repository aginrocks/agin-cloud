use utoipa_axum::routes;

use crate::routes::RouteType;

use super::Route;

const PATH: &str = "/api/files";

pub fn routes() -> Vec<Route> {
    vec![(RouteType::OpenApi(routes!(get)), false)]
}

/// Get files
#[utoipa::path(
    method(get),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = str)
    )
)]
async fn get() -> &'static str {
    "ok"
}
