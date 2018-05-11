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
            {"data": 
                {"type": "wallets", "id": "", "attributes": 
                    {"plain": [{"id": "1", "version": "90", "addresses": ["uno", "dos"]}], 
                     "hd": [{"id": "", "version": "2", "addresses": [], "xpub": "123"}], 
                     "multisig": [] }}}"#;
        let response = client.post("/wallets").header(ContentType::JSON).body(wallets).dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
