# sip2-client-rs

Rust SIP2 Client Library

* The sip2::client API provides canned request types making common SIP2 
  tasks simple to implement.
* The sip2::Connection API provides complete control of the outbound messages.
* Both APIs return the response message to the caller for detailed analysis.

## Client API example


```rs

let user = "sip-user";
let pass = "sip-pass";

let mut client = Client::new("localhost", 6001)?;

let params = LoginParams::new(&user, &pass);

match client.login(&params)?.ok() {
    true => println!("Login OK"),
    false => eprintln!("Login Failed"),
}

```

## Connection API Example

```rs

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

if resp.spec().code == spec::M_LOGIN_RESP.code
    && resp.fixed_fields().len() == 1
    && resp.fixed_fields()[0].value() == "1" {

    println!("Login OK");

} else {

    println!("Login Failed");
}
```

