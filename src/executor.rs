use std::ffi::{CStr, CString};
use std::os::unix::io::RawFd;

use libc::WEXITED;
use nix::sys::signal::Signal::{SIGKILL, SIGTERM};
use nix::sys::signal::{kill, Signal};
use nix::sys::wait::{waitpid, WaitPidFlag};
use nix::unistd::{dup2, execv, fork, pipe, ForkResult, Pid};
use nix::{libc, Error};

use crate::ast::{BinaryExpr, BinaryOp, Command, SingleCommand};
use crate::executor::ProcessType::Attached;

pub struct StreamSet {
    stdin: Option<RawFd>,
    stdout: Option<RawFd>,
    stderr: Option<RawFd>,
}

impl StreamSet {
    fn get_default() -> StreamSet {
        StreamSet {
            stdin: Some(0),
            stdout: Some(1),
            stderr: Some(2),
        }
    }

    fn pipe(self: StreamSet) -> (StreamSet, StreamSet) {
        let (r_stdin, l_stdout) = pipe().unwrap();
        (
            StreamSet {
                stdin: self.stdin,
                stdout: Some(l_stdout),
                stderr: self.stderr,
            },
            StreamSet {
                stdin: Some(r_stdin),
                stdout: self.stdout,
                stderr: self.stderr.clone(),
            },
        )
    }

    fn fork(self: StreamSet) -> (StreamSet, StreamSet) {
        (
            StreamSet {
                stdin: None,
                stdout: self.stdout,
                stderr: self.stderr,
            },
            StreamSet {
                stdin: self.stdin,
                stdout: self.stdout.clone(),
                stderr: self.stderr.clone(),
            },
        )
    }
}

pub struct Process {
    pid: Pid,
    attached: bool,
    streams: StreamSet,
}

impl Drop for Process {
    fn drop(&mut self) {
        if self.attached {
            kill(pid, SIGTERM);
        }
    }
}

impl Process {
    fn spawn(cmd: SingleCommand, attached: bool, streams: StreamSet) -> Process {
        unsafe {
            match fork()? {
                ForkResult::Parent { child } => {
                    return Process {
                        pid: child,
                        attached,
                        streams,
                    }
                }
                ForkResult::Child => {
                    let path = CString::from(&cmd.args[0]);
                    execv(path.as_c_str(), &cmd.args[1..]);
                    panic!("code after execv() should never happen!");
                }
            }
        }
    }
}

unsafe fn execute_binary(binary: BinaryExpr, streams: StreamSet) {
    match binary.op {
        BinaryOp::Fork => {
            execute(*binary.first);
            execute(*binary.second);
        }
        BinaryOp::Seq => {
            let first = execute(*binary.first);
            waitpid(first, WaitPidFlag::from_bits(WEXITED));
            let second = execute(*binary.second);
            waitpid(second, WaitPidFlag::from_bits(WEXITED));
        }
        BinaryOp::Pipe => {
            let (left, right) = streams.pipe();
            let first = execute(*binary.first, left);
            let second = execute(*binary.second, right);
            waitpid(second, WaitPidFlag::from_bits(WEXITED));
        }
        BinaryOp::LogAnd => {}
        BinaryOp::LogOr => {}
    }
}

fn execute(cmd: Command, streams: StreamSet) {
    unsafe {
        match cmd {
            Command::Nil => {}
            Command::Single(c) => execute_single(c),
            Command::BinaryExpr(c) => execute_binary(c),
            Command::FileInput(_) => {}
            Command::FileOutput(_) => {}
        }
    }
}
