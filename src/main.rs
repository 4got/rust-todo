use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, prelude::*, BufReader};

struct TodoList {
    todos: [String],
}

impl TodoList {
    // fn from_file(file: &File) -> TodoList {
    //     TodoList {
    //         todos: BufReader::new(file).lines().collect(),
    //     }
    // }
    fn print_todo_list(file: &File) -> std::io::Result<()> {
        println!("Hi your todo list:");
        let reader = BufReader::new(file);
        for (i, line) in reader.lines().enumerate() {
            println!("{}. {}", i + 1, line?);
        }
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let file_name = String::from("list.txt");
    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .open(&file_name)?;

    // let todo_list = String::new();

    // println!("Hi your todo list:");
    // let reader = BufReader::new(&file);
    // for (i, line) in reader.lines().enumerate() {
    //     println!("{}. {}", i + 1, line?);
    // }
    // TodoList::from_file(&file);
    TodoList::print_todo_list(&file)?;

    // file.read_to_string(&mut todo_list)?;
    // if let Ok(lines) = read_lines

    println!("Want to add something?");
    let mut todo = String::new();
    io::stdin().read_line(&mut todo)?;
    println!("\r\nAdd '{}' was successfull\r\n", todo.trim());

    file.write_all(todo.as_bytes())?;

    // let file = OpenOptions::new().append(true).open()
    Ok(())
}
