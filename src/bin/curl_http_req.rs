use web3::types::{H160, H256};
use serde::{Serialize, Deserialize};
use serde_json::{Value, json};

/// Address struct 
#[derive(Serialize, Debug)]
struct Address {
  address: H160
}
impl Address {
  fn from(addr: String) -> Self {
    Self { address: Self::to_160(addr) }
  }

  /// Convert string address to H256 ([u8; 32]) 
  /// 
  /// Panics: when the address doesn't have the right format 
  fn to_256(addr: String) -> H256 {
    let addr: &[u8] = &hex::decode(&addr[2..]).unwrap()[..];

    if addr.len() > std::mem::size_of::<[u8; 20]>() {
      panic!("Introduce a valid format for the address")
    }

    H256::from(H160::from_slice(addr))
  }

  /// Convert string address to H256 ([u8; 32]) 
  /// 
  /// Panics: when the address doesn't have the right format 
  fn to_160(addr: String) -> H160 {
    let addr: &[u8] = &hex::decode(&addr[2..]).unwrap()[..];

    if addr.len() > std::mem::size_of::<[u8; 20]>() {
      panic!("Introduce a valid format for the address")
    }

    H160::from_slice(addr)
  }
}

impl ToString for Address {
  fn to_string(&self) -> String {
    H160::to_string(&self.address)
  }
}

/// Topics struct
#[derive(Serialize, Debug)]
struct Topics {
  topics: Vec<H256>
}

impl Topics {
  fn new() -> Self {
    Topics { topics: vec![] }
  }

  fn push(&mut self, val: H256) {
    self.topics.push(val);
  }
}

impl ToString for Topics {
  fn to_string(&self) -> String {
    String::from(serde_json::to_string(&self).unwrap())
  }
}

#[derive(Deserialize, Debug)]
struct Response {
  address: String,
  blockHash: String,
  blockNumber: String,
  data: String,
  logIndex: String,
  removed: bool,
  topics: Vec<String>,
  transactionIndex: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
  let uri = "https://bsc-dataseed.binance.org";
  let tether_usdt_addr = String::from("0xdAC17F958D2ee523a2206206994597C13D831ec7");

  let address_param = Address::from(tether_usdt_addr);
  let topics_param = Topics::new();

  let payload = json!({ 
    "jsonrpc": "2.0",
    "method": "eth_getLogs",
    "params": [{ "topics": [] }], //"address": address_param.address,
    "id": "83"
  }); 

  // HTTP req using payload
  let client = reqwest::Client::new();
  let res: Value = client.post(uri)
      .json(&payload)
      .send()
      .await?
      .json()
      .await
      .expect("failed to parse resp");

  // Deserialized the http resp into Vec<Response>
  let r: Vec<Response> = serde_json::from_value(res["result"].clone()).unwrap();

  Ok(())
}