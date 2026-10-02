
use core::error;
use std::io::Error;

use crate::schema::{Task, Status};

#[derive(Debug)]
pub struct TaskManager {
    task_list : Vec<Option<Task>>,
}

impl TaskManager {


    // It will create a new and empty TaskManager 
    // everytime create() is called
    pub fn create() ->Self {
        Self { 
            task_list : Vec::new(),
        }
    }
    
    pub fn add(&mut self, task: String) -> Result<u8, Error> {

        let n: usize = self.task_list.len();
        let mut i: usize = 0; 

        loop {
            if (i == n || self.task_list[i].is_none()) {
                break;
            }
            i += 1;
        };

        if i == n {
            if n >= 255 {
                return Err();
            }
            self.task_list.push(None);
        }

        self.task_list[i] = Some(Task{
            id : (i + 1) as u8,
            status : Status::Pending,
            task : task, 
        });
        
        Ok((i + 1) as u8)
    }
    
    pub fn done(&self) {}
    
    pub fn delete(&self) {}
    
    pub fn list(&self) {}
}