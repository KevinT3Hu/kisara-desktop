use tokio::sync::oneshot::{channel, Receiver, Sender};

use crate::handlers::PlayServeInfo;

#[derive(Default)]
pub struct ServeSignal {
    sender: Option<Sender<()>>,
    torrent_id: Option<String>,
    info: Option<PlayServeInfo>,
}

impl ServeSignal {
    pub fn send(&mut self) {
        if let Some(sender) = self.sender.take() {
            println!("Sending stop signal");
            sender.send(()).expect("This should work.");
        }
    }

    pub fn reset(&mut self, torrent_id: &str) -> Option<Receiver<()>> {
        // if torrent_id not change, ignore
        if let Some(ref id) = self.torrent_id {
            if id == torrent_id && self.info.is_some() {
                return None;
            }
        }
        self.torrent_id = Some(torrent_id.to_owned());
        self.send();
        let (sender, receiver) = channel();
        self.sender = Some(sender);
        Some(receiver)
    }

    pub fn set_info(&mut self, info: PlayServeInfo) {
        self.info = Some(info);
    }

    pub fn get_info(&self) -> Option<PlayServeInfo> {
        self.info.clone()
    }
}
