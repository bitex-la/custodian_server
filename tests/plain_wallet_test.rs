#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate custodian_server;

#[cfg(test)]
mod wallet_test {
    use rocket;
    use rocket::local::Client;
    use rocket::http::Status;
    use std::fs::File;
    use custodian_server::server_state::ServerState;
    use custodian_server::handlers::plain_wallets;
    use rocket::http::ContentType;

    fn rocket() -> rocket::Rocket {
        let f = File::create("/dev/null").unwrap();
        let state: ServerState = ServerState::new("./tests/btc-testnet.cfg", &f, &f).expect("Error creating State");

        rocket::ignite().manage(state).mount("/", routes![plain_wallets::index, plain_wallets::create, plain_wallets::update, plain_wallets::destroy])
    }

    #[test]
    fn get_wallets_empty_data() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/plain_wallets").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("{\"data\":[]}".into()));
    }

    #[test]
    fn creates_plain_wallet() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let wallets = r#"
            {
                "data": {
                    "attributes": { "addresses": [ "uno", "dos" ], "version": "90" },
                    "id": "1",
                    "type": "plain_wallet"
                }
            }"#;
        let get_wallets = || { client.rocket().state::<ServerState>().unwrap().wallets.lock().unwrap() };
        let orig_plain_len = get_wallets().plains.len();
        let response = client.post("/plain_wallets").header(ContentType::JSON).body(wallets).dispatch();
        let after_plain_len = get_wallets().plains.len();
        assert_eq!(response.status(), Status::Ok);
        assert!(after_plain_len > orig_plain_len);
    }

    #[test]
    fn updates_plain_wallet() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let wallets = r#"
            {
                "data": {
                        "attributes": { "addresses": [ "uno", "dos" ], "version": "90" },
                        "id": "1",
                        "type": "plain_wallet"
                    }
            }"#;

        client.post("/plain_wallets").header(ContentType::JSON).body(wallets).dispatch();

        let wallets_to_update = r#"
            {
                "data": {
                        "attributes": { "addresses": [ "tres" ], "version": "92" },
                        "id": "1",
                        "type": "plain_wallet"
                    }
            }"#;

        let response = client.put("/plain_wallets/1").header(ContentType::JSON).body(wallets_to_update).dispatch();

        let get_wallets = || { client.rocket().state::<ServerState>().unwrap().wallets.lock().unwrap() };
        let plain_wallets = &get_wallets().plains;
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(plain_wallets.first().unwrap().addresses.len(), 1);
        assert_eq!(plain_wallets.first().unwrap().addresses.first().unwrap(), "tres");
    }

    #[test]
    fn destroy_plain_wallet() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let wallets = r#"
            {
                "data": {
                        "attributes": { "addresses": [ "uno", "dos" ], "version": "90" },
                        "id": "1",
                        "type": "plain_wallet"
                    }
            }"#;

        client.post("/plain_wallets").header(ContentType::JSON).body(wallets).dispatch();

        let get_wallets = || { client.rocket().state::<ServerState>().unwrap().wallets.lock().unwrap() };

        let response = client.delete("/plain_wallets/1").header(ContentType::JSON).dispatch();
        let plain_wallets = &get_wallets().plains;

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(plain_wallets.len(), 0);
    }
}
