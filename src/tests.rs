#[cfg(test)]
pub mod tests {
    // use super::*;
    use crate::Todo;
    use crate::TodoList;
    use crate::TodoMarker;
    // #[test]
    // #[ignore]
    // fn new_todo_list() {
    //     let file = TodoList::get_file();
    //     let todo_list = TodoList::new(&file);
    // }
    // #[test]
    // #[ignore]
    // fn get_first_todo() {
    //     let todo_list = TodoList::from_file();
    //     let item = todo_list.get(0);
    //     assert_eq!(item.content.to_string(), "Love Maria");
    // }
    // #[test]
    // #[ignore]
    // fn cover_with_tests_is_unchecked() {
    //     let todo_list = TodoList::from_file();
    //     let item = todo_list.get_by_name("Cover with tests".to_string());
    //     assert_eq!(item.is_checked, false);
    // }
    #[ignore]
    #[test]
    fn insert_todo_to_db() {
        TodoList::add_to_db(Todo::new(String::from("Add completion by index"), false)).unwrap();
        let todo_list = TodoList::from_db();
        todo_list.print();
        assert_eq!(11, todo_list.todos.len());
    }
    // #[ignore]
    // #[test]
    // fn migrate_from_files_to_db() {
    //     TodoList::clear_db().unwrap();
    //     let todo_list_from_file = TodoList::from_file();
    //     todo_list_from_file.print();

    //     assert_eq!(10, todo_list_from_file.save_to_db().unwrap());
    // }
    #[ignore]
    #[test]
    fn show_todolist() {
        let todo_list = TodoList::from_db();
        todo_list.print();
        assert_eq!(10, todo_list.todos.len());
    }
    #[ignore]
    #[test]
    fn clear_todolist() {
        TodoList::clear_db().unwrap();
        let todo_list = TodoList::from_db();
        assert_eq!(0, todo_list.todos.len());
    }
    #[ignore]
    #[test]
    fn complete_todo_in_db() {
        let todo_list = TodoList::from_db();
        TodoList::complete_in_db(todo_list.todos[6].id).unwrap();
        TodoList::uncomplete_in_db(todo_list.todos[7].id).unwrap();
        let todo_list = TodoList::from_db();
        todo_list.print();
        assert_eq!(true, todo_list.todos[6].is_checked);
        assert_ne!(true, todo_list.todos[7].is_checked);
    }
    #[ignore]
    #[test]
    fn update_todo_in_db() {
        let todo_list = TodoList::from_db();
        TodoList::update_in_db(todo_list.todos[11].id, String::from("Delete me")).unwrap();
        let todo_list = TodoList::from_db();
        todo_list.print();
        assert_eq!(String::from("Delete me"), todo_list.todos[11].content);
    }
    #[ignore]
    #[test]
    fn delete_todo_in_db() {
        let todo_list = TodoList::from_db();
        let prev_len = todo_list.len();
        let id = todo_list.todos[prev_len - 1].id;
        TodoList::delete_in_db(id).unwrap();
        let todo_list_2 = TodoList::from_db();
        todo_list_2.print();
        assert_eq!(prev_len - 2, todo_list.len());
    }
    #[ignore]
    #[test]
    fn resort() {
        TodoList::resort();
    }
    #[ignore]
    #[test]
    fn get_last_sort_value() {
        let last_value = TodoList::last_sort_value();
        assert_eq!(10, last_value);
    }
    #[ignore]
    #[test]
    fn get_sort_todo_by_id() {
        TodoList::resort();
        use rusqlite::params;
        let conn = TodoList::open_connection().unwrap();
        // get moved sort
        let sort: usize;
        let mut stmt = conn.prepare("SELECT sort FROM todos WHERE id = ?").unwrap();
        let mut query = stmt.query(params![7]).unwrap();
        if let Some(row) = query.next().unwrap() {
            sort = row.get(0).unwrap();
        } else {
            panic!("Select failed");
        }
        assert_eq!(1, sort);
        // get destination id
        let dest_id: usize;
        let mut stmt = conn.prepare("SELECT id FROM todos WHERE sort = ?").unwrap();
        let mut query = stmt.query(params![sort + 1]).unwrap();
        if let Some(row) = query.next().unwrap() {
            dest_id = row.get(0).unwrap();
        } else {
            panic!("Select 2 failed");
        }
        assert_eq!(1, dest_id);
    }
    #[ignore]
    #[test]
    fn recreate_table_todos() {
        let todo_list = TodoList::from_db();
        let conn = TodoList::open_connection().unwrap();
        conn.execute("DROP TABLE todos", []).unwrap();
        todo_list.save_to_db().unwrap();
    }
    // #[ignore]
    #[test]
    fn make_first_todo_questionable() {
        TodoList::marker_as(1, TodoMarker::Questionable).unwrap();
        let todo_list = TodoList::from_db();
        let marker = match todo_list.todos[0].marker {
            TodoMarker::Important => 1,
            TodoMarker::Questionable => 2,
            TodoMarker::Default => 0,
        };
        assert_eq!(marker, 2);
    }
}
