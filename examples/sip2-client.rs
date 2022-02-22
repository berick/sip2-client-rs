use std::env;
use getopts;
use sip2::*;

const HELP_TEXT: &str = r#"

Required Options:

    --sip-host
    --sip-port
    --sip-user
    --sip-pass

"#;

fn print_err(err: &str) -> String {
    format!("\n\nError: {}\n\n------{}", err, HELP_TEXT)
}

fn main() -> Result<(), Error> {

    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let mut opts = getopts::Options::new();

    opts.optopt("h", "sip-host", "SIP Host", "HOST");
    opts.optopt("p", "sip-port", "SIP Port", "PORT");
    opts.optopt("u", "sip-user", "SIP user", "user");
    opts.optopt("w", "sip-pass", "SIP pass", "PASSWORD");

    let options = opts.parse(&args[1..])
        .expect("Error parsing command line options");

    let host = options.opt_str("sip-host").expect(&print_err("--sip-host required"));
    let port = options.opt_str("sip-port").expect(&print_err("--sip-port required"));
    let user = options.opt_str("sip-user").expect(&print_err("--sip-user required"));
    let pass = options.opt_str("sip-pass").expect(&print_err("--sip-pass required"));

    let iport = port.parse::<u32>().expect(HELP_TEXT);

    let mut client = Client::new(&host, iport)?;

    match client.login(Some(&user), Some(&pass), None)?.ok() {
        true => println!("Login OK"),
        false => eprintln!("Login Failed"),
    }

    match client.sc_status()?.ok() {
        true => println!("SC Status OK"),
        false => eprintln!("SC Status Says Offline"),
    }

    let params = PatronStatusParams::new("ABCDEF");

    match client.patron_status(&params)?.ok() {
        true => println!("Patron is valid"),
        false => eprintln!("Patron is not valid"),
    }

    /*

    let mut client = Client::new(&(host + ":" + &port))?;

    send_login(&mut client, &user, &pass)?;
    send_sc_status(&mut client)
    */

    Ok(())
}

/*
fn send_login(client: &mut sip2::Client, user: &str, pass: &str) -> Result<(), Error> {
    let login = Message::new(
        &spec::M_LOGIN,
        vec![
            FixedField::new(&spec::FF_UID_ALGO, "0").unwrap(),
            FixedField::new(&spec::FF_PWD_ALGO, "0").unwrap(),
        ],
        vec![
            Field::new(spec::F_LOGIN_UID.code, &user),
            Field::new(spec::F_LOGIN_PWD.code, &pass),
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
