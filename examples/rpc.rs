use starknet::{
    core::types::Felt,
    providers::{jsonrpc::HttpTransport, JsonRpcClient},
};
use starknet_id::{
    naming::{ResolvingError, MAINNET_CONTRACT},
    ProviderExt,
};
use url::Url;

fn create_jsonrpc_client() -> JsonRpcClient<HttpTransport> {
    let rpc_url =
        std::env::var("STARKNET_RPC").unwrap_or("https://rpc.starknet.id/rpc/v0_9".into());
    JsonRpcClient::new(HttpTransport::new(Url::parse(&rpc_url).unwrap()))
}

#[tokio::main]
async fn main() {
    let client_mainnet = create_jsonrpc_client();
    println!("On mainnet:");
    let addr = client_mainnet
        .domain_to_address("th0rgal.stark", MAINNET_CONTRACT)
        .await;
    match addr {
        Ok(addr) => println!("address: 0x{:x}", addr),
        Err(err) => match err {
            ResolvingError::ConnectionError(cause) => println!("Connection error: {}", cause),
            ResolvingError::InvalidContractResult => println!("Invalid contract result"),
            ResolvingError::InvalidDomain => println!("Invalid domain"),
            ResolvingError::NotSupported => println!("Resolving not supported"),
        },
    }

    let domain_result = client_mainnet
        .address_to_domain(
            Felt::from_hex("0x00a00373a00352aa367058555149b573322910d54fcdf3a926e3e56d0dcb4b0c")
                .unwrap(),
            MAINNET_CONTRACT,
        )
        .await;
    match domain_result {
        Ok(domain_result) => println!("domain: {}", domain_result),
        Err(err) => match err {
            ResolvingError::ConnectionError(cause) => println!("Connection error: {}", cause),
            ResolvingError::InvalidContractResult => println!("Invalid contract result"),
            ResolvingError::InvalidDomain => println!("Invalid domain"),
            ResolvingError::NotSupported => println!("Resolving not supported"),
        },
    }
}
