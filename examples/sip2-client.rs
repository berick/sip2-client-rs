use std::env;
use getopts;
use sip2::*;

const HELP_TEXT: &str = r#"

Required:

    --sip-host
    --sip-port
    --sip-user
    --sip-pass

Optional:

    --patron-barcode
    --patron-pass
    --item-barcode

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
    opts.optopt("u", "sip-user", "SIP User", "USER");
    opts.optopt("w", "sip-pass", "SIP pass", "PASSWORD");

    // Optional
    opts.optopt("b", "patron-barcode", "Patron Barcode", "PATRON-BARCODE");
    opts.optopt("a", "patron-pass", "Patron Password", "PATRON-PASSWORD");
    opts.optopt("i", "item-barcode", "Item Barcode", "ITEM-BARCODE");

    let options = opts.parse(&args[1..])
        .expect("Error parsing command line options");

    let host = options.opt_str("sip-host").expect(&print_err("--sip-host required"));
    let port = options.opt_str("sip-port").expect(&print_err("--sip-port required"));
    let user = options.opt_str("sip-user").expect(&print_err("--sip-user required"));
    let pass = options.opt_str("sip-pass").expect(&print_err("--sip-pass required"));

    let iport = port.parse::<u32>().expect(HELP_TEXT);

    let mut client = Client::new(&host, iport)?;

    let params = LoginParams::new(&user, &pass);

    match client.login(&params)?.ok() {
        true => println!("Login OK"),
        false => eprintln!("Login Failed"),
    }

    match client.sc_status()?.ok() {
        true => println!("SC Status OK"),
        false => eprintln!("SC Status Says Offline"),
    }

    // --- PATRON STUFF ---

    if let Some(patron_id) = options.opt_str("patron-barcode") {

        let mut params = PatronStatusParams::new(&patron_id);
        params.patron_pwd = options.opt_str("patron-pass");

        let resp = client.patron_status(&params)?;

        match resp.ok() {
            true => {
                println!("Patron Info reports valid");
                if let Some(name) = resp.msg().get_field_value("AE") {
                    println!("Patron name is '{}'", name);
                }
            },
            false => eprintln!("Patron Info reports not valid"),
        }


        let mut params = PatronInfoParams::new(&patron_id);
        params.patron_pwd = options.opt_str("patron-pass");
        params.summary = Some(2);

        let resp = client.patron_info(&params)?;

        match resp.ok() {
            true => {
                println!("Patron Info reports valid");
                if let Some(name) = resp.msg().get_field_value("AE") {
                    println!("Patron name is '{}'", name);
                }
            },
            false => eprintln!("Patron Info reports not valid"),
        }
    }

    if let Some(item_id) = options.opt_str("item-barcode") {
        let params = ItemInfoParams::new(&item_id);
        let resp = client.item_info(&params)?;

        match resp.ok() {
            true => {
                println!("Item Info reports valid");
                println!("Item title is '{}'", resp.msg().get_field_value("AJ").unwrap());
            },
            false => eprintln!("Item Info reports not valid"),
        }
    }

    Ok(())
}

