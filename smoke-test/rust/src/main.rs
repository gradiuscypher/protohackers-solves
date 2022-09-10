use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

fn handle_request(mut stream: TcpStream) {
    let mut data = [0 as u8; 4096];
    while match stream.read(&mut data) {
        Ok(size) => {
            if size > 0 {
                println!("Stream size: {}", size);
                println!("got some data: {:?}", &data[0..size]);
                match stream.write(&data[0..size]) {
                    Ok(_) => {}
                    Err(err) => {
                        println!("Error while writing back: {}", err)
                    }
                }
                true
            } else {
                match stream.shutdown(Shutdown::Write) {
                    Ok(_) => {
                        println!("Shutting down connection.")
                    }
                    Err(err) => {
                        println!("Error while shutting down: {}", err)
                    }
                }
                false
            }
        }
        Err(err) => {
            println!("Error while reading: {:?}", err);
            // match stream.shutdown(Shutdown::Write) {
            //     Ok(_) => {}
            //     Err(err) => {
            //         println!("Error while shutting down: {}", err)
            //     }
            // }
            false
        }
    } {}
}

fn new_handler(mut stream: TcpStream) {
    loop {
        let mut data = [0; 1024];
        match stream.read(&mut data) {
            Ok(size) => {
                if size == 0 {
                    stream.shutdown(Shutdown::Write).unwrap();
                    break;
                }
                stream.write(&data[0..size]).unwrap();
            }
            Err(err) => {
                println!("Error while reading: {}", err)
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8888")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || new_handler(stream));
            }
            Err(err) => {
                println!("Error while accepting connection: {}", err)
            }
        }
    }

    drop(listener);

    Ok(())
}
