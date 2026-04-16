

use std::io::{BufReader, BufWriter};
use std::fs::File;
use std::error::Error;


use crate::model::todo::{ToDoList};

const LIST_FILE: &str = "src/list.json";

pub fn load_data() -> Result<ToDoList, Box<dyn Error>> {

    let file = File::open(LIST_FILE)?;
    
    let reader = BufReader::new(file);

    let list = serde_json::from_reader(reader)?;

    //let list: ToDoList = serde_json::from_str(data)?;

    Ok(list)
}

pub fn save_data(new_list: &ToDoList) {
    let save = save_handler(new_list);

    match save {
        Ok(_v) => {
        },
        Err(e) => {
            println!("Error: {e:?}");
        }
    }
}

fn save_handler(new_list: &ToDoList) -> Result<bool, Box<dyn Error>> {

    let file = File::create(LIST_FILE)?;

    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, &new_list)?;

    Ok(true)
}

