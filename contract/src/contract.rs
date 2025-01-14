#![no_std]

use multiversx_sc::imports::*;
use multiversx_sc::derive_imports::*;

const NFT_AMOUNT: u32 = 1;
const ROYALTIES_MAX: u32 = 10_000; 

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, Clone, PartialEq, Debug)]
pub struct TrainTrip<M: ManagedTypeApi> {
    pub id: u32,
    pub source: ManagedBuffer<M>,
    pub destination: ManagedBuffer<M>,
    pub date: ManagedBuffer<M>,
    pub time: ManagedBuffer<M>,
    pub price: BigUint<M>,
    pub ticket_count: u32,
}

impl<M: ManagedTypeApi> TrainTrip<M> {
    pub fn new(
        id: u32,
        source: ManagedBuffer<M>,
        destination: ManagedBuffer<M>,
        date: ManagedBuffer<M>,
        time: ManagedBuffer<M>,
        price: BigUint<M>,
        ticket_count: u32,
    ) -> Result<Self, &'static str> {
        if ticket_count == 0 {
            return Err("Ticket count must be greater than 0");
        }

        Ok(Self {
            id,
            source,
            destination,
            date,
            time,
            price,
            ticket_count,
        })
    }
}

#[multiversx_sc::contract]
pub trait Contract {
    #[init]
    fn init(&self) {}

    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(createTrip)]
    fn create_trip(
        &self,
        source: ManagedBuffer,
        destination: ManagedBuffer,
        date: ManagedBuffer,
        time: ManagedBuffer,
        price: BigUint,
        ticket_count: u32,
    ) {
        require!(ticket_count > 0, "Ticket count must be greater than 0");

        let id = self.trips().len() as u32 + 1;

        let trip = TrainTrip {
            id: id.clone(),
            source: source.clone(),
            destination: destination.clone(),
            date: date.clone(),
            time: time.clone(),
            price: price.clone(),
            ticket_count: ticket_count.clone(),
        };

        self.trips().push(&trip);

        let nft_token_id = self.nft_token_ids().get((id) as usize);

        let trip_details = match TrainTrip::new(id.clone(),
                                                source.clone(),
                                                destination.clone(),
                                                date.clone(),
                                                time.clone(),
                                                price.clone(),
                                                ticket_count.clone()) {
            Ok(details) => details, 
            Err(_) => sc_panic!("Error creating card details"),
        };

        let mut serialized_attributes = ManagedBuffer::new();
        if let core::result::Result::Err(err) = trip_details.top_encode(&mut serialized_attributes) {
            sc_panic!("Attributes encode error: {}", err.message_bytes());
        }

        let attributes_sha256 = self.crypto().sha256(&serialized_attributes);
        let attributes_hash = attributes_sha256.as_managed_buffer();
        for _ in 0..ticket_count {
            let mut name = ManagedBuffer::new();
            name.append(&ManagedBuffer::from(b"Ticket "));

            self.send().esdt_nft_create(
                &nft_token_id,
                &BigUint::from(NFT_AMOUNT),
                &name,
                &BigUint::from(ROYALTIES_MAX),
                attributes_hash,
                &trip_details,
                &ManagedVec::new(),
            );
        }
    }

    #[endpoint(buyTicket)]
    #[payable("*")]
    fn buy_ticket(
        &self,
        id: u64
    ) {
        require!(self.trips().len() >= id as usize, "Trip with specified ID does not exist");

        let trip = self.trips().get(id as usize);
        
        let total_paid = self.call_value().egld_value();
        require!(total_paid.clone_value() == trip.price , "Payment does not match the ticket price");
        
        require!(trip.ticket_count > 0, "No tickets available.");

        let nft_nonce = trip.ticket_count.into();

        let mut updated_trip = trip.clone();
        updated_trip.ticket_count -= 1;
        
        self.trips().set(id as usize, &updated_trip);

        let nft_token_id = self.nft_token_ids().get(id as usize);

        self.send().direct_esdt(
            &self.blockchain().get_caller(),
            &nft_token_id,
            nft_nonce,
            &BigUint::from(NFT_AMOUNT),
        );
    }

    #[only_owner]
    #[endpoint(setSpecialRoles)]
    fn set_special_roles(&self) {
        // Retrieve the last token ID from the vector
        let nft_token_ids = self.nft_token_ids();
        let last_index = nft_token_ids.len();
        let last_token_id = nft_token_ids.get(last_index);

        self.send()
            .esdt_system_sc_tx()
            .set_special_roles(
                &self.blockchain().get_sc_address(),
                &last_token_id,
                [EsdtLocalRole::NftCreate][..].iter().cloned(),
            )
            .async_call_and_exit();
    }


    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueToken)]
    fn issue_token(&self, token_name: ManagedBuffer, token_ticker: ManagedBuffer) {
        let payment_amount = self.call_value().egld_value();
        self.send()
            .esdt_system_sc_tx()
            .issue_non_fungible(
                payment_amount.clone_value(),
                &token_name,
                &token_ticker,
                NonFungibleTokenProperties {
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_transfer_create_role: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .with_callback(self.callbacks().issue_callback())
            .async_call_and_exit()
    }

    #[callback]
    fn issue_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<EgldOrEsdtTokenIdentifier>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {
                self.nft_token_ids().push(&token_id.unwrap_esdt());
            }
            ManagedAsyncCallResult::Err(_) => {
                let returned = self.call_value().egld_or_single_esdt();
                if returned.token_identifier.is_egld() && returned.amount > 0 {
                    self.tx().to(ToCaller).egld(returned.amount).transfer();
                }
            }
        }
    }

    // Storage mappers

    #[view(nftTokenIds)]
    #[storage_mapper("nftTokenIds")]
    fn nft_token_ids(&self) -> VecMapper<TokenIdentifier>;

    #[view(trips)]
    #[storage_mapper("trips")]
    fn trips(&self) -> VecMapper<TrainTrip<Self::Api>>;
}
