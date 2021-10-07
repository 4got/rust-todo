#[cfg(test)]
pub mod tests {
    // use super::*;
    use crate::Todo;
    use crate::TodoList;
    // #[test]
    // fn new_todo_list() {
    //     let file = TodoList::get_file();
    //     let todo_list = TodoList::new(&file);
    // }
    // #[test]
    // fn get_first_todo() {
    //     let todo_list = TodoList::from_file();
    //     let item = todo_list.get(0);
    //     assert_eq!(item.content.to_string(), "Love Maria");
    // }
    // #[test]
    // fn cover_with_tests_is_unchecked() {
    //     let todo_list = TodoList::from_file();
    //     let item = todo_list.get_by_name("Cover with tests".to_string());
    //     assert_eq!(item.is_checked, false);
    // }
    // #[test]
    // fn insert_todo_to_db() {
    //     // let todo_list = TodoList::from_file();
    //     let added_index = TodoList::__add(Todo::new(String::from("Hi"), false));
    //     assert_eq!(added_index, Ok(2));
    // }
    #[test]
    fn migrate_from_files_to_db() {
        let todo_list = TodoList::from_db();
        assert_eq!(1, todo_list.todos.len());
        for todo in todo_list.todos {
            assert_eq!("", todo.content);
        }
    }
    // #[test]
    // fn last_item_is_unchecked() {
    //     let file = TodoList::get_file();
    //     let todo_list = TodoList::new(&file);

    //     todo_list.get();
    //     panic!("Cover with tests");
    // }
}
