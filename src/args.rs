use std::env::args;
use std::process::{exit, ExitCode};

#[derive(Debug)]
pub struct argumentos<T>
where
    T: AsRef<str>,
{
    pub filename: T,
    pub ThreadPollNumber: i64,
    pub filenameProxys: Option<T>,
}
impl<T: AsRef<str>> argumentos<T> {
    pub fn new() -> Result<argumentos<String>, ExitCode> {
        let argsEnv: Vec<String> = args().collect();
        if argsEnv.len() < 3 {
            eprintln!("./bin filename ,  number_tasks , filename_proxys opcional ");
            exit(1);
        }
        let filename = argsEnv[1].as_str();
        let numero_threads: i64 = argsEnv[2].parse().unwrap_or(50);
        let mut filenameProxy = None;
        if let Some(proxyFilename) = argsEnv.get(3) {
            filenameProxy = Some(proxyFilename.to_owned());
        }
        return Ok(argumentos {
            filename: filename.to_string(),
            ThreadPollNumber: numero_threads,
            filenameProxys: filenameProxy,
        });
    }
}
