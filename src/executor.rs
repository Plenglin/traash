use std::ffi::{CStr, CString, NulError};
use std::os::unix::io::RawFd;

use libc::WEXITED;
use nix::sys::signal::Signal::{SIGKILL, SIGTERM};
use nix::sys::signal::{kill, Signal};
use nix::sys::wait::{waitpid, WaitPidFlag};
use nix::unistd::{dup2, execv, fork, pipe, ForkResult, Pid};
use nix::{libc, Error};

use crate::ast::{BinaryExpr, BinaryOp, Command, SingleCommand};

pub struct StreamSet {
    stdin: Option<RawFd>,
    stdout: Option<RawFd>,
    stderr: Option<RawFd>,
}

impl StreamSet {
    pub fn std() -> StreamSet {
        StreamSet {
            stdin: Some(0),
            stdout: Some(1),
            stderr: Some(2),
        }
    }

    pub fn pipe(self: StreamSet) -> (StreamSet, StreamSet) {
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

    pub fn fork(self: StreamSet) -> (StreamSet, StreamSet) {
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

pub enum ProcessSpawnError {
    NixError(nix::Error),
    NulError(NulError),
}
impl From<nix::Error> for ProcessSpawnError {
    fn from(err: Error) -> Self {
        ProcessSpawnError::NixError(err)
    }
}
impl From<NulError> for ProcessSpawnError {
    fn from(err: NulError) -> Self {
        ProcessSpawnError::NulError(err)
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
            kill(self.pid, SIGTERM);
        }
    }
}

impl Process {
    fn spawn(
        cmd: SingleCommand,
        attached: bool,
        streams: StreamSet,
    ) -> Result<Process, ProcessSpawnError> {
        unsafe {
            match fork()? {
                ForkResult::Parent { child } => {
                    return Ok(Process {
                        pid: child,
                        attached,
                        streams,
                    })
                }
                ForkResult::Child => {
                    let path = CString::new(cmd.args[0].as_str())?;
                    /* let argv = cmd
                    .args
                    .into_iter()
                    .skip(1)
                    .map(|x| CString::new(x.as_str()) as Result<CString, NulError>)
                    .collect();*/
                    execv(path.as_c_str(), &[]);
                    panic!("code after execv() should never happen!");
                }
            }
        }
    }
}

unsafe fn execute_single(
    command: SingleCommand,
    streams: StreamSet,
) -> Result<i32, ProcessSpawnError> {
    let p = Process::spawn(command, true, streams)?;
    waitpid(p, WaitPidFlag::from_bits(WEXITED));
}

unsafe fn execute_binary(binary: BinaryExpr, streams: StreamSet) -> Result<i32, ProcessSpawnError> {
    match binary.op {
        BinaryOp::Fork => {
            let (l, r) = streams.fork();
            execute(*binary.first, l);
            Ok(execute(*binary.second, r))
        }
        BinaryOp::Seq => {
            let first = execute(*binary.first, streams);
            waitpid(first, WaitPidFlag::from_bits(WEXITED));
            let second = execute(*binary.second, streams.clone());
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

pub fn execute(cmd: Command, streams: StreamSet) {
    unsafe {
        match cmd {
            Command::Nil => {}
            Command::Single(c) => execute_single(c, streams),
            Command::BinaryExpr(c) => execute_binary(c, streams),
            Command::FileInput(_) => {}
            Command::FileOutput(_) => {}
        }
    }
}
