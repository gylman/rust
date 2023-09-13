use base64::engine::general_purpose;
use base64::Engine as _;
use bincode::{deserialize, serialize};
use dotenv::dotenv;
use hyper::header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use hyper::{Body, Client, Request};
use rocksdb::DB;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;
use std::path::Path;
use std::time::Duration;
use tokio;
use tokio::runtime;

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

fn encode_data_to_base64(original: &str) -> String {
    // Convert string to bytes
    let bytes = original.as_bytes();
    // Convert bytes to base64
    let base64_str: String = general_purpose::STANDARD.encode(&bytes);
    base64_str
}

async fn submit_to_da(data: &str) -> String {
    dotenv().ok();
    let da_host = env::var("DA_HOST").expect("DA_HOST must be set");
    let da_namespace = env::var("DA_NAMESPACE").expect("DA_NAMESPACE must be set");
    let da_auth_token = env::var("DA_AUTH_TOKEN").expect("DA_AUTH_TOKEN must be set");
    let da_auth = format!("Bearer {}", da_auth_token);

    // println!("this is the da_namespace: {:?}", da_namespace);

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

    // Token should be removed from code.
    let req = Request::post(uri.as_str())
        .header(
            AUTHORIZATION,
            HeaderValue::from_str(da_auth.as_str()).unwrap(),
        )
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

    if let Some(result_value) = parsed.get("result") {
        result_value.to_string()
    } else {
        "".to_string()
    }
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
            let data_for_da: String = serde_json::to_string(&deserialized_tx).unwrap();
            let encoded_data_for_da = encode_data_to_base64(&data_for_da);
            let rt = runtime::Runtime::new().unwrap();

            println!("Testing Submitting large data size...");
            rt.block_on(async {
                submit_to_da(&encoded_data_for_da).await;
            });
        }
        i += 1;
    }
}
