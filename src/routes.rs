use warp::Filter;
use super::handlers;

// A function to build our routes
pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_todo()
}

// A route to handle GET requests for a specific post
fn get_todo() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("posts" / u64)
        .and(warp::get())
        .and_then(handlers::get_todo)
}