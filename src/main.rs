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
    update_lists: String,
}

#[derive(Deserialize)]
struct ToUpdate {
    index: usize,
    content: String,
}
#[derive(Deserialize)]
struct ToUpdateLists {
    index: usize,
    name: String,
}

fn usize_field(field: &String) -> usize {
    field.to_string().parse::<usize>().unwrap_or_else(|_| 0)
}

#[post("/")]
async fn home_post(form: web::Form<Request>) -> Result<HttpResponse> {
    // let todo_list = TodoList::from_db();

    let mut action = form.action.to_string();
    let index = usize_field(&form.index);

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
        "remove_list" => {
            let list_id = usize_field(&form.list_id);
            List::delete(list_id).unwrap();
        }
        "save" => {
            let to_update: Vec<ToUpdate> = serde_json::from_str(&form.update)?;

            for u in to_update {
                TodoList::update_in_db(u.index, u.content).unwrap();
            }

            let to_update_lists: Vec<ToUpdateLists> = serde_json::from_str(&form.update_lists)?;
            for u in to_update_lists {
                List::rename(u.index, u.name).unwrap();
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
