use crate::ffi::{Event, EPOLLET, EPOLLIN};
use crate::poll::Poll;
use std::alloc::handle_alloc_error;
use std::collections::HashSet;
use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

mod ffi;
mod poll;

fn main() -> io::Result<()> {
    let mut poll = Poll::new()?;
    let n_events = 5;
    let mut streams = vec![];
    let addr = "localhost:8080";
    for i in 0..n_events {
        let delay = (n_events - 1) * 1000;
        let url_path = format!("/{delay}/request-{i}");
        let request = get_req(&url_path);
        let mut stream = TcpStream::connect(addr)?;
        stream.set_nonblocking(true)?;
        stream.write_all(&request)?;
        poll.registry()
            .register(&stream, i, EPOLLIN | EPOLLET)
            .unwrap();
        streams.push(stream)
    }

    let mut handled_ids = HashSet::new();
    let mut handled_events = 0;
    while handled_events < n_events {
        let mut events = Vec::with_capacity(10);
        poll.poll(&mut events, None)?;
        if events.is_empty() {
            println!("TIMEOUT (or ..)");
            continue;
        }
        handled_events += handle_events(&events, &mut streams, &mut handled_ids)?;
    }
    println!("FINISHED");
    Ok(())
}

fn handle_events(
    events: &Vec<Event>,
    stream: &mut Vec<TcpStream>,
    handled_ids: &mut HashSet<usize>,
) -> std::io::Result<usize> {
    let mut handle_events = 0;
    for event in events {
        let index = event.token();
        let mut data = vec![0u8; 4096];

        loop {
            // doing it on loop to make sure we drained the buffer
            // Unlike level triggered events where high level of voltage triggers event
            // edge-trigger like our implementation triggers only when voltage changes
            // Meaning if we don't drain our buffer, we won't receive any events in future
            match stream[index].read(&mut data) {
                Ok(n) if n == 0 => {
                    if !handled_ids.insert(index) {
                        break;
                    }
                    handle_events += 1;
                    break;
                }
                // No breaks here because buffer is still not drained
                Ok(n) => {
                    let txt = String::from_utf8_lossy(&data[..n]);

                    println!("RECEIVED: {:?}", event);
                    println!("{txt}\n------\n");
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
                // this was not in the book example, but it's a error condition
                // you probably want to handle in some way (either by breaking
                // out of the loop or trying a new read call immediately)
                Err(e) if e.kind() == io::ErrorKind::Interrupted => break,
                Err(e) => return Err(e),
            };
        }
    }
    Ok(handle_events)
}

fn get_req(path: &str) -> Vec<u8> {
    format!("GET {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n").into()
}
