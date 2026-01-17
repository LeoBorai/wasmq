use std::sync::Arc;

use mate_ipc::channel::IpcServer;

pub type SharedServices = Arc<Services>;

#[derive(Clone)]
pub struct Services {
    pub ipc_server: Arc<IpcServer>,
}

impl Services {
    pub fn new(ipc_server: Arc<IpcServer>) -> Self {
        Self { ipc_server }
    }
}
