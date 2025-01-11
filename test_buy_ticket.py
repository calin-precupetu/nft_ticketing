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
contract_address = Address.new_from_bech32("erd1qqqqqqqqqqqqqpgquryd8zwcq6fcy6867ahjszpcj22sy2je325swas6jf")

# Get the sender's nonce
sender_on_network = provider.get_account(sender)

# Trip ID for the ticket purchase
trip_id = 1  # Replace with the desired trip ID (e.g., 0 for the first trip)
trip_id_hex = f"{trip_id:02x}"

ticket_price = 50 

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
