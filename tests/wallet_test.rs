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
    use custodian_server::handlers::wallets;
    use rocket::http::ContentType;

    fn rocket() -> rocket::Rocket {
        let f = File::create("/dev/null").unwrap();
        let state: ServerState = ServerState::new("./tests/btc-testnet.cfg", &f, &f).expect("Error creating State");

        rocket::ignite().manage(state).mount("/", routes![wallets::index, wallets::create])
    }

    #[test]
    fn get_wallets_empty_data() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/wallets").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("{\"data\":{\"attributes\":{\"hd\":[],\"multisig\":[],\"plain\":[]},\"id\":\"\",\"type\":\"wallets\"}}".into()));
    }

    #[test]
    fn creates_plain_and_hd_wallet() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let wallets = r#"
            {
                "data": {
                    "attributes": {},
                    "id": "",
                    "relationships": {
                        "hd": { "data": [ { "id": "12", "type": "hd_wallet" } ] },
                        "multisig": { "data": [] },
                        "plain": { "data": [ { "id": "1", "type": "plain_wallet" } ] }
                    },
                    "type": "wallets"
                },
                "included": [
                    {
                        "attributes": { "addresses": [ "uno", "dos" ], "version": "90" },
                        "id": "1",
                        "type": "plain_wallet"
                    },
                    {
                        "attributes": { "addresses": [], "version": "2", "xpub": "123" },
                        "id": "12",
                        "type": "hd_wallet"
                    }
                ]
            }"#;
        let get_wallets = || { client.rocket().state::<ServerState>().unwrap().wallets.lock().unwrap() };
        let orig_plain_len = get_wallets().plain.len();
        let orig_hd_len = get_wallets().hd.len();
        let response = client.post("/wallets").header(ContentType::JSON).body(wallets).dispatch();
        let after_plain_len = get_wallets().plain.len();
        let after_hd_len = get_wallets().hd.len();
        assert_eq!(response.status(), Status::Ok);
        assert!(after_plain_len > orig_plain_len);
        assert!(after_hd_len > orig_hd_len);
    }
}
