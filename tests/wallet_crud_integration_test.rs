#![feature(plugin)]
#![plugin(rocket_codegen)]
#[macro_use]
extern crate pretty_assertions;

extern crate custodian_server;
extern crate rocket;

extern crate serde_json;

extern crate jsonapi;
extern crate tiny_ram_db;

#[cfg(test)]
mod wallet_test {

    #[cfg(feature = "btc")]
    const CURRENCY: &str = "btc";

    #[cfg(feature = "bch")]
    const CURRENCY: &str = "bch";

    #[cfg(feature = "ltc")]
    const CURRENCY: &str = "ltc";

    use custodian_server::handlers::addresses;
    use custodian_server::handlers::blocks;
    use custodian_server::handlers::transactions;
    use custodian_server::handlers::wallets;
    use custodian_server::models::wallets::Wallets;
    use custodian_server::server_state::ServerState;
    use rocket;
    use rocket::http::ContentType;
    use rocket::http::Status;
    use rocket::local::Client;
    use rocket::local::LocalResponse;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::Read;
    use std::sync::MutexGuard;

    use serde_json::{Error, Value};

    fn rocket() -> rocket::Rocket {
        let f = File::create("/dev/null").unwrap();

        let state: ServerState =
            ServerState::new(&format!("./tests/{}-testnet.cfg", CURRENCY), &f, &f)
                .expect("Error creating State");

        rocket::ignite().manage(state).mount(
            "/",
            routes![
                wallets::plain::index,
                wallets::plain::show,
                wallets::plain::create,
                wallets::plain::update,
                wallets::plain::destroy,
                wallets::plain::get_utxos,
                wallets::plain::get_incoming,
                wallets::hd::index,
                wallets::hd::show,
                wallets::hd::create,
                wallets::hd::update,
                wallets::hd::destroy,
                wallets::hd::get_utxos,
                wallets::hd::get_incoming,
                wallets::multisig::index,
                wallets::multisig::show,
                wallets::multisig::create,
                wallets::multisig::update,
                wallets::multisig::destroy,
                wallets::multisig::get_utxos,
                wallets::multisig::get_incoming,
                addresses::plain::index,
                addresses::plain::create,
                addresses::plain::destroy,
                addresses::plain::balance,
                addresses::hd::index,
                addresses::hd::create,
                addresses::hd::destroy,
                addresses::hd::balance,
                addresses::multisig::index,
                addresses::multisig::create,
                addresses::multisig::destroy,
                addresses::multisig::balance,
                blocks::base::last,
                transactions::base::broadcast
            ],
        )
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

    fn put(client: &Client, url: &str, body: &str) {
        let response = client
            .put(url)
            .header(ContentType::JSON)
            .body(body)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    fn delete(client: &Client, url: &str, body: &str) {
        let response = client
            .delete(url)
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

    fn load_fixture_file(path: &str) -> String {
        let mut file = File::open(path).expect("file not found");
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader
            .read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        contents.replace("\n", "").replace(" ", "")
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
    // Add address mhjp3ZgbGxx5qc9Y8dvk1F71QeQcE9swLE to a plain wallet.
    // Get utxos for address mhjp3ZgbGxx5qc9Y8dvk1F71QeQcE9swLE in plain wallet.
    // Create a MultisigWallet.
    // Get Multisig Utxos for multisig wallet.
    // Get Multisig incoming transactions for multisig wallet.
    #[test]
    fn goes_through_the_full_wallet_lifecycle() {
        let client = Client::new(rocket()).expect("valid rocket instance");

        post(
            &client,
            "/plain_wallets",
            r#"{ 
                "data": {
                    "attributes": { 
                        "wallet": {
                            "version": "90",
                            "label": "my plain wallet"
                        }
                    },
                    "type": "plain_wallet"
                }
            }"#,
        );

        post(
            &client,
            "/hd_wallets",
            r#"{ 
                "data": {
                    "attributes": { 
                        "wallet": {
                            "version": "90",
                            "label": "my hd wallet",
                            "xpub": "xpub2323323232"
                        }
                    },
                    "type": "hd_wallet"
                }
            }"#,
        );

        post(
            &client,
            "/multisig_wallets",
            r#"{
                "data": {
                    "attributes": {
                        "wallet": {
                            "version": "90",
                            "label": "my multisig wallet",
                            "xpubs": ["xpub2323323232", "xpub12121212", "xpub12121221"],
                            "signers": 2
                        }
                    },
                    "type": "multisig_wallet"
                }
            }"#,
        );

        assert_eq!(
            get(&client, "/plain_wallets/0").body_string().unwrap(),
            r#"{"data":{"attributes":{"wallet":{"label":"my plain wallet","version":"90"}},"id":"0","type":"plain_wallet"}}"#,
        );

        post(
            &client,
            "/plain_wallets",
            r#"{
                "data": {
                    "attributes": { 
                        "wallet": {
                            "version": "54",
                            "label": "my second wallet"
                        }
                    },
                    "type": "plain_wallet"
                }
            }"#,
        );

        assert_eq!(
            get(&client, "/plain_wallets").body_string().unwrap(),
            r#"{"data":[{"attributes":{"wallet":{"label":"my plain wallet","version":"90"}},"id":"0","type":"plain_wallet"},{"attributes":{"wallet":{"label":"my second wallet","version":"54"}},"id":"1","type":"plain_wallet"}]}"#
        );

        put(
            &client,
            "/plain_wallets/0",
            r#"{
                "data": {
                    "attributes": { 
                        "wallet": {
                            "version": "91",
                            "label": "my plain wallet updated"
                        }
                    },
                    "type": "plain_wallet"
                }
            }"#,
        );

        assert_eq!(
            get(&client, "/plain_wallets/0").body_string().unwrap(),
            r#"{"data":{"attributes":{"wallet":{"label":"my plain wallet updated","version":"91"}},"id":"0","type":"plain_wallet"}}"#,
        );

        post(
            &client,
            "/plain_addresses",
            r#"{ "data": {
            "attributes": { },
            "id": "lk1jh314",
            "type": "address"
          }}"#,
        );

        // assert_eq!(
        //     get(&client, "/plain_wallets/1/relationships/addresses")
        //         .body_string()
        //         .unwrap(),
        //     r#"{"data":[{"attributes":{},"id":"lk1jh314","type":"address"}]}"#
        // );

        // delete(
        //     &client,
        //     "/plain_wallets/1/relationships/addresses",
        //     r#"{ "data": {
        //     "attributes": { },
        //     "id": "lk1jh314",
        //     "type": "address"
        //   }}"#,
        // );

        // assert_eq!(
        //     get(&client, "/plain_wallets/1/relationships/addresses")
        //         .body_string()
        //         .unwrap(),
        //     r#"{"data":[]}"#
        // );

        // assert_eq!(
        //     get(&client, "/plain_wallets/1").body_string().unwrap(),
        //     r#"{"data":{"attributes":{"version":"91"},"id":"1","type":"plain_wallet"}}"#
        // );

        delete(&client, "/plain_wallets/0", "");

        // let response = client
        //     .get("/plain_wallets/0")
        //     .header(ContentType::JSON)
        //     .dispatch();
        // assert_eq!(response.status(), Status::NotFound);

        assert_eq!(
            get(&client, "/plain_wallets").body_string().unwrap(),
            r#"{"data":[{"attributes":{"wallet":{"label":"my second wallet","version":"54"}},"id":"1","type":"plain_wallet"}]}"#
        );

        // post(
        //     &client,
        //     "/plain_wallets/2/relationships/addresses",
        //     r#"{ "data": {
        //     "attributes": { },
        //     "id": "mhjp3ZgbGxx5qc9Y8dvk1F71QeQcE9swLE",
        //     "type": "address"
        //   }}"#,
        // );

        assert_eq!(
            get(&client, "/plain_wallets/2/get_utxos?since=0&limit=400")
                .body_string()
                .unwrap(),
            load_fixture_file("./tests/data/plain_utxos.json")
        );

        post(
            &client,
            "/multisig_wallets",
            r#"{ "data": {
            "type": "multisig_wallet",
            "attributes": {
                "version": "90",
                "xpubs": ["xpub661MyMwAqRbcGCmcnz4JtnieVyuvgQFGqZqw3KS1g9khndpF3segkAYbYCKKaQ9Di2ZuWLaZU4Axt7TrKq41aVYx8XTbDbQFzhhDMntKLU5",
                          "xpub661MyMwAqRbcFwc3Nmz8WmMU9okGmeVSmuprwNHCVsfhy6vMyg6g79octqwNftK4g62TMWmb7UtVpnAWnANzqwtKrCDFe2UaDCv1HoErssE",
                          "xpub661MyMwAqRbcGkqPSKVkwTMtFZzEpbWXjM4t1Dv1XQbfMxtyLRGupWkp3fcSCDtp6nd1AUrRtq8tnFGTYgkY1pB9muwzaBDnJSMo2rVENhz"],
                "signers": 2
            }
        }}"#,
        );

        post(
            &client,
            "/hd_wallets/1/relationships/addresses",
            r#"{ "data": {
            "attributes": { "address": "2NAHscN6XVqUPzBSJHC3fhkeF5SQVxiR9p9", "path": []},
            "type": "hd_address"
          }}"#,
        );

        get(&client, "/hd_wallets/1/get_utxos?since=0&limit=600");

        assert_eq!(
            get(&client, "/hd_wallets/relationships/addresses/2NAHscN6XVqUPzBSJHC3fhkeF5SQVxiR9p9/balance?since=0&limit=600")
                .body_string()
                .unwrap(),
            "1309846".to_string()
        );

        post(
            &client,
            "/multisig_wallets/2/relationships/addresses",
            r#"{ "data": {
            "attributes": { "address": "2NAHscN6XVqUPzBSJHC3fhkeF5SQVxiR9p9", "path": [42, 1, 1]},
            "type": "hd_address"
          }}"#,
        );

        assert_eq!(
            get(&client, "/multisig_wallets/2/get_utxos?since=0&limit=400")
                .body_string()
                .unwrap(),
            load_fixture_file("./tests/data/multisig_utxos.json")
        );

        assert_eq!(
            get(&client, "/multisig_wallets/2/get_incoming?since=400")
                .body_string()
                .unwrap(),
            load_fixture_file("./tests/data/multisig_incoming_transactions.json")
        );

        let v: Value =
            ::serde_json::from_str(&get(&client, "/blocks/last").body_string().unwrap()).unwrap();
        assert_eq!(
            v["data"]["attributes"]["height"].as_u64().unwrap() > 400,
            true
        );

        post(&client, "/transactions/broadcast", r#"01000000017b1eabe0209b1fe794124575ef807057c77ada2138ae4fa8d6c4de0398a14f3f00000000494830450221008949f0cb400094ad2b5eb399d59d01c14d73d8fe6e96df1a7150deb388ab8935022079656090d7f6bac4c9a94e0aad311a4268e082a725f8aeae0573fb12ff866a5f01ffffffff01f0ca052a010000001976a914cbc20a7664f2f69e5355aa427045bc15e7c6c77288ac00000000"#);
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
        let response = client
            .post("/plain_wallets")
            .header(ContentType::JSON)
            .body(wallets)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
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

        //TODO: Update wallets by adding addresses
        //let addresses = &after_plain_wallets.first().unwrap().addresses;

        //assert_eq!(addresses.len(), 1);
        //assert_eq!(addresses.first().unwrap().clone().id.unwrap(), "tres");
    }

    #[test]
    fn destroy_plain_wallet() {
        let client = creates_wallet_for_other_tests();

        let response = client
            .delete("/plain_wallets/1")
            .header(ContentType::JSON)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn add_address() {
        let client = creates_wallet_for_other_tests();

        let response = client
            .post("/plain_wallets/1/addresses")
            .header(ContentType::JSON)
            .body("tres")
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn destroy_address() {
        let client = creates_wallet_for_other_tests();

        let response = client
            .delete("/plain_wallets/1/addresses")
            .header(ContentType::JSON)
            .body("dos")
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn serialize_plain_wallet() {
        use custodian_server::models::plain_wallet::PlainWallet;
        use jsonapi::model::JsonApiModel;
        use custodian_server::models::resource_wallet::ResourceWallet;

        let plain_wallet: ResourceWallet<PlainWallet> = ResourceWallet {
            id: None,
            wallet: PlainWallet {
                version: "57".to_string(),
                label: "default".to_string()
            }
        };

        assert_eq!(serde_json::to_string(&plain_wallet.to_jsonapi_document()).unwrap(),
                   "{\"data\":{\"type\":\"plain_wallet\",\"id\":null,\"attributes\":{\"data\":{\"label\":\"default\",\"version\":\"57\"}}}}");

        assert_eq!(serde_json::to_string(&plain_wallet).unwrap(), "{\"id\":null,\"data\":{\"version\":\"57\",\"label\":\"default\"}}");
    }
}
