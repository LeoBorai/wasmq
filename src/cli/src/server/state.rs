use std::sync::Arc;

use crate::process::hub::Hub;

pub type SharedServices = Arc<Services>;

#[derive(Clone)]
pub struct Services {
    pub hub: Arc<Hub>,
}

impl Services {
    pub fn new(hub: Arc<Hub>) -> Self {
        Self { hub }
    }
}
