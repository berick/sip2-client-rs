use std::env;
use getopts;
use sip2::*;

fn main() -> Result<(), Error> {

    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let mut opts = getopts::Options::new();

    opts.optopt("h", "sip-host", "SIP Host", "HOST");
    opts.optopt("p", "sip-port", "SIP Port", "PORT");
    opts.optopt("u", "sip-username", "SIP Username", "USERNAME");
    opts.optopt("w", "sip-password", "SIP Password", "PASSWORD");

    let options = match opts.parse(&args[1..]) {
        Ok(o) => o,
        Err(s) => panic!("Failure parsing options: {}", s),
    };

    let host = match options.opt_str("sip-host") {
        Some(h) => h,
        None => panic!("SIP host required"),
    };

    let port = match options.opt_str("sip-port") {
        Some(p) => p,
        None => panic!("SIP port required"),
    };

    let username = match options.opt_str("sip-username") {
        Some(p) => p,
        None => panic!("SIP username required"),
    };

    let password = match options.opt_str("sip-password") {
        Some(p) => p,
        None => panic!("SIP password required"),
    };

    println!("host = {}, port = {}", host, port);

    let mut client = Client::new(&(host + ":" + &port))?;

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

