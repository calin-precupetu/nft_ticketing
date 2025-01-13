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

# ESDT Manager (sender) address and smart contract address (receiver)
sender = Address.new_from_bech32("erd1l0ug3af8jz902qt865rvkn0j9ja93h3mtlzkra0d4xxxgeu0325szx4mgc")
contract_address = Address.new_from_bech32("erd1qqqqqqqqqqqqqpgq6499dpelkyfe5z537t9tfa22el0kqel6325sdwnfyq")

# Token identifier and role in hexadecimal encoding
token_id = "TKT-5f29ae"  # Replace with your NFT token ID
token_id_hex = token_id.encode("utf-8").hex()

# Role(s) to assign in hexadecimal encoding
role_nft_create = "45534454526f6c654e4654437265617465"  # Hex for "ESDTRoleNFTCreate"

# Prepare the transaction data for `setSpecialRole`
data = f"setSpecialRole@{token_id_hex}@{contract_address.hex()}@{role_nft_create}"

print(data)

# Get the sender's nonce
sender_on_network = provider.get_account(sender)

# Create the transaction
transaction = Transaction(
    sender=sender.bech32(),
    receiver="erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u",  # Address of the ESDT System SC
    value=0,  # No EGLD is transferred
    gas_limit=60_000_000,  # Higher gas limit for roles assignment
    data=data.encode(),
    chain_id=network_config.chain_id,
)

# Sign the transaction
transaction.nonce = sender_on_network.nonce
transaction.signature = signer.sign(transaction_computer.compute_bytes_for_signing(transaction))

# Send the transaction
tx_hash = provider.send_transaction(transaction)
print(f"Transaction hash: {tx_hash}")
