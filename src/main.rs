use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, prelude::*, BufReader};

const TODOLIST_PATH: &str = "list.txt";

fn get_input() -> std::io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

struct TodoList<T> {
    todos: Vec<T>,
}

impl TodoList<String> {
    fn new(file: &File) -> TodoList<String> {
        let mut todos = Vec::new();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let todo = line.unwrap();
            if todo.len() > 0 {
                todos.push(todo);
            }
        }
        TodoList { todos }
    }

    fn remove(&mut self, number: usize) -> () {
        println!("remove_number = {:?}", number);
        self.todos.remove(0);
    }

    fn add() -> std::io::Result<()> {
        Ok(())
    }

    fn print(&self) -> () {
        for (i, todo) in self.todos.iter().enumerate() {
            println!("{}. {}", i + 1, todo);
        }
    }

    fn draw_interface() {
        println!("\r\nWhat do you want?\r\n");
        println!("clear: remove all todos");
        println!("remove: remove todo");
        println!("add: add todo");
    }
    fn serve(mut file: File) -> std::io::Result<()> {
        let mut todo_list = TodoList::new(&file);
        todo_list.print();

        // interface
        TodoList::draw_interface();

        // get command
        let command: String = get_input()?;
        match command.as_str() {
            "clear" => {
                file.set_len(0)?;
                println!("\r\nClear was successfull, closing\r\n");
            },
            "remove" => {
                println!("Press number of todo: ");
                // let remove_number_string: String = get_input()?;
                let remove_number = get_input()?
                    .parse::<usize>().unwrap();

                if (remove_number > 0) & (remove_number < todo_list.todos.len()) {
                    todo_list.remove(remove_number);
                    // file.set_len(0)?;
                    // file.flush()?;
                    file.write(todo_list.todos.join("\r\n").as_bytes())?;
                    println!("todo_list.todos = {:?}", todo_list.todos.join("\r\n"));
                }
            },
            "add" => {
                println!("\r\nWant to add something?");
                let todo: String = get_input()?;
                if todo.len() <= 0 {
                    println!("\r\nNothing to add, closing\r\n");
                    return Ok(());
                }

                println!("\r\nAdd '{}' was successfull, closing\r\n", todo.trim());
                file.write_all(todo.trim().as_bytes())?;
            }
            _ => ()

        }
        Ok(())
    }

    fn get_file() -> File {
        OpenOptions::new()
            .read(true)
            .write(true)
            // .append(false)
            .create(true)
            .open(&String::from(TODOLIST_PATH))
            .unwrap()
    }

    #[allow(dead_code)]
    fn print_from_file(file: &File) {
        let reader = BufReader::new(file);
        for (i, line) in reader.lines().enumerate() {
            println!("{}. {}", i + 1, line.unwrap());
        }
    }
}

fn main() -> std::io::Result<()> {
    let file = TodoList::get_file();
    TodoList::serve(file)?;

    Ok(())
}
