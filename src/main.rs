use std::fmt::Display;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::result;
use std::thread;

type Result<T> = result::Result<T, ()>;

const SAFE_MODE: bool = false;

struct Sensitive<T> {
    inner: T,
}
//control the display behavior of the Sensitive structure
//based on the value of the SAFE_MODE constant.
impl<T: Display> Display for Sensitive<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if SAFE_MODE {
            writeln!(f, "[REDUCTED]")
        } else {
            writeln!(f, "{inner}", inner = self.inner)
        }
    }
}

impl<T> Sensitive<T> {
    fn new(inner: T) -> Self {
        Self { inner }
    }
}

fn client(_stream: TcpStream) {
    todo!();
}
fn main() -> Result<()> {
    let address = "192.168.28.15:8080";
    let listener = TcpListener::bind(address).map_err(|err| {
        eprintln!(
            "ERROR: could not bind {}: {}",
            Sensitive::new(address),
            Sensitive::new(err)
        );
    })?;

    println!("listen on {}", Sensitive::new(address));

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let _ = writeln!(stream, "Hello mine freund! KKraut")
                    .map_err(|err| eprintln!("ERROR: could not write message to stream: {err}"));
            }
            Err(error) => {
                eprintln!("ERROR: could not accept connection: {error} ")
            }
        }
    }
    println!("Hello, world!");

    Ok(())
}
