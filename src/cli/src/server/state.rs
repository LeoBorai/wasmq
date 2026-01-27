use std::sync::Arc;

use mate_repository::TaskRepository;

use crate::process::hub::Hub;

pub type SharedServices = Arc<Services>;

#[derive(Clone)]
pub struct Services {
    pub hub: Arc<Hub>,
    pub repo: Arc<TaskRepository>,
}

impl Services {
    pub fn new(hub: Arc<Hub>, repo: Arc<TaskRepository>) -> Self {
        Self { hub, repo }
    }
}
