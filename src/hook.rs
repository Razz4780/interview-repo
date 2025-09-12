use std::sync::OnceLock;

use libc::{c_int, mode_t};
use private::OurState;

mod private {
    use libc::{c_int, mode_t};

    #[allow(unused)]
    #[derive(Debug)]
    pub enum Error {
        /// We failed to talk with our agent in the cluster.
        ConnectivityIssue,
        /// Given file descriptor is not related to any remote file.
        FdNotRemote,
    }

    /// Keeps track of descriptors and provides connection
    /// to the agent in the cluster.
    pub trait OurState: Send + Sync {
        fn is_remote(&self, fd: c_int) -> bool;

        fn remote_fchmod(&self, fd: c_int, mode: mode_t) -> Result<c_int, Error>;

        // more methods...
    }
}

/// Assume it's being set in the library constructor.
static STATE: OnceLock<Box<dyn OurState>> = OnceLock::new();

/// Hook for the `fchmod` function from libc.
/// What's wrong with it?
pub extern "C" fn fchmod_hook(fd: c_int, mode: mode_t) -> c_int {
    let state = STATE.get().expect("state should be set");

    if state.is_remote(fd) {
        state.remote_fchmod(fd, mode).expect("remote failed")
    } else {
        unsafe { libc::fchmod(fd, mode) }
    }
}
