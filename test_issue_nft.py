from pathlib import Path
from multiversx_sdk import (
    Address,
    Transaction,
    UserSigner,
    ProxyNetworkProvider,
    TransactionComputer
)

def string_to_hex(string: str) -> str:
    return string.encode("utf-8").hex()

# Initialize network provider and configurations
provider = ProxyNetworkProvider("https://devnet-gateway.multiversx.com")
network_config = provider.get_network_config()

# Initialize signer from wallet PEM file
signer = UserSigner.from_pem_file(Path("wallet/wallet-owner.pem"))
transaction_computer = TransactionComputer()

# Sender and contract addresses
sender = Address.new_from_bech32("erd1l0ug3af8jz902qt865rvkn0j9ja93h3mtlzkra0d4xxxgeu0325szx4mgc")
contract_address = Address.new_from_bech32("erd1qqqqqqqqqqqqqpgqq0l6kprcc3ngpaxj7g70cnxtk7n99pw3325s8pu4mx")

# Get the sender's nonce
sender_on_network = provider.get_account(sender)

# Define token display name and ticker for the NFT
token_display_name = "Tickets"  # The display name of the NFT collection
token_ticker = "TKT"  # The ticker for the NFT

token_display_name_hex = string_to_hex(token_display_name)
token_ticker_hex = string_to_hex(token_ticker)

# Prepare transaction data for the issueNft endpoint
function = "issueToken"
data = f"{function}@{token_display_name_hex}@{token_ticker_hex}"

# Create the transaction
transaction = Transaction(
    sender=sender.bech32(),
    receiver=contract_address.bech32(),
    value=50000000000000000,
    gas_limit=60000000,
    data=data.encode(),
    chain_id=network_config.chain_id,
)

# Sign and send the transaction
transaction.nonce = sender_on_network.nonce
transaction.signature = signer.sign(transaction_computer.compute_bytes_for_signing(transaction))

# Send the transaction to the network
tx_hash = provider.send_transaction(transaction)
print(f"Transaction hash: {tx_hash}")
