#![feature(plugin)]
#![plugin(rocket_codegen)]
#[macro_use]
extern crate pretty_assertions;

extern crate custodian_server;
extern crate rocket;

extern crate serde_json;

extern crate jsonapi;
extern crate tiny_ram_db;

extern crate rand;
extern crate secp256k1;
extern crate bitcoin;

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
    use jsonapi::model::*;

    use serde_json;
    use serde_json::{Error, Value};

    #[get("/stop")]
    fn stop(state: &ServerState) -> String {
        state.graceful_stop();
        "Stopping soon.".to_string()
    }

    fn rocket() -> rocket::Rocket {
        let f = File::create("/dev/null").unwrap();

        let state: ServerState =
            ServerState::new(&format!("./tests/{}-testnet.cfg", CURRENCY), &f, &f)
                .expect("Error creating State");

        let routes = 
            routes![
                    transactions::base::broadcast,
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
                    addresses::plain::show,
                    addresses::plain::destroy,
                    addresses::plain::balance,
                    addresses::plain::get_utxos,
                    addresses::hd::index,
                    addresses::hd::create,
                    addresses::hd::show,
                    addresses::hd::destroy,
                    addresses::hd::balance,
                    addresses::hd::get_utxos,
                    addresses::multisig::index,
                    addresses::multisig::create,
                    addresses::multisig::show,
                    addresses::multisig::destroy,
                    addresses::multisig::balance,
                    addresses::multisig::get_utxos,
                    blocks::base::last,
                    stop
                ];
        rocket::ignite().manage(state).mount( "/", routes)
    }

    fn creates_wallet_for_other_tests() -> Client {
        let client = Client::new(rocket()).expect("valid rocket instance");
        post(
            &client,
            "/plain_wallets",
            r#"{ "data": {
            "attributes": { "version": "0" },
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

    fn delete<'a>(client: &'a Client, url: &'a str, body: &'a str) -> LocalResponse<'a> {
        let response = client
            .delete(url)
            .header(ContentType::JSON)
            .body(body)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        response
    }

    fn get<'a>(client: &'a Client, url: &'a str) -> LocalResponse<'a> {
        let response = client.get(url).header(ContentType::JSON).dispatch();
        assert_eq!(response.status(), Status::Ok);
        response
    }

    fn not_found<'a>(client: &'a Client, url: &'a str) {
        let response = client.get(url).header(ContentType::JSON).dispatch();
        let response = client
            .get(url)
            .header(ContentType::JSON)
            .dispatch();
        assert_eq!(response.status(), Status::NotFound);
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
                        "version": "0",
                        "label": "my_plain_wallet"
                    },
                    "type": "plain_wallets"
                }
            }"#,
        );

        post(
            &client,
            "/hd_wallets",
            r#"{ 
                "data": {
                    "attributes": { 
                        "version": "0",
                        "label": "my hd wallet",
                        "xpub": "xpub661MyMwAqRbcGCmcnz4JtnieVyuvgQFGqZqw3KS1g9khndpF3segkAYbYCKKaQ9Di2ZuWLaZU4Axt7TrKq41aVYx8XTbDbQFzhhDMntKLU5"
                    },
                    "type": "hd_wallets"
                }
            }"#,
        );

        post(
            &client,
            "/multisig_wallets",
            r#"{
                "data": {
                    "attributes": {
                        "version": "0",
                        "label": "my multisig wallet",
                        "xpubs": ["xpub2323323232", "xpub12121212", "xpub12121221"],
                        "signers": 2
                    },
                    "type": "multisig_wallets"
                }
            }"#,
        );

        assert_eq!(
            get(&client, "/plain_wallets/my_plain_wallet").body_string().unwrap(),
            r#"{"data":{"attributes":{"label":"my_plain_wallet","version":"0"},"id":"my_plain_wallet","type":"plain_wallets"}}"#,
        );

        post(
            &client,
            "/plain_wallets",
            r#"{
                "data": {
                    "attributes": { 
                        "version": "0",
                        "label": "my_second_wallet"
                    },
                    "type": "plain_wallets"
                }
            }"#,
        );

        assert_eq!(
            get(&client, "/plain_wallets").body_string().unwrap(),
            r#"{"data":[{"attributes":{"label":"my_plain_wallet","version":"0"},"id":"my_plain_wallet","type":"plain_wallets"},{"attributes":{"label":"my_second_wallet","version":"0"},"id":"my_second_wallet","type":"plain_wallets"}]}"#
        );

        put(
            &client,
            "/plain_wallets/my_plain_wallet",
            r#"{
                "data": {
                    "attributes": { 
                        "version": "0",
                        "label": "my plain wallet updated"
                    },
                    "type": "plain_wallets"
                }
            }"#,
        );

        assert_eq!(
            get(&client, "/plain_wallets/my plain wallet updated").body_string().unwrap(),
            r#"{"data":{"attributes":{"label":"my plain wallet updated","version":"0"},"id":"my plain wallet updated","type":"plain_wallets"}}"#,
        );

        post(
            &client,
            "/plain_addresses",
            r#"{
                "data": {
                    "attributes": {
                        "public_address": "mru76ADdwx3EFjuknsZZVRXKUrnWxedwH7"
                    },
                    "relationships": {
                        "wallet": {
                            "data": {
                                "type": "plain_wallets",
                                "id": "my plain wallet updated"
                            }
                        }
                    },
                    "type": "plain_addresses"
                }
            }"#,
        );

        post(
            &client,
            "/plain_addresses",
            r#"{
                "data": {
                    "attributes": {
                        "public_address": "n2ivyMi4jExgCeZTfiBuUt3GQhnnv8AXeb"
                    },
                    "relationships": {
                        "wallet": {
                            "data": {
                                "type": "plain_wallets",
                                "id": "my plain wallet updated"
                            }
                        }
                    },
                    "type": "plain_addresses"
                }
            }"#,
        );

        assert_eq!(
            get(&client, "/plain_addresses/1")
                .body_string()
                .unwrap(),
            r#"{"data":{"attributes":{"public_address":"mru76ADdwx3EFjuknsZZVRXKUrnWxedwH7"},"id":"1","relationships":{"wallet":{"data":{"id":"1","type":"plain_wallets"}}},"type":"plain_addresses"},"included":[{"attributes":{"label":"my plain wallet updated","version":"0"},"id":"1","type":"plain_wallets"}]}"#,
        );

        assert_eq!(
            get(&client, "/plain_addresses")
                .body_string()
                .unwrap(),
            r#"{"data":[{"attributes":{"public_address":"mru76ADdwx3EFjuknsZZVRXKUrnWxedwH7"},"id":"1","relationships":{"wallet":{"data":{"id":"1","type":"plain_wallets"}}},"type":"plain_addresses"},{"attributes":{"public_address":"n2ivyMi4jExgCeZTfiBuUt3GQhnnv8AXeb"},"id":"2","relationships":{"wallet":{"data":{"id":"1","type":"plain_wallets"}}},"type":"plain_addresses"}],"included":[{"attributes":{"label":"my plain wallet updated","version":"0"},"id":"1","type":"plain_wallets"},{"attributes":{"label":"my plain wallet updated","version":"0"},"id":"1","type":"plain_wallets"}]}"#,
        );

        assert_eq!(
            get(&client, "/plain_wallets/my plain wallet updated").body_string().unwrap(),
            r#"{"data":{"attributes":{"label":"my plain wallet updated","version":"2"},"id":"my plain wallet updated","type":"plain_wallets"}}"#,
        );

        assert_eq!(
            get(&client, "/plain_wallets").body_string().unwrap(), 
            r#"{"data":[{"attributes":{"label":"my plain wallet updated","version":"2"},"id":"my plain wallet updated","type":"plain_wallets"},{"attributes":{"label":"my_second_wallet","version":"0"},"id":"my_second_wallet","type":"plain_wallets"}]}"#);

        delete(&client, "/plain_addresses/1", "");
        not_found(&client, "/plain_addresses/1");

        assert_eq!(
            get(&client, "/plain_wallets/my plain wallet updated").body_string().unwrap(),
            r#"{"data":{"attributes":{"label":"my plain wallet updated","version":"1"},"id":"my plain wallet updated","type":"plain_wallets"}}"#,
        );

        post(
            &client,
            "/plain_addresses",
            r#"{
                "data": {
                    "attributes": {
                        "public_address": "mhjp3ZgbGxx5qc9Y8dvk1F71QeQcE9swLE"
                     },
                     "relationships": {
                         "wallet": {
                             "data": {
                                 "type": "plain_wallets",
                                 "id": "my plain wallet updated"
                             }
                         }
                     },
                     "type": "plain_addresses"
                }
            }"#,
        );

        assert_eq!(
            get(&client, "/plain_wallets/my plain wallet updated/get_utxos?since=0&limit=400")
                .body_string()
                .unwrap(),
            load_fixture_file("./tests/data/plain_utxos.json")
        );

        post(
            &client,
            "/multisig_wallets",
            r#"{
                "data": {
                    "attributes": { 
                        "version": "0",
                        "label": "my_second_wallet",
                        "xpubs": ["xpub661MyMwAqRbcGCmcnz4JtnieVyuvgQFGqZqw3KS1g9khndpF3segkAYbYCKKaQ9Di2ZuWLaZU4Axt7TrKq41aVYx8XTbDbQFzhhDMntKLU5",
                                "xpub661MyMwAqRbcFwc3Nmz8WmMU9okGmeVSmuprwNHCVsfhy6vMyg6g79octqwNftK4g62TMWmb7UtVpnAWnANzqwtKrCDFe2UaDCv1HoErssE",
                                "xpub661MyMwAqRbcGkqPSKVkwTMtFZzEpbWXjM4t1Dv1XQbfMxtyLRGupWkp3fcSCDtp6nd1AUrRtq8tnFGTYgkY1pB9muwzaBDnJSMo2rVENhz"],
                        "signers": 2
                    },
                    "type": "multisig_wallets"
                }
            }"#,
        );

        post(
            &client,
            "/hd_addresses",
            r#"{
                "data": {
                    "attributes": {
                        "public_address": "2NAHscN6XVqUPzBSJHC3fhkeF5SQVxiR9p9",
                        "path": []
                    },
                    "relationships": {
                        "wallet": {
                            "data": {
                                "id": "my hd wallet",
                                "type": "hd_wallets"
                            }
                        }
                    },
                    "type": "hd_addresses"
                }
            }"#,
        );

        get(&client, "/hd_wallets/my hd wallet/get_utxos?since=0&limit=600");

        assert_eq!(
            get(&client, "/hd_addresses/2NAHscN6XVqUPzBSJHC3fhkeF5SQVxiR9p9/balance?since=0&limit=600")
                .body_string()
                .unwrap(),
            "1309846".to_string()
        );

        post(
            &client,
            "/multisig_addresses",
            r#"{
                "data": {
                    "attributes": {
                        "public_address": "2NAHscN6XVqUPzBSJHC3fhkeF5SQVxiR9p9",
                        "path": [42, 1, 1]
                     },
                     "relationships": {
                         "wallet": {
                             "data": {
                                 "id": "my_second_wallet",
                                 "type": "multisig_wallets"
                             }
                         }
                     },
                     "type": "multisig_addresses"
                }
            }"#,
        );

        let transactions : JsonApiDocument = serde_json::from_str(&get(&client, "/multisig_wallets/my_second_wallet/get_utxos?since=0&limit=400").body_string().unwrap()).unwrap();
        if let PrimaryData::Multiple(data) = transactions.data.unwrap() {
            let mut transaction_hashes = vec![];
            for resource in data {
                if let serde_json::value::Value::Object(ref value) = resource.attributes["transaction"] {
                    if let serde_json::value::Value::String(ref inner_value) = value["transaction_hash"] {
                        transaction_hashes.push(inner_value.clone());
                    }
                }
            }
            assert_eq!(transaction_hashes,
                vec!["956b30c3c4335f019dbee60c60d76994319473acac356f774c7858cd5c968e40",
                     "1db1f22beb84e5fbe92c8c5e6e7f43d80aa5cfe5d48d83513edd9641fc00d055",
                     "0ded7f014fa3213e9b000bc81b8151bc6f2f926b9afea6e3643c8ad658353c72"]);
        }

        let transactions : JsonApiDocument = serde_json::from_str(&get(&client, "/multisig_wallets/my_second_wallet/get_incoming?since=400").body_string().unwrap()).unwrap();
        if let PrimaryData::Multiple(data) = transactions.data.unwrap() {
            let mut transaction_hashes = vec![];
            for resource in data {
                if let serde_json::value::Value::String(ref value) = resource.attributes["transaction_hash"] {
                    transaction_hashes.push(value.clone());
                }
            }
            assert_eq!(transaction_hashes,
                vec!["81fa58dc058b60f725bd703651c9bcd17b436705a8aa5c29815d0b8e2980e729",
                     "956b30c3c4335f019dbee60c60d76994319473acac356f774c7858cd5c968e40",
                     "558b3e19b720bed80e334d2e5f0fc291f57f5c611263d9634255c10ffc3f4c46",
                     "c9355b79000bea68d9fa078d0e73a5a53bd6d4b46f46993acae283530aa77351",
                     "243f38d7127d5dd23a737932ba6e2f6699f93df44db17082cfa71b542542f852",
                     "85eb1fed8b704d38cbf4bcff3297f2d3c991211a35e9feaa4e12849cffa24455",
                     "1db1f22beb84e5fbe92c8c5e6e7f43d80aa5cfe5d48d83513edd9641fc00d055",
                     "0ded7f014fa3213e9b000bc81b8151bc6f2f926b9afea6e3643c8ad658353c72",
                     "dab5ac2087837e8ac46c9a224f2e86b4816d61d3ac0f8169df0c270d721c8681",
                     "5775ffba2abebeef1c3b63272ef78024937b2a20291b8dd1c6cfc6a511ea28c9",
                     "f257cbb984db0a457c02e3b91df175e9ed1b73c36229583b291680e0400c47da"]);
        }

        let v: Value =
            ::serde_json::from_str(&get(&client, "/blocks/last").body_string().unwrap()).unwrap();
        assert_eq!(
            v["data"]["attributes"]["height"].as_u64().unwrap() > 400,
            true
        );

        delete(&client, "/plain_wallets/my plain wallet updated", "");
        not_found(&client, "/plain_addresses/2");
        not_found(&client, "/plain_addresses/3");

        assert_eq!(
            get(&client, "/plain_wallets").body_string().unwrap(),
            r#"{"data":[{"attributes":{"label":"my_second_wallet","version":"0"},"id":"my_second_wallet","type":"plain_wallets"}]}"#
        );

        post(&client, "/transactions/broadcast", r#"01000000017b1eabe0209b1fe794124575ef807057c77ada2138ae4fa8d6c4de0398a14f3f00000000494830450221008949f0cb400094ad2b5eb399d59d01c14d73d8fe6e96df1a7150deb388ab8935022079656090d7f6bac4c9a94e0aad311a4268e082a725f8aeae0573fb12ff866a5f01ffffffff01f0ca052a010000001976a914cbc20a7664f2f69e5355aa427045bc15e7c6c77288ac00000000"#);
    }

    #[test]
    fn goes_through_a_hd_wallet_lifecycle() {}

    #[test]
    fn goes_through_a_multisig_wallet_lifecycle() {}

    fn generate_addresses() -> Vec<String> {
        use bitcoin::network::constants::Network;
        use bitcoin::util::address::Payload;
        use bitcoin::util::address::Address;
        use secp256k1::Secp256k1;
        use secp256k1::key::PublicKey;
        use rand::thread_rng;

        let network = Network::Testnet;
        let s = Secp256k1::new();
         
        let mut addresses = vec![];

        for i in 0..90000 {
            let (secret_key, public_key) = s.generate_keypair(&mut thread_rng());
         
            let address = Address::p2pk(&public_key, network);

            addresses.push(address.to_string());
        }

        addresses
    }

    fn create_and_add_addresses(addresses: Vec<String>) {
        use std::fs::File;
        use std::io::prelude::*;

        let mut file = File::create("./tests/data/addresses2.txt").unwrap();
        for address in &addresses {
            file.write_all(format!("\"{}\",", address).as_bytes()).unwrap();
        }
    }

    #[test]
    fn load_addresses() {
        use std::time::Instant;

        let client = Client::new(rocket()).expect("valid rocket instance");

        post(
            &client,
            "/plain_wallets",
            r#"{ 
                "data": {
                    "attributes": { 
                        "version": "0",
                        "label": "my plain wallet"
                    },
                    "type": "plain_wallets"
                }
            }"#,
        );

        let mut addresses_with_balances = vec![];
        let mut contents = String::new();
        BufReader::new(File::open("./tests/data/addresses.txt").unwrap()).read_to_string(&mut contents).unwrap();
        let mut addresses: Vec<String> = serde_json::from_str(&contents).unwrap(); //100837 addresses

        let adding_addresses = Instant::now();
        for address in &addresses {
            post(
                &client,
                "/plain_addresses",
                &format!(r#"{{
                    "data": {{
                        "attributes": {{
                            "public_address": "{}"
                        }},
                        "relationships": {{
                            "wallet": {{
                                "data": {{
                                    "type": "plain_wallets",
                                    "id": "1"
                                }}
                            }}
                        }},
                        "type": "plain_addresses"
                    }}
                }}"#, &address),
            );
        }
        let finish_adding_addresses = adding_addresses.elapsed();

        let consulting_balances = Instant::now();
        for address in &addresses {
            let url = &format!("/hd_addresses/{}/balance?since=0&limit=1000000", &address);
            let balance: u64 = get(&client, url).body_string().unwrap().parse().unwrap();
            if balance > 0 {
                addresses_with_balances.push(format!("{}: {}", &address, balance));
            }
        }
        let finish_consulting_balances = consulting_balances.elapsed();

        let get_utxos = Instant::now();
        let response = get(&client, "/plain_wallets/1/get_utxos?since=0&limit=1000000").body_string().unwrap();
        let finish_get_utxos = get_utxos.elapsed();

        println!("Finish adding 100837 addresses {:?}", finish_adding_addresses);
        println!("Finish consulting balances for 100837 addresses {:?}", finish_consulting_balances);
        println!("Finish consulting utxos for a wallet of 100837 addresses {:?}", finish_get_utxos);
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
                    "attributes": { "addresses": [ "uno", "dos" ], "version": "0" },
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
                    "attributes": { "addresses": [ "uno", "dos", ], "version": "0" },
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
    fn updates_plain_wallet() {
        let client = creates_wallet_for_other_tests();

        let wallets_to_update = r#"
            {
                "data": {
                        "attributes": { "version": "0" },
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

}
