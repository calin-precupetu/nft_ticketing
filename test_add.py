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
signer = UserSigner.from_pem_file(Path("wallet/wallet-owner.pem"))
transaction_computer = TransactionComputer()

# Sender and contract addresses
sender = Address.new_from_bech32("erd1l0ug3af8jz902qt865rvkn0j9ja93h3mtlzkra0d4xxxgeu0325szx4mgc")
contract_address = Address.new_from_bech32("erd1qqqqqqqqqqqqqpgqrtewahvjcyel89zrtlvdgcgquldwq5sp325s0d5dl6")

# Get the sender's nonce
sender_on_network = provider.get_account(sender)

# 427563686172657374 

# Trip details
source = "436f6e7374616e7461"  # "Bucharest" in hex
destination = "437261696f7661"  # "Craiova" in hex
date = "323032352d30312d3130"  # "2025-01-10" in hex
time = "31323a3330"            # "12:30" in hex
price = "32"                   # 50 (in hundredths of EGLD, i.e., 0.50 EGLD) in hex
ticket_count = "64"            # 100 in decimal, in hex

# Prepare transaction data for the addTrip endpoint
function = "createTrip"
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
