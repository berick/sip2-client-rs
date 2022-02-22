use std::env;
use getopts;
use sip2::*;

const HELP_TEXT: &str = r#"

Required Options:

    --sip-host
    --sip-port
    --sip-username
    --sip-password

"#;

fn main() -> Result<(), Error> {

    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let mut opts = getopts::Options::new();

    opts.optopt("h", "sip-host", "SIP Host", "HOST");
    opts.optopt("p", "sip-port", "SIP Port", "PORT");
    opts.optopt("u", "sip-username", "SIP Username", "USERNAME");
    opts.optopt("w", "sip-password", "SIP Password", "PASSWORD");

    let options = opts.parse(&args[1..])
        .expect("Error parsing command line options");

    let host = options.opt_str("sip-host").expect(HELP_TEXT);
    let port = options.opt_str("sip-port").expect(HELP_TEXT);

    let username = options.opt_str("sip-username").expect(HELP_TEXT);
    let password = options.opt_str("sip-password").expect(HELP_TEXT);

    let iport = port.parse::<u32>().expect(HELP_TEXT);

    let mut client = Client::new(&host, iport)?;

    client.login(&username, &password)?;

    println!("Login OK to {}", host);

    /*

    let mut client = Client::new(&(host + ":" + &port))?;

    send_login(&mut client, &username, &password)?;
    send_sc_status(&mut client)
    */

    Ok(())
}

/*
fn send_login(client: &mut sip2::Client, username: &str, password: &str) -> Result<(), Error> {
    let login = Message::new(
        &spec::M_LOGIN,
        vec![
            FixedField::new(&spec::FF_UID_ALGO, "0").unwrap(),
            FixedField::new(&spec::FF_PWD_ALGO, "0").unwrap(),
        ],
        vec![
            Field::new(spec::F_LOGIN_UID.code, &username),
            Field::new(spec::F_LOGIN_PWD.code, &password),
        ],
    );

    let resp1 = client.sendrecv(&login)?;

    println!("SIP server responded:\n{}", resp1);

    Ok(())
}

fn send_sc_status(client: &mut sip2::Client) -> Result<(), Error> {

    let sc_status = Message::new(
        &spec::M_SC_STATUS,
        vec![
            FixedField::new(&spec::FF_STATUS_CODE, "0").unwrap(),
            FixedField::new(&spec::FF_MAX_PRINT_WIDTH, "999").unwrap(),
            FixedField::new(&spec::FF_PROTOCOL_VERSION, &spec::SIP_PROTOCOL_VERSION).unwrap(),
        ],
        vec![],
    );

    let resp2 = client.sendrecv(&sc_status)?;

    println!("SIP server responded:\n{}", resp2);

    Ok(())
}

*/
