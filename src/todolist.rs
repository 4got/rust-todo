use ansi_term::Colour::RGB;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, prelude::*, BufReader};

use rusqlite::{params, Connection, Result};
const TODOLIST_PATH: &str = "list.txt";

pub fn get_input() -> std::io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub struct TodoList<T> {
    pub todos: Vec<T>,
}

pub struct Todo<T> {
    pub content: T,
    pub is_checked: bool,
}

impl Todo<String> {
    pub fn new(content: String, is_checked: bool) -> Self {
        Self {
            content,
            is_checked,
        }
    }

    pub fn from_line(line: String) -> std::io::Result<Self> {
        let (content, is_checked_string) = line.split_once("##").unwrap();
        let mut is_checked = false;
        if is_checked_string.trim() == "true" {
            is_checked = true
        }
        Ok(Self::new(content.to_string(), is_checked))
    }

    pub fn as_line(&self) -> String {
        let is_checked_string = if self.is_checked { "true" } else { "false" };
        self.content.to_string() + "##" + is_checked_string
    }

    pub fn check(&mut self) {
        self.is_checked = true;
    }
    pub fn uncheck(&mut self) {
        self.is_checked = false;
    }
}

impl TodoList<Todo<String>> {
    pub fn new(file: &File) -> Self {
        let mut todos = Vec::new();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let todo = line.unwrap();
            if todo.len() > 0 {
                todos.push(Todo::from_line(todo).unwrap());
            }
        }
        Self { todos }
    }
    #[allow(dead_code)]
    pub fn get(&self, n: usize) -> &Todo<String> {
        let ref a = self.todos[n];
        &*a
    }
    #[allow(dead_code)]
    pub fn get_by_name(&self, name: String) -> &Todo<String> {
        &self
            .todos
            .iter()
            .find(|&todo| todo.content == name)
            .unwrap()
        // &*a
    }

    pub fn from_file() -> Self {
        let file = TodoList::get_file();
        Self::new(&file)
    }

    pub fn has_item(&self, n: usize) -> bool {
        if n == 0 || self.todos.len() < n {
            false
        } else {
            true
        }
    }

    pub fn remove(&mut self, index: usize) -> () {
        println!("remove_number = {:?}", index);
        self.todos.remove(index);
    }

    pub fn add(&mut self, todo: Todo<String>) -> std::io::Result<()> {
        if todo.content.len() <= 0 {
            println!("\r\nNothing to add, closing\r\n");
            return Ok(());
        }
        println!(
            "\r\nAdd '{}' was successfull, closing\r\n",
            todo.content.trim()
        );
        self.todos.push(todo);
        Ok(())
    }

    pub fn move_to(&mut self, from: usize, to: usize) {
        let todo = self.todos.remove(from);
        self.todos.insert(to, todo);
    }

    pub fn save(self) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&String::from(TODOLIST_PATH))
            .unwrap();
        file.set_len(0)?;
        file.write_all(
            self.todos
                .iter()
                .map(|t| t.as_line())
                .collect::<Vec<String>>()
                .join("\r\n")
                .as_bytes(),
        )?;
        Ok(())
    }

    pub fn print(&self) -> () {
        println!("\r\n{}\r\n", RGB(127, 255, 127).paint("Todo List"));
        for (i, todo) in self.todos.iter().enumerate() {
            if todo.is_checked {
                println!(
                    "{}. {} {}",
                    i + 1,
                    RGB(255, 255, 0).paint("✓"),
                    RGB(255, 255, 0).paint(todo.content.to_string())
                );
            } else {
                println!(
                    "{}. {} {}",
                    i + 1,
                    RGB(166, 166, 166).paint("_"),
                    RGB(166, 166, 166).paint(todo.content.to_string())
                );
            };
        }
    }

    pub fn draw_interface() {
        println!("\r\n{}\r\n", RGB(127, 255, 127).paint("What do you want?"));
        println!("clear: {}", RGB(166, 166, 166).paint("remove all todos"));
        println!("remove: {}", RGB(166, 166, 166).paint("remove todo"));
        println!("add: {}", RGB(166, 166, 166).paint("add todo"));
        println!("complete: {}", RGB(166, 166, 166).paint("complete todo"));
        println!("restart: {}", RGB(166, 166, 166).paint("restart todo"));
        println!("exit: {}", RGB(166, 166, 166).paint("exit"));
    }
    pub fn serve(file: Option<File>) -> std::io::Result<String> {
        let mut todo_list;
        let mut is_file = false;
        if let Some(ref file) = file {
            todo_list = TodoList::new(&file);
            is_file = true;
        } else {
            todo_list = TodoList::from_db();
        }
        todo_list.print();

        // interface
        TodoList::draw_interface();

        // get command
        let command: String = get_input()?;
        match command.as_str() {
            "clear" => {
                if is_file {
                    file.unwrap().set_len(0)?;
                }
                println!("\r\nClear was successfull, closing\r\n");
            }
            "remove" => {
                println!("Press number of todo: ");
                let remove_number = get_input()?.parse::<usize>().unwrap();

                if todo_list.has_item(remove_number - 1) {
                    todo_list.remove(remove_number - 1);
                    todo_list.print();
                    todo_list.save()?;
                } else {
                    println!("Wrong number = {:?}", remove_number);
                }
            }
            "add" => {
                println!("\r\nWant to add something?");
                let todo: String = get_input()?;
                todo_list.add(Todo::new(todo, false))?;
                todo_list.save()?;
            }
            "complete" => {
                println!("Press number of todo: ");
                let check_number = get_input()?.parse::<usize>().unwrap();
                if todo_list.has_item(check_number) {
                    todo_list.todos[check_number - 1].check();
                    todo_list.save()?;
                } else {
                    println!("Wrong number = {:?}", check_number);
                }
            }
            "restart" => {
                println!("Press number of todo: ");
                let uncheck_number = get_input()?.parse::<usize>().unwrap();
                if todo_list.has_item(uncheck_number) {
                    todo_list.todos[uncheck_number - 1].uncheck();
                    todo_list.save()?;
                } else {
                    println!("Wrong number = {:?}", uncheck_number);
                }
            }
            "exit" => {
                println!("Goodbye my love");
                return Ok(String::from("Exit"));
            }
            _ => (),
        }
        Ok(String::from("Continue"))
    }

    pub fn get_file() -> File {
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&String::from(TODOLIST_PATH))
            .unwrap()
    }

    #[allow(dead_code)]
    pub fn print_from_file(file: &File) {
        let reader = BufReader::new(file);
        for (i, line) in reader.lines().enumerate() {
            println!("{}. {}", i + 1, line.unwrap());
        }
    }

    // db
    pub fn open_connection() -> Result<rusqlite::Connection> {
        let conn = Connection::open("todos.db")?;

        conn.execute(
            "CREATE table if not exists todos (
                id integer primary key,
                content text not null unique,
                is_checked TINYINT(1) not null,
                sort integer not null default 0
            )",
            params![],
        )?;

        Ok(conn)
    }
    pub fn from_db() -> Self {
        let conn = TodoList::open_connection().unwrap();
        let mut stmt = conn
            .prepare("SELECT content, is_checked from todos")
            .unwrap();
        let mut todo_list = TodoList { todos: vec![] };
        let todos = stmt.query_map([], |row| {
            Ok(Todo {
                content: row.get(0)?,
                is_checked: <bool>::from(row.get(1).unwrap()),
            })
        });
        if let Ok(todos) = todos {
            for todo in todos {
                if let Ok(todo) = todo {
                    todo_list.todos.push(todo);
                }
            }
        }
        todo_list
    }
    pub fn add_to_db(todo: Todo<String>) -> Result<usize, rusqlite::Error> {
        let conn = TodoList::open_connection().unwrap();
        conn.execute(
            "INSERT INTO todos (content, is_checked) values (?1, ?2)",
            params![
                todo.content.to_string(),
                if todo.is_checked {
                    1.to_string()
                } else {
                    0.to_string()
                },
            ],
        )
    }
    pub fn clear_db() -> Result<usize, rusqlite::Error> {
        let conn = TodoList::open_connection()?;
        conn.execute("DELETE from todos", [])
    }
    pub fn save_to_db(self) -> std::io::Result<()> {
        TodoList::clear_db();
        let conn = TodoList::open_connection().unwrap();
        for todo in self.todos {
            TodoList::add_to_db(todo);
        }
        Ok(())
    }
}
