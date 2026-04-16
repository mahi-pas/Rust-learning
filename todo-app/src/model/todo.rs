use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToDoList {
    pub list: Vec<ToDoEntry>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToDoEntry {
    pub id: i64,
    pub description: String,
    pub done: bool
}

impl ToDoList {
    pub fn add_entry(&mut self, entry: ToDoEntry) {
        self.list.push(entry);
    }

    pub fn delete_entry(&mut self, id: &i64) {
        for i in 0..self.list.len() {
            if self.list[i].id == *id {
                self.list.remove(i);
                return;
            }
        }
    }
}

impl fmt::Display for ToDoList {
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        writeln!(f,"------ TO DO ------")?;
        for entry in &self.list {
            if entry.done == false {
                writeln!(f,"[ ] {} | {}", entry.id, entry.description)?;
            }
        }
        writeln!(f,"---- COMPLETE ----")?;
        for entry in &self.list {
            if entry.done == true {
                writeln!(f,"[x] {} | {}", entry.id, entry.description)?;
            }
        }
        Ok(())
    }
}