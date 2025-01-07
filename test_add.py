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
contract_address = Address.new_from_bech32("erd1qqqqqqqqqqqqqpgqvfmcv9acsvgf45n94kwx5p84k48safy9t7kqj44xyu")

# Get the sender's nonce
sender_on_network = provider.get_account(sender)

# Trip details
source = "426563686172657374"  # "Bucharest" in hex
destination = "437261696f7661"  # "Craiova" in hex
date = "323032352d30312d3130"  # "2025-01-10" in hex
time = "31323a3330"            # "12:30" in hex
price = "32"                   # 50 (in hundredths of EGLD, i.e., 0.50 EGLD) in hex
ticket_count = "64"            # 100 in decimal, in hex

# Prepare transaction data for the addTrip endpoint
function = "addTrip"
data = f"{function}@{source}@{destination}@{date}@{time}@{price}@{ticket_count}"

# Create the transaction
transaction = Transaction(
    sender=sender.bech32(),
    receiver=contract_address.bech32(),
    value=0,  # No EGLD transferred for adding a trip
    gas_limit=5000000,
    data=data.encode(),
    chain_id=network_config.chain_id,
)

# Sign and send the transaction
transaction.nonce = sender_on_network.nonce
transaction.signature = signer.sign(transaction_computer.compute_bytes_for_signing(transaction))

# Send the transaction to the network
tx_hash = provider.send_transaction(transaction)
print(f"Transaction hash: {tx_hash}")
