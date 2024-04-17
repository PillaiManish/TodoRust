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
    fn add(self: Todo) -> Todo {
        return self
    }

    fn view(self: Todo) -> Todo {
        return self
    }
}

fn main() {
    let date = std::env::args().nth(1).expect("Please specify an date");
    let title = std::env::args().nth(2).expect("Please specify an title");
    let status = std::env::args().nth(2).expect("Please specify an status");

    let todo = new_to_do(date, title, status);

    println!("{:?}", todo.view());


}
