use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
    path::PathBuf,
};

use dioxus_core::Template;
#[cfg(feature = "file_watcher")]
pub use dioxus_html::HtmlCtx;
use serde::{Deserialize, Serialize};

#[cfg(feature = "custom_file_watcher")]
mod file_watcher;
#[cfg(feature = "custom_file_watcher")]
pub use file_watcher::*;

/// A message the hot reloading server sends to the client
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(bound(deserialize = "'de: 'static"))]
pub enum HotReloadMsg {
    /// A template has been updated
    UpdateTemplate(Template),

    /// An asset discovered by rsx! has been updated
    UpdateAsset(PathBuf),

    /// The program needs to be recompiled, and the client should shut down
    Shutdown,
}

/// Connect to the hot reloading listener. The callback provided will be called every time a template change is detected
pub fn connect(mut callback: impl FnMut(HotReloadMsg) + Send + 'static, host: String, port: u16) {
    std::thread::spawn(move || {
        // There might be a socket since the we're not running under the hot reloading server
        let Ok(socket) = TcpStream::connect((host.as_str(), port)) else {
            println!(
                "could not find hot reloading server at {:?}, make sure it's running",
                (host, port)
            );
            return;
        };

        let mut buf_reader = BufReader::new(socket);

        loop {
            let mut buf = String::new();

            if let Err(err) = buf_reader.read_line(&mut buf) {
                if err.kind() != std::io::ErrorKind::WouldBlock {
                    break;
                }
            }

            let Ok(template) = serde_json::from_str(Box::leak(buf.into_boxed_str())) else {
                continue;
            };

            callback(template);
        }
    });
}

/// Start the hot reloading server with the current directory as the root
#[macro_export]
macro_rules! hot_reload_init {
    () => {
        #[cfg(debug_assertions)]
        dioxus_hot_reload::init(dioxus_hot_reload::Config::new().root(env!("CARGO_MANIFEST_DIR")));
    };

    ($cfg: expr) => {
        #[cfg(debug_assertions)]
        dioxus_hot_reload::init($cfg.root(env!("CARGO_MANIFEST_DIR")));
    };
}
