use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, prelude::*, BufReader};

const TODOLIST_PATH: &str = "list.txt";

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

    fn serve(mut file: File) -> std::io::Result<()> {
        let mut todo_list = TodoList::new(&file);
        todo_list.print();
        // println!("todo_list.todos.join = {:?}", todo_list.todos.join("\r\n"));

        //  interface
        println!("\r\nWhat do you want?\r\n");
        println!("clear: remove all todos");
        println!("remove: remove todo");
        println!("add: add todo");

        let mut command = String::new();
        io::stdin().read_line(&mut command)?;
        command = command.trim().to_string();

        // clear
        if command.eq("clear") {
            file.set_len(0)?;
            println!("\r\nClear was successfull, closing\r\n");
            
        // remove
        } else if command.eq("remove") {
            println!("Press number of todo: ");
            let mut remove_number_string = String::new();
            io::stdin().read_line(&mut remove_number_string)?;

            let remove_number = remove_number_string.trim().parse::<usize>().unwrap();

            if (remove_number > 0) & (remove_number < todo_list.todos.len()) {
                todo_list.remove(remove_number);
                // file.set_len(0)?;
                // file.flush()?;
                file.write(todo_list.todos.join("\r\n").as_bytes())?;
                println!("todo_list.todos = {:?}", todo_list.todos.join("\r\n"));
            }
        
        // add
        } else if command.eq("add") {
            println!("\r\nWant to add something?");
            let mut todo = String::new();
            io::stdin().read_line(&mut todo)?;
            if todo.len() <= 0 {
                println!("\r\nNothing to add, closing\r\n");
                return Ok(());
            }

            println!("\r\nAdd '{}' was successfull, closing\r\n", todo.trim());
            file.write_all(todo.as_bytes())?;
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
