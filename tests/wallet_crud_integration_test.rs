#![feature(plugin)]
#![plugin(rocket_codegen)]
#[macro_use]
extern crate pretty_assertions;

extern crate custodian_server;
extern crate rocket;

#[cfg(test)]
mod wallet_test {
    use custodian_server::models::wallets::Wallets;
    use rocket;
    use rocket::http::ContentType;
    use rocket::http::Status;
    use rocket::local::Client;
    use rocket::local::LocalResponse;
    use std::fs::File;
    use std::sync::MutexGuard;
    //use custodian_server::handlers::addresses;
    use custodian_server::handlers::wallets;
    use custodian_server::server_state::ServerState;

    fn rocket() -> rocket::Rocket {
        let f = File::create("/dev/null").unwrap();
        let state: ServerState =
            ServerState::new("./tests/btc-testnet.cfg", &f, &f).expect("Error creating State");

        rocket::ignite().manage(state).mount(
            "/",
            routes![
                wallets::plain::index,
                wallets::plain::show,
                wallets::plain::create,
                wallets::plain::update,
                wallets::plain::destroy,
                wallets::hd::index,
                wallets::hd::show,
                wallets::hd::create,
                wallets::hd::update,
                wallets::hd::destroy,
                wallets::multisig::index,
                wallets::multisig::show,
                wallets::multisig::create,
                wallets::multisig::update,
                wallets::multisig::destroy,
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
        post(
            &client,
            "/plain_wallets",
            r#"{ "data": {
            "attributes": { "version": "90" },
            "type": "plain_wallet",
            "id": "1"
          }
        }"#,
        );
        client
    }

    fn post(client: &Client, url: &str, body: &str) {
        let response = client
            .post(url)
            .header(ContentType::JSON)
            .body(body)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    fn get<'a>(client: &'a Client, url: &'a str) -> LocalResponse<'a> {
        let response = client.get(url).header(ContentType::JSON).dispatch();
        assert_eq!(response.status(), Status::Ok);
        response
    }

    fn count_wallets(wallets: &Wallets) -> usize {
        wallets.plains.len() + wallets.hds.len() + wallets.multisigs.len()
    }

    // Adds 1 wallet of each type
    // Shows the plain_wallet
    // Creates another plain_wallet
    // Listing all wallets shows 2 items.
    // Updates the first plain wallet version.
    // -- Adds addresses to the first plain wallet
    // -- removes addresses from the first plain wallet
    // Showing the first plain wallet sees the change.
    // Destroys the first plain wallet
    // Lists all wallets again, only the second plain wallet exists.
    #[test]
    fn goes_through_the_full_wallet_lifecycle() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        assert_eq!(count_wallets(&get_wallets(&client)), 0);

        post(
            &client,
            "/plain_wallets",
            r#"{ "data": {
            "attributes": { "version": "90" },
            "type": "plain_wallet"
          }
        }"#,
        );

        post(
            &client,
            "/hd_wallets",
            r#"{ "data": {
            "attributes": { "version": "90", "xpub": "xpub2323323232" },
            "type": "hd_wallet"
        }}"#,
        );

        post(
            &client,
            "/multisig_wallets",
            r#"{ "data": {
            "type": "multisig_wallet",
            "attributes": {
                "version": "90",
                "xpubs": ["xpub2323323232", "xpub12121212", "xpub12121221"],
                "signers": 2
            }
        }}"#,
        );

        assert_eq!(count_wallets(&get_wallets(&client)), 3);

        assert_eq!(
            get(&client, "/plain_wallets").body_string().unwrap(),
            r#"{"data":[{"attributes":{"version":"90"},"id":"1","type":"plain_wallet"}]}"#
        );

        assert_eq!(
            get(&client, "/plain_wallets/1").body_string().unwrap(),
            r#"{"data":{"attributes":{"version":"90"},"id":"1","type":"plain_wallet"}}"#
        );
    }

    #[test]
    fn goes_through_a_hd_wallet_lifecycle() {}

    #[test]
    fn goes_through_a_multisig_wallet_lifecycle() {}

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
                        "attributes": { "version": "92" },
                        "id": "1",
                        "type": "plain_wallet"
                    }
            }"#;

        let response = client
            .put("/plain_wallets/1")
            .header(ContentType::JSON)
            .body(wallets_to_update)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);

        let after_plain_wallets = &get_wallets(&client).plains;

        //TODO: Update wallets by adding addresses
        //let addresses = &after_plain_wallets.first().unwrap().addresses;

        //assert_eq!(addresses.len(), 1);
        //assert_eq!(addresses.first().unwrap().clone().id.unwrap(), "tres");

        assert_eq!(after_plain_wallets.first().unwrap().version, "92");
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
        /*
        assert_eq!(plain_wallets.first().unwrap().addresses.len(), 3);
        assert_eq!(
            plain_wallets.first().unwrap().addresses,
            vec!["uno", "dos", "tres"]
        );
        */
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
        /*
        assert_eq!(plain_wallets.first().unwrap().addresses.len(), 1);
        assert_eq!(
            plain_wallets.first().unwrap().addresses,
            vec!["uno"]
        );
        */
    }
}
