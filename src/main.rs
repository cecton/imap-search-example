extern crate clap;
extern crate imap;
extern crate native_tls;
extern crate rpassword;

use clap::{App, Arg};
use imap::error::Result;
use std::collections::HashSet;
use std::net;
use std::ops;
use std::str;

const NAME: &'static str = env!("CARGO_PKG_NAME");

struct ImapSession {
    session: imap::Session<native_tls::TlsStream<net::TcpStream>>,
}

impl ImapSession {
    fn new(domain: &str, user: &str, password: &str) -> Result<ImapSession> {
        let tls = native_tls::TlsConnector::builder().build()?;
        let client = imap::connect((domain, 993), domain, &tls)?;
        let session = client.login(user, password).map_err(|e| e.0)?;

        Ok(ImapSession { session })
    }
}

impl Drop for ImapSession {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        self.session.logout();
    }
}

impl ops::Deref for ImapSession {
    type Target = imap::Session<native_tls::TlsStream<net::TcpStream>>;

    fn deref(&self) -> &Self::Target {
        &self.session
    }
}

impl ops::DerefMut for ImapSession {
    fn deref_mut(&mut self) -> &mut imap::Session<native_tls::TlsStream<net::TcpStream>> {
        &mut self.session
    }
}

fn search(session: &mut ImapSession, mailbox: &str, query: &str) -> Result<HashSet<u32>> {
    session.select(mailbox)?;
    let ids = session.uid_search(query)?;

    Ok(ids)
}

fn main() {
    let matches = App::new(NAME)
        .arg(
            Arg::with_name("domain")
                .required(true)
                .help("Ex.: imap.example.org"),
        ).arg(Arg::with_name("user").required(true).help("Username"))
        .arg(Arg::with_name("mailbox").required(true).help("Ex.: INBOX"))
        .arg(
            Arg::with_name("query")
                .required(true)
                .help("Ex.: SUBJECT \"example\""),
        ).get_matches();

    let domain = matches.value_of("domain").unwrap();
    let user = matches.value_of("user").unwrap();
    let mailbox = matches.value_of("mailbox").unwrap();
    let query = matches.value_of("query").unwrap();
    let password = rpassword::prompt_password_stderr("Password: ").unwrap();
    eprintln!();
    match ImapSession::new(domain, user, &password) {
        Err(err) => eprintln!("{}", err),
        Ok(mut session) => match search(&mut session, mailbox, query) {
            Err(err) => eprintln!("{}", err),
            Ok(ids) => println!("Found: {:?}", ids),
        },
    }
}
