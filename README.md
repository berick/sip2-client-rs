# sip2-client-rs

Rust SIP2 Client Library

* The sip2::client API provides canned request types making common SIP2 
  tasks simple to implement.
* The sip2::Connection API provides complete control of how each message
  is structured.
* Both APIs return the response message to the caller.
* See examples/sip2-client.rs for more examples.

## Client API example


```rs
use sip2::*;

let user = "sip-user";
let pass = "sip-pass";

let mut client = Client::new("localhost", 6001)?;

let params = LoginParams::new(&user, &pass);
let resp = client.login(&params)?;

prinln!("Received: {}", resp.msg());

match resp.ok() {
    true => println!("Login OK"),
    false => eprintln!("Login Failed"),
}

```

## Connection API Example

```rs
use sip2::*;

let con = Connection::new("localhost:6001")?;

let user = "sip-user";
let pass = "sip-pass";

let req = Message::new(
    &spec::M_LOGIN,
    vec![
        FixedField::new(&spec::FF_UID_ALGO, "0").unwrap(),
        FixedField::new(&spec::FF_PWD_ALGO, "0").unwrap(),
    ],
    vec![
        Field::new(spec::F_LOGIN_UID.code, user),
        Field::new(spec::F_LOGIN_PWD.code, pass),
    ]
);

let resp = con.sendrecv(&req)?;

println!("Received: {}", resp);

if resp.spec().code == spec::M_LOGIN_RESP.code
    && resp.fixed_fields().len() == 1
    && resp.fixed_fields()[0].value() == "1" {

    println!("Login OK");

} else {

    println!("Login Failed");
}

```

