from pathlib import Path
from multiversx_sdk import (
    Address,
    Transaction,
    UserSigner,
    ProxyNetworkProvider,
    TransactionComputer
)

# Initialize network provider and configurations
provider = ProxyNetworkProvider("https://devnet-gateway.multiversx.com")
network_config = provider.get_network_config()

# Initialize signer from wallet PEM file
signer = UserSigner.from_pem_file(Path("/home/usuario/Desktop/BLOCKCHAIN/new_wallet.pem"))
transaction_computer = TransactionComputer()

# Sender and contract addresses
sender = Address.new_from_bech32("erd1fzg7jf50sdn6rka8vpqueyqzq96ncfjl3ccgvjz0puj97t6ht7kq6kutgs")
contract_address = Address.new_from_bech32("erd1qqqqqqqqqqqqqpgq6499dpelkyfe5z537t9tfa22el0kqel6325sdwnfyq")

# Get the sender's nonce
sender_on_network = provider.get_account(sender)

# Trip ID for the ticket purchase
trip_id = 1  # Replace with the desired trip ID (e.g., 0 for the first trip)
trip_id_hex = "02"

ticket_price = 10**18

# Prepare transaction data for the buyTicket endpoint
function = "buyTicket"
data = f"{function}@{trip_id_hex}"

# Create the transaction
transaction = Transaction(
    sender=sender.bech32(),
    receiver=contract_address.bech32(),
    value=ticket_price,  # Amount to pay for the ticket
    gas_limit=5_000_000,
    data=data.encode(),
    chain_id=network_config.chain_id,
)

# Sign and send the transaction
transaction.nonce = sender_on_network.nonce
transaction.signature = signer.sign(transaction_computer.compute_bytes_for_signing(transaction))

# Send the transaction to the network
tx_hash = provider.send_transaction(transaction)
print(f"Transaction hash: {tx_hash}")
