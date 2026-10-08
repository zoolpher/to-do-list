
use core::error;
use std::io::Error;
use std::task::Poll::Pending;
use std::vec;

use crate::schema::{Task, Status};
use crate::errors::{AddError, ActionError};


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
    
    pub fn add(&mut self, task: String) -> Result<u8, AddError> {

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
                return Err(AddError::ListFull);
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
    
    pub fn done(&mut self, id: u8) -> Result<(), ActionError> {

        if id == 0 {
            return Err(ActionError::TaskNotFound);
        }

        let index = (id - 1) as usize;

        match self.task_list.get_mut(index) {
            Some(Some(task)) => {
                task.status = Status::Done;
                Ok(())
            }
            _ => Err(ActionError::TaskNotFound),
        }
    }
    
    pub fn delete(&mut self, id: u8) -> Result<(), ActionError> {

        if id == 0 {
            return Err(ActionError::TaskNotFound);
        }

        let index = (id - 1) as usize; 

        match self.task_list.get_mut(index) {
            Some(slot) if slot.is_some() => {
                *slot = None;
                Ok(())
            }
            _ => Err(ActionError::TaskNotFound),
        }
    }
    
    pub fn list(&self) -> Vec<&Task> {

        let mut v : Vec<&Task> = Vec::new();
        
        for task in &self.task_list {
            match task {
                Some(task) => v.push(task),
                None => {}
            }
        }
        
        v
    }
}