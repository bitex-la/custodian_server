#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate custodian_server;
extern crate rocket;

#[cfg(test)]
mod wallet_test {
    use rocket;
    use rocket::http::ContentType;
    use rocket::http::Status;
    use rocket::local::Client;
    use std::fs::File;
    use std::sync::MutexGuard;
    use custodian_server::handlers::addresses;
    use custodian_server::handlers::plain_wallets;
    use custodian_server::server_state::ServerState;
    use custodian_server::models::wallets::Wallets;

    fn rocket() -> rocket::Rocket {
        let f = File::create("/dev/null").unwrap();
        let state: ServerState =
            ServerState::new("./tests/btc-testnet.cfg", &f, &f).expect("Error creating State");

        rocket::ignite().manage(state).mount(
            "/",
            routes![
                plain_wallets::index,
                plain_wallets::create,
                plain_wallets::update,
                plain_wallets::destroy,
                addresses::create,
                addresses::destroy
            ],
        )
    }

    fn get_wallets(client: &Client) -> MutexGuard<Wallets> {
        client
            .rocket()
            .state::<ServerState>()
            .unwrap()
            .wallets_lock()
    }

    fn creates_wallet_for_other_tests() -> Client {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let wallets = r#"
            {
                "data": {
                        "attributes": { "addresses": [ "uno", "dos" ], "version": "90" },
                        "id": "1",
                        "type": "plain_wallet"
                    }
            }"#;

        client
            .post("/plain_wallets")
            .header(ContentType::JSON)
            .body(wallets)
            .dispatch();

        client
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
        let orig_plain_len = get_wallets(&client).plains.len();
        let response = client
            .post("/plain_wallets")
            .header(ContentType::JSON)
            .body(wallets)
            .dispatch();
        let after_plain_len = get_wallets(&client).plains.len();
        assert_eq!(response.status(), Status::Ok);
        assert!(after_plain_len > orig_plain_len);
    }

    #[test]
    fn internal_error_when_create_wallet() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let wallets = r#"
            {
                "data": {
                    "attributes": { "addresses": [ "uno", "dos", ], "version": "90" },
                    "id": "1",
                    "type": "plain_wallet"
                }
            }"#;
        let response = client
            .post("/plain_wallets")
            .header(ContentType::JSON)
            .body(wallets)
            .dispatch();
        assert_eq!(response.status(), Status::BadRequest);
    }

    #[test]
    fn not_found_wallet() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client
            .get("/plain_wallets/1")
            .header(ContentType::JSON)
            .dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn updates_plain_wallet() {
        let client = creates_wallet_for_other_tests();

        let wallets_to_update = r#"
            {
                "data": {
                        "attributes": { "addresses": [ "tres" ], "version": "92" },
                        "id": "1",
                        "type": "plain_wallet"
                    }
            }"#;

        let response = client
            .put("/plain_wallets/1")
            .header(ContentType::JSON)
            .body(wallets_to_update)
            .dispatch();

        let plain_wallets = &get_wallets(&client).plains;
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(plain_wallets.first().unwrap().addresses.len(), 1);
        assert_eq!(
            plain_wallets.first().unwrap().addresses.first().unwrap(),
            "tres"
        );
    }

    #[test]
    fn destroy_plain_wallet() {
        let client = creates_wallet_for_other_tests();

        let response = client
            .delete("/plain_wallets/1")
            .header(ContentType::JSON)
            .dispatch();
        let plain_wallets = &get_wallets(&client).plains;

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(plain_wallets.len(), 0);
    }

    #[test]
    fn add_address() {
        let client = creates_wallet_for_other_tests();

        let response = client
            .post("/plain_wallets/1/addresses")
            .header(ContentType::JSON)
            .body("tres")
            .dispatch();
        let plain_wallets = &get_wallets(&client).plains;

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(plain_wallets.first().unwrap().addresses.len(), 3);
        assert_eq!(
            plain_wallets.first().unwrap().addresses,
            vec!["uno", "dos", "tres"]
        );
    }

    #[test]
    fn destroy_address() {
        let client = creates_wallet_for_other_tests();

        let response = client
            .delete("/plain_wallets/1/addresses")
            .header(ContentType::JSON)
            .body("dos")
            .dispatch();
        let plain_wallets = &get_wallets(&client).plains;

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(plain_wallets.first().unwrap().addresses.len(), 1);
        assert_eq!(
            plain_wallets.first().unwrap().addresses,
            vec!["uno"]
        );
    }
}
