use std::fs::OpenOptions;
use std::io::Write;
use mysql::*;
use mysql::prelude::Queryable;

#[derive(Debug)]
struct Todo {
    date: String,
    title: String,
    status: String,
}


fn new_to_do(date: String, title: String, status: String) -> Todo {
    return Todo{date, title, status}
}

impl Todo {
    fn add(self: &Todo) {
        let mut file = OpenOptions::new().write(true).append(true).open("/home/pillaimanish/Documents/Projects/Todo/sample-file.txt").unwrap();

        let content = format!("{}\t{}\t{}\t\n", self.date, self.title, self.status);
        if let Err(e) =  file.write(content.as_ref()) {
            eprint!("Error: {}", e)
        }


        // return self
    }

    fn view(self: &Todo) -> &Todo {
        return &self
    }
}

fn main() {
    // TODO: fix error handling
    let url = "mysql://pillaimanish:password@localhost:3306/todoRust";
    let pool = Pool::new(url);

    let mut conn = pool.unwrap().get_conn();

    let check = conn.expect("REASON").query_drop("CREATE TABLE TODO (date varchar(250), title varchar(250), status varchar(250))");

    let date = std::env::args().nth(1).expect("Please specify an date");
    let title = std::env::args().nth(2).expect("Please specify an title");
    let status = std::env::args().nth(2).expect("Please specify an status");

    let todo = new_to_do(date, title, status);

    todo.add();
    println!("{:?}", todo.view());


}
