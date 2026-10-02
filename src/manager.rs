
#[derive(Debug)]
struct TaskManager {
    task_list : Vec<Option<Task>>,
}

impl TaskManager {


    // It will create a new and empty TaskManager 
    // everytime create() is called
    pub fn create(task: String) ->Self {
        Self { 
            task_list : Vec::new(),
        }
    }
    
    pub fn add(&self) {}
    
    pub fn done(&self) {}
    
    pub fn delete(&self) {}
    
    pub fn list(&self) {}
}