#[cfg(test)]
pub mod tests {
    // use super::*;
    use crate::TodoList;
    // #[test]
    // fn new_todo_list() {
    //     let file = TodoList::get_file();
    //     let todo_list = TodoList::new(&file);
    // }
    #[test]
    fn get_first_todo() {
        let file = TodoList::get_file();
        let todo_list = TodoList::new(&file);
        let item = todo_list.get(0);
        assert_eq!(item.content.to_string(), "Use struct static print method");
    }
    #[test]
    fn cover_with_tests_is_unchecked() {
        let file = TodoList::get_file();
        let todo_list = TodoList::new(&file);
        let item = todo_list.get_by_name("Cover with tests".to_string());
        assert_eq!(item.is_checked, false);
    }
    // #[test]
    // fn last_item_is_unchecked() {
    //     let file = TodoList::get_file();
    //     let todo_list = TodoList::new(&file);

    //     todo_list.get();
    //     panic!("Cover with tests");
    // }
}
