mod tests;
mod todolist;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Result};
use actix_web_static_files;
use ansi_term::Colour::RGB;
use askama::Template;
use serde::Deserialize;
use serde_json;
use std::env;
use todolist::*;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    todo_lists: Vec<TodoList<Todo<String>>>,
}
#[get("/")]
async fn home() -> Result<HttpResponse> {
    let todo_lists = TodoList::from_db_as_lists();
    let s = HomeTemplate { todo_lists }.render().unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[derive(Deserialize)]
struct Request {
    index: String,
    new: String,
    action: String,
    update: String,
    move_to: String,
    mark_as: String,
    list_id: String,
}

#[derive(Deserialize)]
struct ToUpdate {
    index: usize,
    content: String,
}

#[post("/")]
async fn home_post(form: web::Form<Request>) -> Result<HttpResponse> {
    // let todo_list = TodoList::from_db();

    let mut action = form.action.to_string();
    let index = if form.index.to_string().len() > 0 {
        form.index.to_string().parse::<usize>().unwrap()
    } else {
        0
    };

    if action.len() == 0 {
        action = if form.new.to_string().len() > 0 {
            String::from("new")
        } else {
            String::from("save")
        };
    }
    match action.as_str() {
        "check" => {
            TodoList::complete_in_db(index).unwrap();
        }
        "uncheck" => {
            TodoList::uncomplete_in_db(index).unwrap();
        }
        "remove" => {
            TodoList::delete_in_db(index).unwrap();
        }
        "save" => {
            // todo_list.save_to_db().unwrap();
            // let to_update: Vec<(usize, String)> = form
            //     .update
            //     .to_string()
            //     .split("\n")
            //     .map(|l| {
            //         let l = l.to_string();
            //         let entries = l.split("##").collect::<Vec<&str>>();
            //         (entries[0].parse::<usize>().unwrap(), entries[1].to_string())
            //     })
            //     .collect();
            let to_update: Vec<ToUpdate> = serde_json::from_str(&form.update)?;

            for u in to_update {
                TodoList::update_in_db(u.index, u.content).unwrap();
            }
        }
        "move" => {
            let to = form.move_to.parse::<i32>().unwrap();
            TodoList::move_to_in_db(index, to).unwrap();
        }
        "mark_as" => {
            let marker_id = form.mark_as.parse::<usize>().unwrap();
            let marker = TodoMarker::from_usize(marker_id);
            TodoList::mark_as(index, marker).unwrap();
        }
        "new" => {
            let content = form.new.to_string();
            let list_id = form.list_id.parse::<usize>().unwrap();
            if content.len() > 0 {
                TodoList::add_to_db(Todo::new(content, false, list_id)).unwrap();
            }
        }
        "new_list" => {
            TodoList::create().unwrap();
        }
        _ => (),
    }

    // todo_list.save_to_db().unwrap();
    Ok(HttpResponse::Found().header("Location", "/").finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    TodoList::open_connection().unwrap();
    let context = env::args().nth(1).unwrap_or_else(|| "".to_string());
    match context.as_str() {
        "server" => {
            println!(
                "\n{} {}",
                RGB(255, 255, 0).paint("Gui:"),
                RGB(127, 255, 127).paint("http://127.0.0.1:8080")
            );
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
        _ => ()
        // "dev" => {
        //     let todo_list = TodoList::from_db();
        //     todo_list.print();
        // }
        // "file" => loop {
        //     let file = TodoList::get_file();
        //     if let Ok(then) = TodoList::serve(Some(file)) {
        //         if then.as_str() == "Exit" {
        //             break;
        //         }
        //     }
        // },
        // _ => loop {
        //     if let Ok(then) = TodoList::serve() {
        //         if then.as_str() == "Exit" {
        //             break;
        //         }
        //     }
        // },
    }

    Ok(())
}
