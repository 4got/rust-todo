mod tests;
mod todolist;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Result};
use actix_web_static_files;
use askama::Template;
use serde::Deserialize;
use std::env;
use todolist::*;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    todo_list: TodoList<Todo<String>>,
}
#[get("/")]
async fn home() -> Result<HttpResponse> {
    let todo_list = TodoList::from_file();
    let s = HomeTemplate { todo_list }.render().unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[derive(Deserialize)]
struct Request {
    index: String,
    new: String,
    action: String,
}
#[post("/")]
async fn home_post(form: web::Form<Request>) -> Result<HttpResponse> {
    let mut todo_list = TodoList::from_file();

    let action = form.action.to_string();
    let index = if form.index.to_string().len() > 0 {
        form.index.to_string().parse::<usize>().unwrap()
    } else {
        0
    };
    match action.as_str() {
        "check" => todo_list.todos[index].check(),
        "uncheck" => todo_list.todos[index].uncheck(),
        "remove" => todo_list.remove(index),
        "new" | _ => {
            let content = form.new.to_string();
            if content.len() > 0 {
                todo_list.add(Todo::new(content, false))?;
            }
        }
    }

    todo_list.save()?;
    Ok(HttpResponse::Found().header("Location", "/").finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let context = env::args().nth(1).unwrap_or_else(|| "".to_string());
    match context.as_str() {
        "server" => {
            return HttpServer::new(move || {
                let generated = generate();
                App::new()
                    .service(actix_web_static_files::ResourceFiles::new(
                        "/static", generated,
                    ))
                    .service(home)
                    .service(home_post)
            })
            .bind(("127.0.0.1", 8080))?
            .run()
            .await;
        }
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
