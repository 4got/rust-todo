use ansi_term::Colour::RGB;
use std::env;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, prelude::*, BufReader};
use std::net::TcpListener;
use std::net::TcpStream;

mod tests;

const TODOLIST_PATH: &str = "list.txt";

fn get_input() -> std::io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub struct TodoList<T> {
    todos: Vec<T>,
}

struct Todo<T> {
    content: T,
    is_checked: bool,
}

impl Todo<String> {
    fn new(content: String, is_checked: bool) -> Self {
        Self {
            content,
            is_checked,
        }
    }

    fn from_line(line: String) -> std::io::Result<Self> {
        let (content, is_checked_string) = line.split_once("##").unwrap();
        let mut is_checked = false;
        if is_checked_string.trim() == "true" {
            is_checked = true
        }
        Ok(Self::new(content.to_string(), is_checked))
    }

    fn as_line(&self) -> String {
        let is_checked_string = if self.is_checked { "true" } else { "false" };
        self.content.to_string() + "##" + is_checked_string
    }

    fn check(&mut self) {
        self.is_checked = true;
    }
    fn uncheck(&mut self) {
        self.is_checked = false;
    }
    // fn print(self, n: usize) {
    //     println!("{}. {}", n, self.content);
    // }
}

impl TodoList<Todo<String>> {
    fn new(file: &File) -> Self {
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
    fn get(&self, n: usize) -> &Todo<String> {
        let ref a = self.todos[n];
        &*a
    }
    #[allow(dead_code)]
    fn get_by_name(&self, name: String) -> &Todo<String> {
        let ref a = self
            .todos
            .iter()
            .find(|&todo| todo.content == name)
            .unwrap();
        &*a
    }

    fn has_item(&self, n: usize) -> bool {
        if n == 0 || self.todos.len() < n {
            false
        } else {
            true
        }
    }

    fn remove(&mut self, number: usize) -> () {
        println!("remove_number = {:?}", number);
        self.todos.remove(number - 1);
    }

    fn add(&mut self, todo: Todo<String>) -> std::io::Result<()> {
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

    fn save(self) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&String::from(TODOLIST_PATH))
            .unwrap();
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

    fn print(&self) -> () {
        println!("\r\n{}\r\n", RGB(127, 255, 127).paint("Todo List"));
        // self.todos[0].print(1);
        for (i, todo) in self.todos.iter().enumerate() {
            // todo.print(i + 1);
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

    fn draw_interface() {
        println!("\r\n{}\r\n", RGB(127, 255, 127).paint("What do you want?"));
        println!("clear: {}", RGB(166, 166, 166).paint("remove all todos"));
        println!("remove: {}", RGB(166, 166, 166).paint("remove todo"));
        println!("add: {}", RGB(166, 166, 166).paint("add todo"));
        println!("complete: {}", RGB(166, 166, 166).paint("complete todo"));
        println!("restart: {}", RGB(166, 166, 166).paint("restart todo"));
        println!("exit: {}", RGB(166, 166, 166).paint("exit"));
    }
    fn serve(file: File) -> std::io::Result<String> {
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
            }
            "remove" => {
                println!("Press number of todo: ");
                let remove_number = get_input()?.parse::<usize>().unwrap();

                if todo_list.has_item(remove_number) {
                    todo_list.remove(remove_number);
                    todo_list.print();
                    file.set_len(0)?;
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
                    file.set_len(0)?;
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
                    file.set_len(0)?;
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

    fn get_file() -> File {
        OpenOptions::new()
            .read(true)
            .write(true)
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

use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn server() -> impl Responder {
    let file = TodoList::get_file();
    let todo_list = TodoList::new(&file);

    let contents = todo_list.get(0).content.to_string();
    HttpResponse::Ok().body(contents)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let context = env::args().nth(1).unwrap_or_else(|| "".to_string());
    match context.as_str() {
        "server" => HttpServer::new(|| App::new().service(server))
            .bind(("127.0.0.1", 8080))?
            .run()
            .await
            .unwrap(),
        _ => loop {
            let file = TodoList::get_file();

            if let Ok(then) = TodoList::serve(file) {
                if then.as_str() == "Exit" {
                    break;
                }
            }
        },
    }

    Ok(())
}

// fn handle_connection(mut stream: TcpStream) {
//     let mut buffer = [0; 1024];
//     stream.read(&mut buffer).unwrap();

//     // let contents = fs::read_to_string("index.html").unwrap();
//     let file = TodoList::get_file();
//     let todo_list = TodoList::new(&file);

//     let contents = todo_list.get(0).content.to_string();

//     let response = format!(
//         "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
//         contents.len(),
//         contents
//     );

//     stream.write(response.as_bytes()).unwrap();
//     stream.flush().unwrap();

//     // let mut reg = Handlebars::new();
//     // // render without register
//     // println!(
//     //     "{}",
//     //     reg.render_template("Hello {{name}}", &json!({"name": "foo"}))?
//     // );

//     // // register template using given name
//     // reg.register_template_string("tpl_1", "Good afternoon, {{name}}")?;
//     // println!("{}", reg.render("tpl_1", &json!({"name": "foo"}))?);
//     // Ok(())
// }
