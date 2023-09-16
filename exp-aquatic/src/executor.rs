use std::collections::VecDeque;
use super::actions::{Actions, Action};

pub enum Priority {
    Low,
    High
}

pub struct Task {
    action: Actions,
    blocking: bool
}

impl Task {
    pub async fn new(action: Actions, blocking: bool) -> Self {
        Self {
            action: action,
            blocking: blocking
        }
    }
    pub async fn exec(self) {
        if self.blocking {
            self.action.exec().await;
        } else {
            self.action.exec();
        }
    }
}

pub struct Exec {
    tasks: VecDeque<Task>,
    priority: VecDeque<Task>
}

impl Exec {
    pub async fn new() -> Self {
        Self {
            tasks: VecDeque::new(),
            priority: VecDeque::new()
        }
    }

    pub async fn run(&mut self) {
        loop {
            let task = if !self.priority.is_empty() {
                self.priority.pop_front().unwrap()
            } else if !self.tasks.is_empty() {
                self.tasks.pop_front().unwrap()
            } else {
                continue
            };
            task.exec();
        }
    }

    pub async fn register(&mut self, action: Actions, priority: Priority, blocking: bool) {
        match priority {
            Priority::Low => self.priority.push_back(Task::new(action, blocking).await),
            Priority::High => self.tasks.push_back(Task::new(action, blocking).await)
        }
    }
}

