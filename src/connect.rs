extern crate imap;
extern crate native_tls;

use crate::connect::Connect::ErroAuth;
use imap::Client;
use native_tls::TlsConnector;
use std::io;

pub fn connect<T: AsRef<str>>(host: T, username: T, password: T, port: u16) -> Connect {
    let domain = host.as_ref();
    let tls = native_tls::TlsConnector::builder().build();
    match tls {
        Ok(TlsConnect) => {
            //sucess build
            let client = imap::connect((domain, port), domain, &TlsConnect);
            match client {
                Ok(TlsStream) => {
                    let mut imap_session = TlsStream.login(username.as_ref(), password.as_ref());
                    match imap_session {
                        Ok(AUHT) => {
                            return Connect::ConnectionSucces(
                                username.as_ref().to_string(),
                                password.as_ref().to_string(),
                            );
                        }
                        Err(e) => {
                            return ErroAuth(e.0.to_string());
                        }
                    }
                }
                Err(e) => {
                    return Connect::ErroConnection(e.to_string());
                }
            }
        }
        Err(e) => {
            return Connect::ErroTls;
        }
    }
}

#[derive(Debug)]
pub enum Connect {
    ErroTls,
    ErroConnection(String),
    ErroAuth(String),
    ConnectionSucces(String, String),
}
