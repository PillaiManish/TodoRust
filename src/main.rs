mod handlers;
mod routes;

use std::fs::OpenOptions;
use std::io::Write;
use mysql::*;
use mysql::prelude::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Todo {
    date: String,
    title: String,
    status: String,
}


fn new_to_do(date: String, title: String, status: String) -> Todo {
    return Todo{date, title, status}
}

impl Todo {
    fn add(self: &Todo, conn: &mut PooledConn) {
        let query = "INSERT INTO TODO (date, title, status) values (:date, :title, :status)";
        let params = params! {"date" => &self.date, "title" => &self.title, "status"=> &self.status};
        let err = conn.exec_drop(query, params).unwrap();

        /* NOTE: not required as storing in the db
        let mut file = OpenOptions::new().write(true).append(true).open("/home/pillaimanish/Documents/Projects/Todo/sample-file.txt").unwrap();

        let content = format!("{}\t{}\t{}\t\n", self.date, self.title, self.status);
        if let Err(e) =  file.write(content.as_ref()) {
            eprint!("Error: {}", e)
        }

         */


        // return self
    }

    fn view(self: &Todo) -> &Todo {
        return &self
    }
}

#[tokio::main]
async fn main() {
    let routes = routes::routes();

    println!("Server started at http://localhost:8000");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;

    // TODO: fix error handling
    let url = "mysql://pillaimanish:password@localhost:3306/todoRust";
    let pool = Pool::new(url).unwrap();

    let mut conn = pool.get_conn().unwrap();

    let check = conn.query_drop("CREATE TABLE TODO (date varchar(250), title varchar(250), status varchar(250))");

    let date = std::env::args().nth(1).expect("Please specify an date");
    let title = std::env::args().nth(2).expect("Please specify an title");
    let status = std::env::args().nth(2).expect("Please specify an status");

    let todo = new_to_do(date, title, status);
    todo.add(&mut conn);

    println!("{:?}", todo.view());


}
