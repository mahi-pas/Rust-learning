
use crate::tools::data::{load_data, save_data};
use crate::model::todo::{ToDoList, ToDoEntry};

pub fn help() {
    println!("usage: todo add \"buy milk\", todo list, todo done 2, todo remove 2");
}

pub fn list() {

    let list = load_data();

    match list {
        Ok(v) => {
            println!("{}",v);
        },
        Err(e) => {
            println!("Error: {e:?}");
        }
    }
}

pub fn add(item: &String) {
    let list = load_data();

    match list {
        Ok(v) => {
            add_to_list(&v, &item)
        },
        Err(e) => {
            println!("Error: {e:?}");
        }
    }
}

fn add_to_list(list: &ToDoList, item: &String) {
    // Make mutable copy of list
    let mut new_list = list.clone();

    // Get new id: highest number + 1
    let mut new_id: i64 = 0;
    for entry in &new_list.list {
        if entry.id > new_id {
            new_id = entry.id
        }
    }
    new_id += 1;

    let new_entry = ToDoEntry {
        id: new_id,
        description: item.clone(),
        done: false
    };

    new_list.add_entry(new_entry);

    // Save changes
    save_data(&new_list); 
}

pub fn done(id: &i64) {
    let list = load_data();

    match list {
        Ok(v) => {
            list_entry_done(&v, &id)
        },
        Err(e) => {
            println!("Error: {e:?}");
        }
    }
}

fn list_entry_done(list: &ToDoList, id: &i64) {
    // Make mutable copy of list
    let mut new_list = list.clone();

    // Make edits on entries
    for entry in new_list.list.iter_mut() {
        if entry.id == *id {
            entry.done = true;
        }
    }

    // Save changes
    save_data(&new_list);
}

pub fn remove(id: &i64) {
    let list = load_data();

    match list {
        Ok(v) => {
            list_entry_remove(&v, &id);
        },
        Err(e) => {
            println!("Error: {e:?}");
        }
    }
}

fn list_entry_remove(list: &ToDoList, id: &i64) {
    // Make mutable copy of list
    let mut new_list = list.clone();

    // Make edits on entries
    new_list.delete_entry(id);

    // Save changes
    save_data(&new_list);
}