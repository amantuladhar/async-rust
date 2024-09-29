#[derive(Debug)]
#[repr(C)]
#[cfg_attr(target_arch = "x86_64", repr(packed))]
pub struct Event {
    pub (crate) events: u32,
    pub (crate) epoll_data: usize,
}

impl Event {
    pub fn token(&self) -> usize {
        self.epoll_data
    }
}

pub const EPOLL_CTL_ADD: i32 = 1;
/// Represents the bitflag indicating we're interested in read operation on the file handle
pub const EPOLLIN: i32 = 0x1;
/// Represents a bitflag indicating that we're interested in getting event notified with epoll set to an edge-triggered mode
pub const EPOLLET: i32 = 1 << 31;

#[link(name = "c")]
extern "C" {
    /// Syscall to create an epoll queue
    /// https://man7.org/linux/man-pages/man2/epoll_create.2.html
    pub fn epoll_create(size: i32) -> i32;
    /// Syscall to close file descriptor we get when we create our epoll instance
    /// https://man7.org/linux/man-pages/man2/close.2.html
    pub fn close(fd: i32) -> i32;
    /// Control interface we use to perform operations on our epoll interface
    /// This can be used to register an interest in events on the source
    /// https://man7.org/linux/man-pages/man2/epoll_ctl.2.html
    pub fn epoll_ctl(epfd: i32, op: i32, fd: i32, event: *mut Event) -> i32;
    /// Call that will block the curren thread and wait until one of two things happens
    pub fn epoll_wait(epfd: i32, events: *mut Event, max_events: i32, timeout: i32) -> i32;
}
