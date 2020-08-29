use super::config::Config;
use tokio::{
    io,
    net::{TcpListener, TcpStream},
};

pub struct Broker {
    config: Config,
}

pub struct Handler {
    stream: TcpStream,
}

impl Handler {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    async fn handle(mut self) {
        let (mut rd, mut wr) = self.stream.split();

        if io::copy(&mut rd, &mut wr).await.is_err() {
            eprintln!("failed to copy");
        }
    }
}

impl Broker {
    pub fn new(cfg: Config) -> Self {
        Broker { config: cfg }
    }

    #[tokio::main]
    pub async fn run(&self) {
        let addr = format!("{}:{}", self.config.host, self.config.port);
        let mut listener = TcpListener::bind(addr).await.unwrap();

        loop {
            // The second item contains the ip and port of the new connection.
            let (socket, _) = listener.accept().await.unwrap();

            tokio::spawn(async move {
                let handler = Handler::new(socket);
                handler.handle().await;
            });
        }
    }
}
