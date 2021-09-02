use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, prelude::*, BufReader};

const TODOLIST_PATH: &str = "list.txt";

struct TodoList<T> {
    todos: Vec<T>,
}

impl TodoList<Box<String>> {
    fn new(file: &File) -> TodoList<Box<String>> {
        let mut todos = Vec::new();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let todo = line.unwrap();
            if todo.len() > 0 {
                todos.push(Box::new(todo));
            }
        }
        TodoList { todos }
    }

    fn print(self) -> Self {
        for (i, todo) in self.todos.iter().enumerate() {
            println!("{}. {}", i + 1, todo);
        }
        self
    }

    fn serve(mut file: File) -> std::io::Result<()> {
        println!("\r\nWant to add something?");
        let mut todo = String::new();
        io::stdin().read_line(&mut todo)?;
        if todo.trim().len() <= 0 {
            println!("\r\nNothing to add, closing\r\n");
            return Ok(());
        }

        println!("\r\nAdd '{}' was successfull, closing\r\n", todo.trim());
        file.write_all(todo.as_bytes())?;
        Ok(())
    }

    fn get_file() -> File {
        OpenOptions::new()
            .read(true)
            .append(true)
            .open(&String::from(TODOLIST_PATH))
            .unwrap()
    }

    #[allow(dead_code)]
    fn print_from_file(file: &File) {
        println!("Hi, your todo list:");
        let reader = BufReader::new(file);
        for (i, line) in reader.lines().enumerate() {
            println!("{}. {}", i + 1, line.unwrap());
        }
    }
}

fn main() -> std::io::Result<()> {
    let file = TodoList::get_file();

    let todo_list = TodoList::new(&file);
    todo_list.print();

    TodoList::serve(file)?;

    Ok(())
}
