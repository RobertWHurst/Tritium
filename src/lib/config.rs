use std::path::PathBuf;
use http::HttpServerConfig;

pub struct Config {
    source_path: Option<PathBuf>,
    // udp_servers: Vec<UdpServerConfig>,
    // tcp_servers: Vec<TcpServerConfig>,
    http_servers: Vec<HttpServerConfig>,
    // https_servers: Vec<HttpsServerConfig>,
    // http2_servers: Vec<Http2ServerConfig>,
}

impl Config {
    pub fn from_path<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        let path = path.into();

        // TODO: this is where we will generate all of the config instances

        Self {
            source_path: Some(path),
            http_servers: Vec::new(),
        }
    }
}
