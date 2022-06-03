use std::collections::HashMap;
use std::env;
/// SERVER
use std::io::{self, ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 1024;

fn sleep() {
    thread::sleep(std::time::Duration::from_millis(100));
}

fn start_server(name: String) {
    let server = TcpListener::bind(LOCAL).expect("Listener failed to bind");
    server
        .set_nonblocking(true)
        .expect("failed to initialize non-blocking");

    // let mut users = HashMap::new();
    let mut clients = vec![];
    let (tx, rx) = mpsc::channel::<String>();

    loop {
        // Only fires when a client JOINS
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr);

            let tx = tx.clone();
            clients.push(socket.try_clone().expect("failed to clone client"));

            let mut buff = vec![0; MSG_SIZE];

            // Gets the username
            match socket.read_exact(&mut buff) {
                Ok(_) => {
                    let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                    let msg = format!("{}", String::from_utf8(msg).expect("Invalid utf8 message"));
                    println!("USERNAME: {:?}", msg);
                }
                Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                Err(_) => {
                    println!("closing connection with: {}", addr);
                    break;
                }
            }

            thread::spawn(move || loop {
                let mut buff = vec![0; MSG_SIZE];

                // Waits until message is sent from client
                match socket.read_exact(&mut buff) {
                    Ok(_) => {
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg =
                            format!("{}", String::from_utf8(msg).expect("Invalid utf8 message"));

                        println!("{}: {:?}", addr, msg);
                        // Sends message to receiver
                        tx.send(msg).expect("failed to send msg to rx");
                    }
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("closing connection with: {}", addr);
                        break;
                    }
                }

                sleep();
            });
        }

        // Receives message
        if let Ok(msg) = rx.try_recv() {
            clients = clients
                .into_iter()
                .filter_map(|mut client| {
                    let mut buff = msg.clone().into_bytes();
                    buff.resize(MSG_SIZE, 0);

                    client.write_all(&buff).map(|_| client).ok()
                })
                .collect::<Vec<_>>();
        }

        sleep();
    }
}

fn start_client(name: String) {
    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    client
        .set_nonblocking(true)
        .expect("failed to initiate non-blocking");

    let (tx, rx) = mpsc::channel::<String>();
    tx.send(name).expect("failed to send msg to rx");

    // Loops to check if client has entered message in terminal
    thread::spawn(move || loop {
        // Reads if other clients send a message
        let mut buff = vec![0; MSG_SIZE];
        match client.read_exact(&mut buff) {
            Ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                let msg = format!("{}", String::from_utf8_lossy(&msg));
                println!("{}", msg);
            }
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                break;
            }
        }

        match rx.try_recv() {
            Ok(msg) => {
                let msg = format!("{}", msg);
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).expect("writing to socket failed");
                //        println!("Client: {:?}", client.read_exact(&mut buff));
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }

        sleep();
    });

    println!("Write a Message:");
    loop {
        let mut buff = String::new();
        io::stdin()
            .read_line(&mut buff)
            .expect("reading from stdin failed");
        let msg = buff.trim().to_string();
        if msg == ":q" || tx.send(msg).is_err() {
            break;
        }
    }

    println!("Session terminated")
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let name = args[2].clone();
    if &args[1] == "host" {
        start_server(name);
    } else if &args[1] == "connect" {
        start_client(name);
    } else {
        panic!("Argument 1 neither 'host' or 'connect'");
    }
}
