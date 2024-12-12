use crate::worker;

pub struct Runtime {
    pub worker_sender: tokio::sync::mpsc::UnboundedSender<worker::Message>,
}

impl rustler::Resource for Runtime {}
