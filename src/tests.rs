use rocket::local::Client;
use rocket::http::Status;

#[test]
fn builds_transaction_for_emptying_incoming(){
	// Request:
	// 	- A wallet id (will return 404 if not found)
	// 	- The quantity of utxos to include. Defaults to 100.

	// Result will contain:
	//	- A reference to the utxo lock.
	//  - A trezor signable list of inputs/multisig inputs.
	// 	- A trezor compatible list of transactions
	//  - The amount to use for the output, minus the fee.
	//  - The calculated fee.
}

fn queries_confirmed_deposits_for_crediting(){
	// Request:
	// 	- A wallet id
}

// Address indexing
// How to avoid sending all addresses each time?
// Create Wallets that have a unique ID and contain addresses.
// 		- HD wallet is identified by a string, has a version: a an xpub and a derivation path.
//	  - Regular wallet has a fingerprint that hashes all its addresses.
//
// All methods receive a wallet identifier, if the wallet is not found a 404 is added.
// Wallets have a crud, where creating and updating one will increase the version.
// When a new wallet is created or updated a recaching for all its aspects will be warmed up.

// * UTXO locking / multithreading
// show active utxo lock
// removes an utxo lock
// lists all utxo locks
fn unlocks_utxos(){
}

/*
fn register_hit(client: &Client) {
    let response = client.get("/").dispatch();;
    assert_eq!(response.status(), Status::Ok);
}

fn get_count(client: &Client) -> usize {
    let mut response = client.get("/count").dispatch();
    response.body_string().and_then(|s| s.parse().ok()).unwrap()
}

#[test]
fn test_count() {
    let client = Client::new(super::rocket()).unwrap();

    // Count should start at 0.
    assert_eq!(get_count(&client), 0);

    for _ in 0..99 { register_hit(&client); }
    assert_eq!(get_count(&client), 99);

    register_hit(&client);
    assert_eq!(get_count(&client), 100);
}

// Cargo runs each test in parallel on different threads. We use all of these
// tests below to show (and assert) that state is managed per-Rocket instance.
#[test] fn test_count_parallel() { test_count() }
#[test] fn test_count_parallel_2() { test_count() }
#[test] fn test_count_parallel_3() { test_count() }
#[test] fn test_count_parallel_4() { test_count() }
#[test] fn test_count_parallel_5() { test_count() }
#[test] fn test_count_parallel_6() { test_count() }
#[test] fn test_count_parallel_7() { test_count() }
#[test] fn test_count_parallel_8() { test_count() }
#[test] fn test_count_parallel_9() { test_count() }
*/
