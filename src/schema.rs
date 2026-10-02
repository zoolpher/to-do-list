
#[derive(Debug)]
pub enum Status {
    Done, 
    Pending,
}

#[derive(Debug)]
pub struct Task {
    pub status : Status,
    pub id: u8, 
    pub task: String,
}