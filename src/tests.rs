#[cfg(test)]
pub mod tests {
    // use super::*;
    use crate::Todo;
    use crate::TodoList;
    #[test]
    #[ignore]
    fn new_todo_list() {
        let file = TodoList::get_file();
        let todo_list = TodoList::new(&file);
    }
    #[test]
    #[ignore]
    fn get_first_todo() {
        let todo_list = TodoList::from_file();
        let item = todo_list.get(0);
        assert_eq!(item.content.to_string(), "Love Maria");
    }
    #[test]
    #[ignore]
    fn cover_with_tests_is_unchecked() {
        let todo_list = TodoList::from_file();
        let item = todo_list.get_by_name("Cover with tests".to_string());
        assert_eq!(item.is_checked, false);
    }
    #[ignore]
    #[test]
    fn insert_todo_to_db() {
        let added_index = TodoList::add_to_db(Todo::new(String::from("Fine"), true));
        assert_eq!(added_index, Ok(4));
    }
    // #[ignore]
    #[test]
    fn migrate_from_files_to_db() {
        TodoList::clear_db().unwrap();
        let todo_list_from_file = TodoList::from_file();

        for todo in todo_list_from_file.todos {
            TodoList::add_to_db(Todo {
                content: todo.content,
                is_checked: todo.is_checked,
            })
            .expect("Cannot add todo to db");
        }
        let todo_list = TodoList::from_db();

        assert_eq!(10, todo_list.todos.len());
    }
    #[ignore]
    #[test]
    fn show_todolist() {
        // let todo_list = TodoList::from_db();
        // TodoList::add_to_db(Todo::new(String::from("True"), true)).unwrap();
        let todo_list = TodoList::from_db();
        todo_list.print();
        assert_eq!(0, todo_list.todos.len());
    }
    #[ignore]
    #[test]
    fn clear_todolist() {
        TodoList::clear_db().unwrap();
        let todo_list = TodoList::from_db();
        assert_eq!(0, todo_list.todos.len());
    }
}
