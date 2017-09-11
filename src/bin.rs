extern crate tritium;

use tritium::Proxy;

fn main() {
    let mut proxy = Proxy::new();
    // TODO: We need to discuss what our configuration looks like, and how to load it.
    // proxy.load_config();
    proxy.start();
}
