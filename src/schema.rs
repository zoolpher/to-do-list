
#[derive(Debug)]
pub enum Status {
    Done, 
    Pending,
}

#[derive(Debug)]
pub struct Task {
    pub id: u8, 
    pub task: String,
    pub status : Status,
}