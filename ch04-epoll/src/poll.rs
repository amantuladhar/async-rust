use crate::ffi::{close, epoll_create, epoll_ctl, epoll_wait, Event, EPOLL_CTL_ADD};
use std::io;
use std::net::TcpStream;
use std::os::fd::AsRawFd;

type Events = Vec<Event>;

/// Represents the event queue itself
pub struct Poll {
    registry: Registry,
}

impl Poll {
    /// Creates a new event queue
    pub fn new() -> std::io::Result<Self> {
        let file_descriptor = unsafe {
            // Number generally doesn't matter too much nowadays
            // Needs to be greater than 0
            epoll_create(1)
        };
        if file_descriptor < 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(Self {
            registry: Registry {
                raw_fd: file_descriptor,
            },
        })
    }

    /// Returns the references to the registry that we use to
    /// register interest to be notified about the new events
    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    /// Blocks the thread it's called on
    /// until an event is ready, or it times out
    /// whichever occurs first
    pub fn poll(&mut self, events: &mut Events, timeout: Option<i32>) -> std::io::Result<()> {
        let fd = self.registry.raw_fd;
        // -1 = wait for eternity
        let timeout = timeout.unwrap_or(-1);
        let max_events = events.capacity() as i32;
        let res = unsafe { epoll_wait(fd, events.as_mut_ptr(), max_events, timeout) };
        if res < 0 {
            return Err(std::io::Error::last_os_error());
        }
        unsafe {
            events.set_len(res as usize);
        };
        Ok(())
    }
}

/// Handle that allows us to register interest in new events
pub struct Registry {
    raw_fd: i32,
}

impl Registry {
    pub fn register(&self, source: &TcpStream, token: usize, interests: i32) -> io::Result<()> {
        let mut event = Event {
            events: interests as u32,
            epoll_data: token,
        };
        let op = EPOLL_CTL_ADD;
        let res = unsafe { epoll_ctl(self.raw_fd, op, source.as_raw_fd(), &mut event) };
        if res < 0 {
            return Err(io::Error::last_os_error());
        };
        Ok(())
    }
}

impl Drop for Registry {
    fn drop(&mut self) {
        let res = unsafe { close(self.raw_fd) };
        if res < 0 {
            let err = std::io::Error::last_os_error();
            eprintln!("[ERROR]: {err:?}")
        }
    }
}
