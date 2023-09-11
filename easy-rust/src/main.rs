use bincode::{deserialize, serialize};
use rocksdb::{Options, DB};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct Tx {
    nonce: String,
    gas_price: String,
    gas_limit: String,
    to: String,
    value: String,
    data: String,
    v: String,
    r: String,
    s: String,
}

fn main() {
    // Open or create a RocksDB database.
    let path = Path::new("my_db");
    let db = DB::open_default(&path).expect("Failed to open database");

    // Define a sample tx.
    // let my_tx: Vec<u32> = vec![1, 2, 3, 4, 5];
    let my_tx = Tx {
        nonce: String::from("0x0"),
        gas_price: String::from("0x09184e72a000"),
        gas_limit: String::from("0x30000"),
        to: String::from("0xRecipientAddress"),
        value: String::from("0x12345"),
        data: String::from("0x"),
        v: String::from("0x1c"),
        r: String::from("0x7c1ec827d6a215e24f..."),
        s: String::from("0x77fb11b3a54b7f3d10f..."),
    };

    // Serialize the tx to bytes.
    let serialized_tx = serialize(&my_tx).expect("Serialization failed");

    // Store the serialized tx in RocksDB.
    db.put(b"my_key", &serialized_tx)
        .expect("Failed to put tx into RocksDB");

    // Retrieve the tx from RocksDB.
    let stored_tx = db.get(b"my_key").expect("Failed to get tx from RocksDB");

    // Deserialize the tx back into a tx.
    if let Some(tx) = stored_tx {
        let deserialized_tx: Tx = deserialize(&tx).expect("Deserialization failed");
        println!("Deserialized tx: {:?}", deserialized_tx);
    }
}
