#[derive(Debug)]
pub enum Status {
    Done,
    InProgress,
    NotStarted,
}

#[derive(Debug)]
pub struct Task {
    pub id: usize,
    pub status: Status,
    pub title: String,
}

impl Task {
    pub fn new(i: usize, s: String) -> Self {
        Self {
            id: i,
            status: Status::NotStarted,
            title: s,
        }
    }

    pub fn start(&mut self) -> String {
        match self.status {
            Status::Done => String::from("Task already finished"),
            Status::InProgress => String::from("Task already started"),
            Status::NotStarted => {
                self.status = Status::InProgress;
                format!("Starting task {}", self.id)
            },
        }
    }

    pub fn finish(&mut self) -> String {
        match self.status {
            Status::Done => String::from("Task already finished"),
            Status::InProgress => {
                self.status = Status::Done;
                format!("Finished task {}", self.id)
            },
            Status::NotStarted => String::from("Task not started"),
        }
    }
}
