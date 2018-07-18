extern crate mio;
extern crate mio_extras;

use mio::*;
use mio_extras::channel;
use mio_extras::channel::Receiver;
use mio::tcp::{TcpListener, TcpStream};
use std::process::{Command, ExitStatus};
use std::thread;
use test_result::TestResult;
use config::*;
use TestConfig;

const SERVER: Token = mio::Token(1);
const STATUS: Token = mio::Token(2);

#[allow(dead_code)]
pub struct Agent {
    pub name: String,
    path: String,
    args: Vec<String>,
    pub socket: TcpStream,
    child: Receiver<i32>,
    pub alive: bool,
    exit_value: Option<ExitStatus>,
}


impl Agent {
    pub fn new(name: &str,
               path: &str,
               agent: &Option<TestCaseAgent>,
               args: Vec<String>,
               conf: &TestConfig)
               -> Result<Agent, i32> {
        // IPv6 listener by default, IPv4 fallback, unless IPv4 is forced.
        let addr6 = "[::1]:0".parse().unwrap();
        let addr4 = "127.0.0.1:0".parse().unwrap();
        let listener = match conf.force_ipv4 {
            false => TcpListener::bind(&addr6).or_else(|_| {
                TcpListener::bind(&addr4)}).unwrap(),
            true => TcpListener::bind(&addr4).unwrap(),
        };

        // Start the subprocess.
        let mut command = Command::new(path.to_owned());
        // Process parameters.
        if let Some(ref a) = *agent {
            if let Some(ref min) = a.min_version {
                command.arg("-min-version");
                command.arg(min.to_string());
            }
            if let Some(ref min) = a.max_version {
                command.arg("-max-version");
                command.arg(min.to_string());
            }
            if let Some(ref flags) = a.flags {
                for f in flags {
                    command.arg(f);
                }
            }
        }

        // Add specific args.
        for arg in &args {
            command.arg(arg);
        }

        // Add common args.
        command.arg("-port");
        command.arg(listener.local_addr().unwrap().port().to_string());
        debug!("Executing command {:?}", &command);
        let mut child = command.spawn().unwrap();

        // Listen for connect
        // Create an poll instance
        let poll = Poll::new().unwrap();
        poll.register(&listener, SERVER, Ready::readable(), PollOpt::level())
            .unwrap();
        let mut events = Events::with_capacity(1024);

        // This is gross, but we can't reregister channels.
        // https://github.com/carllerche/mio/issues/506
        let (txf, rxf) = channel::channel::<i32>();
        let (txf2, rxf2) = channel::channel::<i32>();

        poll.register(&rxf, STATUS, Ready::readable(), PollOpt::level())
            .unwrap();

        thread::spawn(move || {
            let ecode = child.wait().expect("failed waiting for subprocess");
            txf.send(ecode.code().unwrap_or(-1)).ok();
            txf2.send(ecode.code().unwrap_or(-1)).ok();
        });

        poll.poll(&mut events, None).unwrap();
        debug!("Poll finished!");

        match events.iter().next().unwrap().token() {
            SERVER => {
                let sock = listener.accept();

                debug!("Accepted");
                Ok(Agent {
                    name: name.to_owned(),
                    path: path.to_owned(),
                    args: args,
                    socket: sock.unwrap().0,
                    child: rxf2,
                    alive: true,
                    exit_value: None,
                })
            }
            STATUS => {
                let err = rxf.try_recv().unwrap();
                info!("Failed {}", err);
                Err(err)
            }
            _ => Err(-1)
        }
    }

    // Read the status from the subthread.
    pub fn check_status(&self) -> TestResult {
        debug!("Getting status for {}", self.name);
        // try_recv() is nonblocking, so poll until it's readable.
        let poll = Poll::new().unwrap();
        poll.register(&self.child, STATUS, Ready::readable(), PollOpt::level())
            .unwrap();
        let mut events = Events::with_capacity(1);
        poll.poll(&mut events, None).unwrap();

        let code = self.child.try_recv().unwrap();
        debug!("Exit status for {} = {}", self.name, code);
        TestResult::from_status(code)
    }
}
