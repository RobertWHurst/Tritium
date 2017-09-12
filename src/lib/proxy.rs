use config::Config;
use std::path::PathBuf;

pub struct Proxy {
    config: Option<Config>,
}

impl Proxy {
    pub fn new() -> Self {
        Self { config: None }
    }

    pub fn load_config<P>(&mut self, config_path: P)
    where
        P: Into<PathBuf>,
    {
        self.config = Some(Config::from_path(config_path));
        // TODO: here we will want to call instance methods on Proxy in order to configure the
        // instance.
    }

    pub fn config(&self) -> Option<&Config> {
        self.config.as_ref()
    }

    pub fn run(&mut self) {
        loop {
            // Where the magic happens
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_proxy_with_new() {
        let _: Proxy = Proxy::new();
    }

    #[test]
    fn can_set_config_path_with_load_config() {
        let mut proxy = Proxy::new();
        proxy.load_config("my_config");
    }

    #[test]
    fn can_get_config_path() {
        let mut proxy = Proxy::new();

        proxy.load_config("my_config");
        let _: &Config = proxy.config().unwrap();
    }
}
