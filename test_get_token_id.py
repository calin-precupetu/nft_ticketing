from pathlib import Path
from multiversx_sdk import (
    ProxyNetworkProvider,
    QueryRunnerAdapter,
    SmartContractQueriesController,
    Address
)

# Replace with your contract address
contract = Address.new_from_bech32("erd1qqqqqqqqqqqqqpgqtcvvvpkqzg740uspt3dsgjr9pvg08xxv325sag5udl")
query_runner = QueryRunnerAdapter(ProxyNetworkProvider("https://devnet-api.multiversx.com"))
query_controller = SmartContractQueriesController(query_runner)

# Function to query the getTokenId
def get_token_ids():
    data_parts = query_controller.query(
        contract=contract.to_bech32(),
        function="nftTokenIds",
        arguments=[],  # No arguments for this view function
    )
    return data_parts

# Fetch the token ID and write it to a file
token_ids = get_token_ids()

output_path = Path("token_id.txt")
with output_path.open("w") as file:
    for token_id in token_ids:
        file.write(f"Token ID: {token_id}\n")

print(f"Token ID has been written to {output_path}")
