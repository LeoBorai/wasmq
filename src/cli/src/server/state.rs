use std::sync::Arc;

use mate_config::Config;
use mate_repository::TaskRepository;

use crate::process::hub::Hub;

pub type SharedServices = Arc<Services>;

#[derive(Clone)]
pub struct Services {
    pub config: Arc<Config>,
    pub hub: Arc<Hub>,
    pub repo: Arc<TaskRepository>,
}

impl Services {
    pub fn new(config: Arc<Config>, hub: Arc<Hub>, repo: Arc<TaskRepository>) -> Self {
        Self { hub, repo, config }
    }
}
