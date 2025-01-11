#![no_std]

use multiversx_sc::imports::*;
use multiversx_sc::derive_imports::*;

const NFT_AMOUNT: u32 = 1;

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

#[multiversx_sc::contract]
pub trait Contract {
    #[init]
    fn init(&self) {}

    #[only_owner]
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
        require!(price >= BigUint::from(25u32) && price <= BigUint::from(100u32), "Price must be between 0.25 and 1 EGLD");
        require!(ticket_count > 0, "Ticket count must be greater than 0");

        let id = self.trips().len() as u32 + 1;

        let trip = TrainTrip {
            id,
            source,
            destination,
            date,
            time,
            price,
            ticket_count,
        };

        self.trips().push(&trip);
    }


    #[endpoint(buyTicket)]
    #[payable("*")]
    fn buy_ticket(
        &self,
        id: u64
    ) {
        require!(self.trips().len() > id as usize, "Trip with specified ID does not exist");

        let trip = self.trips().get(id as usize);
        
        let total_paid = self.call_value().egld_value();
        require!(total_paid.clone_value() == trip.price, "Payment does not match the ticket price");

        require!(trip.ticket_count > 0, "No tickets available");

        let mut updated_trip = trip.clone();
        updated_trip.ticket_count -= 1;
        
        self.trips().set(id as usize, &updated_trip);
        
        //  creare NFT

        require!(!self.token_id().is_empty(), "NFT not issued");

        let mut ticket_attributes_buffer = ManagedBuffer::new();
        ticket_attributes_buffer.append(&ManagedBuffer::from(b"TripID: "));
        let id_bytes = id.to_le_bytes();
        ticket_attributes_buffer.append(&ManagedBuffer::from(&id_bytes));

        self.token_id().nft_create_and_send(
            &self.blockchain().get_caller(),
            BigUint::from(1u32),
            &ticket_attributes_buffer,
        );

        // // Create NFT with the trip ID as an attribute
        // let nft_token_id = TokenIdentifier::from(ManagedBuffer::from(b"train_ticket"));
        // let mut ticket_attributes_buffer = ManagedBuffer::new();
        // ticket_attributes_buffer.append(&ManagedBuffer::from(b"TripID: "));
        // let id_bytes = id.to_le_bytes();
        // ticket_attributes_buffer.append(&ManagedBuffer::from(&id_bytes));

        // // Hash the attributes to create a unique identifier for the NFT
        // let attributes_sha256 = self.crypto().sha256(&ticket_attributes_buffer);
        // let attributes_hash = attributes_sha256.as_managed_buffer();

        // // Define the name of the NFT (could be the trip's source and destination for example)
        // let nft_name = ManagedBuffer::from(b"Train Ticket");

        // // Create the NFT
        // self.send().esdt_nft_create(
        //     &nft_token_id,                // NFT token ID (class identifier)
        //     &BigUint::from(1u32),          // Quantity (1 ticket per NFT)
        //     &nft_name,                     // Name of the NFT
        //     &BigUint::from(0u32),          // Royalties (assuming no royalties for simplicity)
        //     attributes_hash,               // Hash of the attributes
        //     &ManagedBuffer::new(),         // Card details or additional metadata
        //     &ManagedVec::new(),            // Additional optional fields
        // );

        // let nonce: u64 = 1;

        // self.send().direct_esdt(
        //     &self.blockchain().get_caller(),
        //     self.token_id().get_token_id_ref(),
        //     &BigUint::from(NFT_AMOUNT),
        // );

        // let payment = self.call_value().egld_value();
        // let owner = self.blockchain().get_owner_address();
        // self.tx().to(owner).payment(payment).transfer();
    }

    #[only_owner]
    #[endpoint(issueNft)]
    #[payable("EGLD")]
    fn issue_nft(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer
    ) {
        require!(self.token_id().is_empty(), "NFT already issued");

        let issue_cost = self.call_value().egld_value();

        self.token_id().issue_and_set_all_roles(
            EsdtTokenType::NonFungible,
            issue_cost.clone_value(),
            token_display_name,
            token_ticker,
            0,
            None,
        );
    }

    fn send_nft_to_caller(
        &self,
        nonce: u64
    ) {
        self.send().direct_esdt(
            &self.blockchain().get_caller(),
            self.token_id().get_token_id_ref(),
            nonce,
            &BigUint::from(NFT_AMOUNT),
        );
    }

    // View

    #[view(getTokenId)]
    fn get_token_id(&self) -> TokenIdentifier {
        self.token_id().get_token_id()
    }

    #[view(getTokenData)]
    fn get_token_data(&self, token_nonce: u64) -> EsdtTokenData {
        self.token_id().get_all_token_data(token_nonce)
    }

    // Storage mappers

    #[view(tokenId)]
    #[storage_mapper("tokenId")]
    fn token_id(&self) -> NonFungibleTokenMapper;

    #[view(trips)]
    #[storage_mapper("trips")]
    fn trips(&self) -> VecMapper<TrainTrip<Self::Api>>;
}
