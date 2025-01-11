from pathlib import Path
from multiversx_sdk import (
    ProxyNetworkProvider,
    QueryRunnerAdapter,
    SmartContractQueriesController,
    Address
)

# Replace with your contract address
contract = Address.new_from_bech32("erd1qqqqqqqqqqqqqpgquryd8zwcq6fcy6867ahjszpcj22sy2je325swas6jf")
query_runner = QueryRunnerAdapter(ProxyNetworkProvider("https://devnet-api.multiversx.com"))
query_controller = SmartContractQueriesController(query_runner)

# Function to query the getTokenId
def get_token_id():
    data_parts = query_controller.query(
        contract=contract.to_bech32(),
        function="getTokenId",
        arguments=[],  # No arguments for this view function
    )
    return data_parts

# Fetch the token ID and write it to a file
token_id = get_token_id()

output_path = Path("token_id.txt")
with output_path.open("w") as file:
    file.write(f"Token ID: {token_id[0]}\n")  # Assuming the response is a single value

print(f"Token ID has been written to {output_path}")
