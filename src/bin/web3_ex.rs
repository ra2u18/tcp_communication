use hex::FromHex;
use hex_literal::hex;
use std::time;
use web3::{
    contract::{Contract, Options},
    futures::StreamExt,
    types::{FilterBuilder, H160, H256},
    signing::{keccak256}
};
use std::str::FromStr;

#[tokio::main]
async fn main () -> web3::contract::Result<()> {
  let uri = "https://mainnet.infura.io/v3/6bc1bae7fbdd4ab89c9fb9eab62601d9";
  let tether_usdt_addr = "0xdAC17F958D2ee523a2206206994597C13D831ec7";

  let web3 = web3::Web3::new(web3::transports::Http::new(uri)?);

  let event_sign: &[u8] = "Transfer(address,address,uint256)".as_bytes();
  let event_sign = keccak256(event_sign);

  println!("{:?}", event_sign);
  // let decoded_msg = &hex::decode(msg).expect("Decoding failed");

  // println!("{:?}", H160::from_str(tether_usdt_addr).unwrap());

  let filter = FilterBuilder::default()
    .address(vec![H160::from_str(tether_usdt_addr).unwrap()])
    .topics(
      Some(vec![H256::from(event_sign)]),
      None,
      None,
      None
    )
    .build();

  // 1
  let filter = web3.eth_filter().create_logs_filter(filter).await?;
  let logs_stream = filter.stream(time::Duration::from_millis(100));
  futures::pin_mut!(logs_stream);

  loop {
    let log = logs_stream.next().await.unwrap();
    println!("got log: {:?}", log);
  }

  Ok(())
}