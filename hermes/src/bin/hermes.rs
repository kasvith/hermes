use hermes::{broker::Broker, config::Config};

fn main() {
    let broker = Broker::new(Config {
        host: String::from("127.0.0.1"),
        port: 1234,
    });

    broker.run();
}
