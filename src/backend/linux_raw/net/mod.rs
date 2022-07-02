pub(crate) mod addr;
#[cfg(not(feature = "std"))]
pub(crate) mod ext;
pub(crate) mod read_sockaddr;
pub(crate) mod send_recv;
pub(crate) mod syscalls;
pub(crate) mod types;
pub(crate) mod write_sockaddr;