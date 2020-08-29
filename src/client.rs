use tokio::{io::BufWriter, net::TcpStream};

#[derive(Debug)]
pub(crate) struct Client {
    stream: BufWriter<TcpStream>,
}

impl Client {
    pub(crate) fn new(stream: TcpStream) -> Self {
        Client {
            stream: BufWriter::new(stream),
        }
    }

    pub(crate) fn parse_packet(&self) -> crate::Result<Option<Packet>> {}
}
