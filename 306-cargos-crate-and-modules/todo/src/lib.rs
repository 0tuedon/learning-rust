mod list {

    pub struct Tasks {
        pub item: String,
    }

    pub mod things_todo {}
    mod items_completed {}
}

mod things_todo;

use things_todo::add_activity;
use things_todo::items_completed;

fn lets_add_task() {
    let task = list::Tasks {
        item: String::from("Tasks"),
    };
    add_activity();
    add_activity();
    items_completed::move_back_todo();
}
