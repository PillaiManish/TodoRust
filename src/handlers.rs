use warp::Filter;
use super::Todo;

// A function to handle GET requests at /posts/{id}
pub async fn get_todo(id: u64) -> Result<impl warp::Reply, warp::Rejection> {
    let todo = Todo {
        date: "".to_string(),
        title: String::from("Hello, Warp!"),
        status: "".to_string(),
    };
    Ok(warp::reply::json(&todo))
}