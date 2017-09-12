use std::path::PathBuf;

pub struct Proxy {
    config_path: Option<PathBuf>,
}

impl Proxy {
    pub fn new() -> Self {
        Self { config_path: None }
    }

    pub fn load_config<P>(&mut self, config_path: P)
    where
        P: Into<PathBuf>,
    {
        self.config_path = Some(config_path.into());
        // TODO: here we will want to call instance methods on Proxy in order to configure the
        // instance.
    }

    pub fn config_path(&self) -> Option<&PathBuf> {
        self.config_path.as_ref()
    }

    pub fn run(&mut self) {
        loop {
            // Where the magic happens
        }
    }
}
