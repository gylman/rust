use bincode::{deserialize, serialize};
use rocksdb::{Options, DB};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct Tx {
    block_height: String,
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
    // let my_tx: Vec<Tx> = vec![Tx, Tx, Tx, Tx];

    let my_txs: Vec<Tx> = vec![
        Tx {
            block_height: String::from("300523"),
            nonce: String::from("0x0"),
            gas_price: String::from("0x09184e72a000"),
            gas_limit: String::from("0x30000"),
            to: String::from("0xRecipientAddress"),
            value: String::from("0x12345"),
            data: String::from("0x"),
            v: String::from("0x1c"),
            r: String::from("0x7c1ec827d6a215e24f..."),
            s: String::from("0x77fb11b3a54b7f3d10f..."),
        },
        Tx {
            block_height: String::from("311523"),
            nonce: String::from("0x0"),
            gas_price: String::from("0x09184e72a000"),
            gas_limit: String::from("0x30000"),
            to: String::from("0xRecipientAddress"),
            value: String::from("0x12345"),
            data: String::from("0x"),
            v: String::from("0x1c"),
            r: String::from("0x7c1ec827d6a215e24f..."),
            s: String::from("0x77fb11b3a54b7f3d10f..."),
        },
        Tx {
            block_height: String::from("333523"),
            nonce: String::from("0x0"),
            gas_price: String::from("0x09184e72a000"),
            gas_limit: String::from("0x30000"),
            to: String::from("0xRecipientAddress"),
            value: String::from("0x12345"),
            data: String::from("0x"),
            v: String::from("0x1c"),
            r: String::from("0x7c1ec827d6a215e24f..."),
            s: String::from("0x77fb11b3a54b7f3d10f..."),
        },
        Tx {
            block_height: String::from("324333"),
            nonce: String::from("0x0"),
            gas_price: String::from("0x09184e72a000"),
            gas_limit: String::from("0x30000"),
            to: String::from("0xRecipientAddress"),
            value: String::from("0x12345"),
            data: String::from("0x"),
            v: String::from("0x1c"),
            r: String::from("0x7c1ec827d6a215e24f..."),
            s: String::from("0x77fb11b3a54b7f3d10f..."),
        },
    ];

    let mut i = 0;
    loop {
        if i == my_txs.len() {
            break;
        }
        let serialized_tx = serialize(&my_txs[i]).expect("Serialization failed");
        db.put(my_txs[i].block_height.as_bytes(), &serialized_tx)
            .expect("Failed to put tx into RocksDB");

        i += 1;
    }

    i = 0;

    loop {
        if i == my_txs.len() {
            break;
        }
        let stored_tx = db
            .get(my_txs[i].block_height.as_bytes())
            .expect("Failed to get tx from RocksDB");
        if let Some(tx) = stored_tx {
            let deserialized_tx: Tx = deserialize(&tx).expect("Deserialization failed");
            println!("Deserialized tx: {:?}", deserialized_tx);
        }
        i += 1;
    }
}
