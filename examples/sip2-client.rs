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
    opts.optopt("l", "location-code", "Location Code", "LOCATION-CODE");

    let options = opts.parse(&args[1..])
        .expect("Error parsing command line options");

    let host = options.opt_str("sip-host").expect(&print_err("--sip-host required"));
    let port = options.opt_str("sip-port").expect(&print_err("--sip-port required"));
    let user = options.opt_str("sip-user").expect(&print_err("--sip-user required"));
    let pass = options.opt_str("sip-pass").expect(&print_err("--sip-pass required"));

    let iport = port.parse::<u32>().expect(HELP_TEXT);

    let mut client = Client::new(&host, iport)?;

    let mut builder = ParamBuilder::new();
    builder.set_sip_user(&user).set_sip_pass(&pass);

    if let Some(location) = options.opt_str("location-code") {
        builder.set_location(&location);
    }

    match client.login(&builder)?.ok() {
        true => println!("Login OK"),
        false => eprintln!("Login Failed"),
    }

    match client.sc_status()?.ok() {
        true => println!("SC Status OK"),
        false => eprintln!("SC Status Says Offline"),
    }

    // --- PATRON STUFF ---

    if let Some(patron_id) = options.opt_str("patron-barcode") {

        builder.set_patron_id(&patron_id);
        if let Some(pass) = options.opt_str("patron-pass") {
            builder.set_patron_pwd(&pass);
        }

        let resp = client.patron_status(&builder)?;

        match resp.ok() {
            true => {
                println!("Patron Info reports valid");
                if let Some(name) = resp.value("AE") {
                    println!("Patron name is '{}'", name);
                }
            },
            false => eprintln!("Patron Info reports not valid"),
        }


        builder.set_summary(2);

        let resp = client.patron_info(&builder)?;

        match resp.ok() {
            true => {
                println!("Patron Info reports valid");
                if let Some(name) = resp.value("AE") {
                    println!("Patron name is '{}'", name);
                }
            },
            false => eprintln!("Patron Info reports not valid"),
        }
    }

    // ----- Item Stuff -----

    if let Some(item_id) = options.opt_str("item-barcode") {
        builder.set_item_id(&item_id);
        let resp = client.item_info(&builder)?;

        match resp.ok() {
            true => {
                println!("Item Info reports valid");
                println!("Item title is '{}'", resp.value("AJ").unwrap());
            },
            false => eprintln!("Item Info reports not valid"),
        }
    }

    Ok(())
}

