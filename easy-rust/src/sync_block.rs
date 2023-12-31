// let a: Vec<_> = txs.encrypted_pool.values().cloned().collect();
// println!("self.txs.get_mut: {:?}", a);

use std::collections::HashMap;
use std::path::Path;
use std::str::Bytes;
use std::time::Duration;
use std::{env, str, thread, time};

use base64::engine::general_purpose;
use base64::Engine as _;
use bincode::{deserialize, serialize};
use dotenv::dotenv;
use hyper::header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use hyper::{Body, Client, Request};
use rocksdb::DB;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio;
use tokio::runtime::Runtime;

fn encode_data_to_base64(original: &str) -> String {
    // Convert string to bytes
    let bytes = original.as_bytes();
    // Convert bytes to base64
    let base64_str: String = general_purpose::STANDARD.encode(&bytes);
    base64_str
}

pub fn submit_block_to_db(block_height: u64, txs: Vec<u8>) {
    println!("submit_block_to_db: key: {:?}", block_height);

    // Open or create a RocksDB database.
    let path = Path::new("epool");
    let db = DB::open_default(&path).expect("Failed to open database");
    db.put(block_height.to_be_bytes(), txs).expect("Failed to put tx into RocksDB");
    db.put("pre_sync".as_bytes(), serialize(&block_height).expect("Failed to serialize"))
        .expect("Failed to put tx into RocksDB");
}

pub fn submit_to_db(key: &[u8], value: Vec<u8>) {
    println!("submit_to_db: key: {:?} value: {:?}", key, value);
    // Open or create a RocksDB database.
    let path = Path::new("epool");
    let db = DB::open_default(&path).expect("Failed to open database");
    db.put(key, value).expect("Failed to put tx into RocksDB");
}

pub fn retrieve_from_db(key: &[u8]) -> Vec<u8> {
    let path = Path::new("epool");
    let db = DB::open_default(&path).expect("Failed to open database");
    let value = db.get(key).expect("msg");
    let result = match value {
        Some(val) => val,
        None => Vec::new(), // Provide a default value (empty vector) for the None arm
    };
    result
}

async fn submit_to_da(data: &str) -> String {
    dotenv().ok();
    let da_host = env::var("DA_HOST").expect("DA_HOST must be set");
    let da_namespace = env::var("DA_NAMESPACE").expect("DA_NAMESPACE must be set");
    let da_auth_token = env::var("DA_AUTH_TOKEN").expect("DA_AUTH_TOKEN must be set");
    let da_auth = format!("Bearer {}", da_auth_token);

    let client = Client::new();
    let rpc_request = json!({
        "jsonrpc": "2.0",
        "method": "blob.Submit",
        "params": [
            [
                {
                    "namespace": da_namespace,
                    "data": data,
                }
            ]
        ],
        "id": 1,
    });

    let uri = std::env::var("da_uri").unwrap_or(da_host.into());

    let req = Request::post(uri.as_str())
        .header(AUTHORIZATION, HeaderValue::from_str(da_auth.as_str()).unwrap())
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .body(Body::from(rpc_request.to_string()))
        .unwrap();
    let response_future = client.request(req);

    let resp = tokio::time::timeout(Duration::from_secs(100), response_future)
        .await
        .map_err(|_| "Request timed out")
        .unwrap()
        .unwrap();

    let response_body = hyper::body::to_bytes(resp.into_body()).await.unwrap();
    let parsed: Value = serde_json::from_slice(&response_body).unwrap();

    // println!("stompesi - {:?}", parsed);

    if let Some(result_value) = parsed.get("result") { result_value.to_string() } else { "".to_string() }
}

// From_ne_bytes. In Rust we have standard library functions to convert from bytes to integers and
// back again. The from_le_bytes, from_ne_bytes and from_be_bytes functions can be used. To convert
// from an integer back into an array of bytes, we can use functions like to_ne_bytes. The correct
// Endianness must be selected.

pub fn sync_with_da() {
    // Open or create a RocksDB database.
    println!("sync_with_da started");
    // let path = Path::new("epool");
    // println!("open db");
    // let db = DB::open_default(&path).expect("Failed to open database");

    let three_seconds = time::Duration::from_millis(3000);

    // let mut i: u64 = 0;
    // println!("submit sync into db");
    // submit_to_db("sync".as_bytes(), serialize(&i).expect("Failed to serialize"));
    let mut pre_sync: u64;
    let mut txs;
    let mut pre_sync_bin: Vec<u8>;

    let mut block_height = "".to_string();
    // Create the runtime
    let rt = Runtime::new().unwrap();

    loop {
        println!("sleep 3 seconds");
        thread::sleep(three_seconds);
        pre_sync_bin = retrieve_from_db("pre_sync".as_bytes());
        println!("this is pre_sync_bin outside if: {:?}", pre_sync_bin);
        if !pre_sync_bin.is_empty() {
            println!("this is pre_sync_bin inside if: {:?}", pre_sync_bin);
            pre_sync = deserialize(&pre_sync_bin).expect("Failed to deserialize");
            println!("this is pre_sync inside if: {:?}", pre_sync);

            txs = retrieve_from_db(&serialize(&pre_sync).expect("Failed to serialize"));
            if !txs.is_empty() {
                println!("if !txs.is_empty");
                let s = match str::from_utf8(&txs) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };
                rt.block_on(async {
                    block_height = submit_to_da(&encode_data_to_base64(s)).await;
                    println!("this is the block height from DA: {}", block_height);
                });
                println!("try to submit block no. {:?}", pre_sync);
                if !(block_height.len() == 0) {
                    submit_to_db("sync".as_bytes(), serialize(&pre_sync).expect("Failed to serialize"));
                    println!("last synced block is updated to {:?}", pre_sync);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 4; //= sumit_to(2, 2);
        assert_eq!(result, 4);
    }
}
