use ansi_term::Colour::RGB;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use global_counter::primitive::exact::CounterUsize;
use rusqlite::{params, Connection, Result};

// pub fn get_input() -> std::io::Result<String> {
//     let mut input = String::new();
//     io::stdin().read_line(&mut input)?;
//     Ok(input.trim().to_string())
// }

static LAST_ID: CounterUsize = CounterUsize::new(0);
pub fn last_id() -> usize {
    LAST_ID.inc();
    LAST_ID.get()
}

pub enum TodoMarker {
    Default,
    Important,
    Questionable,
}
impl TodoMarker {
    pub fn from_usize(marker_id: usize) -> Self {
        match marker_id {
            1 => TodoMarker::Important,
            2 => TodoMarker::Questionable,
            _ => TodoMarker::Default,
        }
    }
}

pub struct Todo<T> {
    pub id: usize,
    pub content: T,
    pub is_checked: bool,
    pub sort: usize,
    pub marker: TodoMarker,
    pub list_id: usize,
}
impl Todo<String> {
    pub fn new(content: String, is_checked: bool, list_id: usize) -> Self {
        let id = last_id();
        Self {
            id,
            content,
            is_checked,
            sort: TodoList::last_sort_value() + 1,
            marker: TodoMarker::Default,
            list_id,
        }
    }
    pub fn is_questionable(&self) -> bool {
        match self.marker {
            TodoMarker::Questionable => true,
            _ => false,
        }
    }
    pub fn is_important(&self) -> bool {
        match self.marker {
            TodoMarker::Important => true,
            _ => false,
        }
    }
}

#[allow(dead_code)]
pub struct List {
    pub id: usize,
    pub name: String,
}
pub struct TodoList<T> {
    pub todos: Vec<T>,
    pub list: List,
    // pub lists: Vec<List>,
}
impl TodoList<Todo<String>> {
    // #[allow(dead_code)]
    // pub fn new(todos: Vec<Todo<String>>) -> Self {
    //     Self { todos }
    // }
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.todos.len()
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
    }

    // #[allow(dead_code)]
    // pub fn has_item(&self, n: usize) -> bool {
    //     if n == 0 || self.todos.len() < n {
    //         false
    //     } else {
    //         true
    //     }
    // }

    #[allow(dead_code)]
    pub fn remove(&mut self, index: usize) -> () {
        println!("remove_number = {:?}", index);
        self.todos.remove(index);
    }
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn draw_interface() {
        println!("\r\n{}\r\n", RGB(127, 255, 127).paint("What do you want?"));
        println!("clear: {}", RGB(166, 166, 166).paint("remove all todos"));
        println!("remove: {}", RGB(166, 166, 166).paint("remove todo"));
        println!("add: {}", RGB(166, 166, 166).paint("add todo"));
        println!("complete: {}", RGB(166, 166, 166).paint("complete todo"));
        println!("restart: {}", RGB(166, 166, 166).paint("restart todo"));
        println!("exit: {}", RGB(166, 166, 166).paint("exit"));
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
            "CREATE table if not exists todolists (
                id integer primary key,
                name text not null default 'Todolist'
            )",
            params![],
        )?;

        conn.execute(
            "CREATE table if not exists todos (
                id integer primary key,
                content text not null,
                is_checked TINYINT(1) not null,
                sort integer not null default 0,
                marker TINYINT(1) not null default 0,
                list_id integer not null default 1
            )",
            params![],
        )?;

        Ok(conn)
    }
    #[allow(dead_code)]
    pub fn create() -> Result<usize, rusqlite::Error> {
        let conn = TodoList::open_connection().unwrap();
        conn.execute(
            "INSERT INTO todolists (name) values ('Todolist')",
            params![],
        )
    }
    pub fn lists_from_db() -> Vec<List> {
        let conn = TodoList::open_connection().unwrap();
        let mut stmt = conn.prepare("SELECT id, name from todolists").unwrap();
        let lists: Vec<List> = stmt
            .query_map([], |row| {
                Ok(List {
                    id: row.get(0)?,
                    name: row.get(1)?,
                })
            })
            .unwrap()
            .into_iter()
            .map(|t| t.unwrap())
            .collect();
        lists
    }
    pub fn from_db_by_list(list: List) -> Self {
        let conn = TodoList::open_connection().unwrap();
        let mut stmt = conn
            .prepare("SELECT id, content, is_checked, sort, marker, list_id from todos WHERE list_id = ?  ORDER BY sort ASC")
            .unwrap();
        let todos: Vec<Todo<String>> = stmt
            .query_map(params![list.id], |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    content: row.get(1)?,
                    is_checked: <bool>::from(row.get(2).unwrap()),
                    sort: row.get(3)?,
                    marker: TodoMarker::from_usize(row.get(4).unwrap()),
                    list_id: row.get(5)?,
                })
            })
            .unwrap()
            .into_iter()
            .map(|t| t.unwrap())
            .collect();
        Self { todos, list }
    }
    pub fn from_db_as_lists() -> Vec<TodoList<Todo<String>>> {
        let mut todo_lists = vec![];
        let lists = TodoList::lists_from_db();
        for list in lists {
            todo_lists.push(TodoList::from_db_by_list(list));
        }
        todo_lists
    }
    pub fn delete_in_db(id: usize) -> Result<usize, rusqlite::Error> {
        let conn = TodoList::open_connection().unwrap();
        conn.execute("DELETE from todos WHERE id = ?", params![id])
    }

    pub fn add_to_db(todo: Todo<String>) -> Result<usize, rusqlite::Error> {
        let conn = TodoList::open_connection().unwrap();
        conn.execute(
            "INSERT INTO todos (content, is_checked, sort, list_id) values (?1, ?2, ?3, ?4)",
            params![
                todo.content.to_string(),
                if todo.is_checked {
                    1.to_string()
                } else {
                    0.to_string()
                },
                todo.sort,
                todo.list_id
            ],
        )
    }

    #[allow(dead_code)]
    pub fn clear_db() -> Result<usize, rusqlite::Error> {
        let conn = TodoList::open_connection()?;
        conn.execute("DELETE from todos", [])
    }

    #[allow(dead_code)]
    pub fn save_to_db(self) -> Result<usize, rusqlite::Error> {
        TodoList::clear_db().unwrap();
        let mut result = 0;
        for todo in self.todos {
            if let Ok(index) = TodoList::add_to_db(todo) {
                result += index;
            }
        }
        Ok(result)
    }
    pub fn complete_in_db(id: usize) -> Result<usize, rusqlite::Error> {
        let conn = TodoList::open_connection().unwrap();
        conn.execute(
            "UPDATE todos SET is_checked = '1' WHERE id = ?",
            params![id],
        )
    }
    pub fn uncomplete_in_db(id: usize) -> Result<usize, rusqlite::Error> {
        let conn = TodoList::open_connection().unwrap();
        conn.execute(
            "UPDATE todos SET is_checked = '0' WHERE id = ?",
            params![id],
        )
    }
    pub fn update_in_db(id: usize, content: String) -> Result<usize, rusqlite::Error> {
        let conn = TodoList::open_connection().unwrap();
        conn.execute(
            "UPDATE todos SET content = ?1 WHERE id = ?2",
            params![content, id],
        )
    }
    pub fn mark_as(id: usize, marker: TodoMarker) -> Result<usize, rusqlite::Error> {
        let conn = TodoList::open_connection().unwrap();
        let marker = match marker {
            TodoMarker::Important => 1,
            TodoMarker::Questionable => 2,
            TodoMarker::Default => 0,
        };
        conn.execute(
            "UPDATE todos SET marker = ?1 WHERE id = ?2",
            params![marker, id],
        )
    }
    pub fn move_to_in_db(id: usize, to: i32) -> Result<usize, rusqlite::Error> {
        let conn = TodoList::open_connection().unwrap();
        // get moved sort
        let mut sort: i32 = 0;
        let mut stmt = conn.prepare("SELECT sort FROM todos WHERE id = ?")?;
        if let Some(row) = stmt.query(params![id])?.next()? {
            sort = row.get(0)?;
        }
        // resort on destination absence
        let mut stmt = conn.prepare("SELECT id FROM todos WHERE sort = ?")?;
        if let None = stmt.query(params![sort + to])?.next()? {
            TodoList::resort();
        }
        // get moved sort
        let mut sort: i32 = 1;
        let mut stmt = conn.prepare("SELECT sort FROM todos WHERE id = ?")?;
        if let Some(row) = stmt.query(params![id])?.next()? {
            sort = row.get(0)?;
        }
        // get destination id
        let mut dest_id: usize = id;
        let mut stmt = conn.prepare("SELECT id FROM todos WHERE sort = ?")?;
        if let Some(row) = stmt.query(params![sort + to])?.next()? {
            dest_id = row.get(0)?;
        }
        // change sorts
        let mut stmt = conn
            .prepare("UPDATE todos SET sort = ?1 WHERE id = ?2")
            .unwrap();
        stmt.execute(params![sort + to, id]).unwrap();
        stmt.execute(params![sort, dest_id])
    }
    pub fn resort() {
        let conn = TodoList::open_connection().unwrap();
        let mut stmt = conn
            .prepare("SELECT id FROM todos ORDER BY sort ASC")
            .unwrap();
        let mut query = stmt.query([]).unwrap();
        let mut sort: usize = 1;
        while let Some(row) = query.next().expect("Next row error") {
            let id: usize = row.get(0).expect("Field error");
            conn.execute(
                "UPDATE todos SET sort = ?1 WHERE id = ?2",
                params![sort, id],
            )
            .unwrap();
            sort += 1;
        }
    }
    pub fn last_sort_value() -> usize {
        let conn = TodoList::open_connection().unwrap();
        let mut stmt = conn
            .prepare("SELECT sort FROM todos ORDER BY sort DESC LIMIT 1")
            .unwrap();
        let mut query = stmt.query([]).unwrap();
        if let Some(row) = query.next().expect("Next row error") {
            return row.get(0).expect("Field error");
        }
        return 0;
    }
}
