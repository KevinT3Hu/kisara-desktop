#![allow(
    clippy::let_underscore_must_use,
    clippy::used_underscore_binding,
    clippy::used_underscore_items,
    clippy::needless_pass_by_value
)]
mod anime;
mod settings;
mod torrent;
mod watch;
mod window;

pub use anime::*;
pub use settings::*;
pub use torrent::*;
pub use watch::*;
pub use window::*;
