#[derive(Debug)]
struct Todo<'a> {
    content: &'a str,
    line_nb: usize,
}

fn find_todos(input: &String) -> Vec<Todo> {
    return input
        .lines()
        .enumerate()
        .filter_map(|(idx, content)| {
            content.contains("todo:").then_some(Todo {
                content,
                line_nb: idx + 1,
            })
        })
        .collect::<Vec<_>>();
}

fn main() {
    let input = String::from("some code\ntodo: something\n\nmore");
    let todos = find_todos(&input);

    for t in &todos {
        println!("{:?}", t);
    }
}
