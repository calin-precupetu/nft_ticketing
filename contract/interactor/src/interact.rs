#![allow(non_snake_case)]

mod config;
mod proxy;

use config::Config;
use multiversx_sc_snippets::imports::*;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::Path,
};

const STATE_FILE: &str = "state.toml";

pub async fn contract_cli() {
    env_logger::init();

    let mut args = std::env::args();
    let _ = args.next();
    let cmd = args.next().expect("at least one argument required");
    let mut interact = ContractInteract::new().await;
    match cmd.as_str() {
        "deploy" => interact.deploy().await,
        "createTrip" => interact.create_trip().await,
        "buyTicket" => interact.buy_ticket().await,
        "issueToken" => interact.issue_token().await,
        "nftTokenId" => interact.nft_token_id().await,
        "trips" => interact.trips().await,
        _ => panic!("unknown command: {}", &cmd),
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    contract_address: Option<Bech32Address>
}

impl State {
        // Deserializes state from file
        pub fn load_state() -> Self {
            if Path::new(STATE_FILE).exists() {
                let mut file = std::fs::File::open(STATE_FILE).unwrap();
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                toml::from_str(&content).unwrap()
            } else {
                Self::default()
            }
        }
    
        /// Sets the contract address
        pub fn set_address(&mut self, address: Bech32Address) {
            self.contract_address = Some(address);
        }
    
        /// Returns the contract address
        pub fn current_address(&self) -> &Bech32Address {
            self.contract_address
                .as_ref()
                .expect("no known contract, deploy first")
        }
    }
    
    impl Drop for State {
        // Serializes state to file
        fn drop(&mut self) {
            let mut file = std::fs::File::create(STATE_FILE).unwrap();
            file.write_all(toml::to_string(self).unwrap().as_bytes())
                .unwrap();
        }
    }

pub struct ContractInteract {
    interactor: Interactor,
    wallet_address: Address,
    contract_code: BytesValue,
    state: State
}

impl ContractInteract {
    pub async fn new() -> Self {
        let config = Config::new();
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.use_chain_simulator());

        interactor.set_current_dir_from_workspace("contract");
        // let wallet_address = interactor.register_wallet(test_wallets::alice()).await;

        let wallet_key = Wallet::from_pem_file("../../wallet/wallet-owner.pem").unwrap();
        let wallet_address: Address = wallet_key.to_address().into();
        interactor.register_wallet(wallet_key.clone()).await;

        // Useful in the chain simulator setting
        // generate blocks until ESDTSystemSCAddress is enabled
        interactor.generate_blocks_until_epoch(1).await.unwrap();
        
        let contract_code = BytesValue::interpret_from(
            "mxsc:../output/contract.mxsc.json",
            &InterpreterContext::default(),
        );

        ContractInteract {
            interactor,
            wallet_address,
            contract_code,
            state: State::load_state()
        }
    }

    pub async fn deploy(&mut self) {
        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(60_000_000u64)
            .typed(proxy::ContractProxy)
            .init()
            .code(&self.contract_code)
            .returns(ReturnsNewAddress)
            .run()
            .await;
        let new_address_bech32 = bech32::encode(&new_address);
        self.state
            .set_address(Bech32Address::from_bech32_string(new_address_bech32.clone()));

        println!("new address: {new_address_bech32}");
    }

    pub async fn create_trip(&mut self) {
        let egld_amount = BigUint::<StaticApi>::from(0u128);

        let source = ManagedBuffer::new_from_bytes(&b""[..]);
        let destination = ManagedBuffer::new_from_bytes(&b""[..]);
        let date = ManagedBuffer::new_from_bytes(&b""[..]);
        let time = ManagedBuffer::new_from_bytes(&b""[..]);
        let price = BigUint::<StaticApi>::from(0u128);
        let ticket_count = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::ContractProxy)
            .create_trip(source, destination, date, time, price, ticket_count)
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn buy_ticket(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let id = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::ContractProxy)
            .buy_ticket(id)
            .payment((TokenIdentifier::from(token_id.as_str()), token_nonce, token_amount))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn issue_token(&mut self) {
        let egld_amount = BigUint::<StaticApi>::from(0u128);

        let token_name = ManagedBuffer::new_from_bytes(&b""[..]);
        let token_ticker = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::ContractProxy)
            .issue_token(token_name, token_ticker)
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn nft_token_id(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::ContractProxy)
            .nft_token_id()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn trips(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::ContractProxy)
            .trips()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

}
