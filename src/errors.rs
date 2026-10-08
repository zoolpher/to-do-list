
#[derive(Debug)]
pub enum AddError {
    ListFull,       // for add() function
}


#[derive(Debug)]
pub enum ActionError {
    TaskNotFound,   // for done() and delete() function
    IdOutOfBound255,
}
