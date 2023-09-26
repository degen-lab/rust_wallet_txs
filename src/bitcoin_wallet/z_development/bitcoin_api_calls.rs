#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("RPC Error: {0}")]
    RPCError(String),
    #[error("{0}")]
    InvalidResponseJSON(String),
    #[error("Invalid utxo: {0}")]
    InvalidUTXO(String),
    #[error("Invalid transaction hash")]
    InvalidTxHash,
}

#[test]
fn call_getblockcount() {
    // debug!("Making Bitcoin RPC {} call...", method);
    let json_rpc =
        serde_json::json!({"jsonrpc": "2.0", "id": "stx", "method": "getblockcount", "params": []});
    let url = "https://frosty-dry-haze.btc-testnet.quiknode.pro/4a2d4b738df1f216ebbc46957c44edfae8b6cb86/";

    let response = ureq::post(url)
        .send_json(json_rpc)
        .map_err(|e| Error::RPCError(parse_rpc_error(e)))
        .unwrap();

    let json_response = response
        .into_json::<serde_json::Value>()
        .map_err(|_| Error::InvalidResponseJSON("Invalid JSON response.".to_string()))
        .unwrap();

    // let json_response = response.into_json::<serde_json::Value>()?;
    let json_result = json_response
        .get("result")
        .ok_or_else(|| Error::InvalidResponseJSON("Missing entry 'result'.".to_string()))
        .unwrap()
        .to_owned();
    println!("block height {}", json_result);
    // Ok(json_result)
}

#[test]
fn call_listunspent() {
    // debug!("Making Bitcoin RPC {} call...", method);
    let addresses: Vec<String> = vec!["mxVFsFW5N4mu1HPkxPttorvocvzeZ7KZyk".to_string()];
    let min_conf = 0i64;
    let max_conf = 9999999i64;
    // let params = [min_conf, max_conf, addresses];
    let json_rpc = serde_json::json!({"jsonrpc": "2.0", "id": "stx", "method": "listunspent", "params": [min_conf, max_conf]});
    let url = "https://frosty-dry-haze.btc-testnet.quiknode.pro/4a2d4b738df1f216ebbc46957c44edfae8b6cb86/";

    let response = ureq::post(url)
        .send_json(json_rpc)
        .map_err(|e| Error::RPCError(parse_rpc_error(e)))
        .unwrap();

    let json_response = response
        .into_json::<serde_json::Value>()
        .map_err(|_| Error::InvalidResponseJSON("Invalid JSON response.".to_string()))
        .unwrap();

    // let json_response = response.into_json::<serde_json::Value>()?;
    let json_result = json_response
        .get("result")
        .ok_or_else(|| Error::InvalidResponseJSON("Missing entry 'result'.".to_string()))
        .unwrap()
        .to_owned();
    println!("block height {}", json_result);
    // Ok(json_result)
}

fn parse_rpc_error(err: ureq::Error) -> String {
    match err {
        ureq::Error::Status(status, response) => format!(
            "{} {}",
            status,
            response.into_string().unwrap_or_else(|e| e.to_string())
        ),
        ureq::Error::Transport(err) => err.to_string(),
    }
}
