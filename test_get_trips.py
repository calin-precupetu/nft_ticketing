from pathlib import Path
from multiversx_sdk import (
    ProxyNetworkProvider,
    QueryRunnerAdapter,
    SmartContractQueriesController,
    Address
)

# Replace with your contract address
contract = Address.new_from_bech32("erd1qqqqqqqqqqqqqpgqrtewahvjcyel89zrtlvdgcgquldwq5sp325s0d5dl6")
query_runner = QueryRunnerAdapter(ProxyNetworkProvider("https://devnet-api.multiversx.com"))
query_controller = SmartContractQueriesController(query_runner)

# Function to query the trips
def get_trips():
    data_parts = query_controller.query(
        contract=contract.to_bech32(),
        function="trips",
        arguments=[],
    )
    return data_parts

# Fetch the trips and write them to a file
trips = get_trips()

output_path = Path("trips.txt")
with output_path.open("w") as file:
    for trip in trips:
        file.write(f"{trip}\n")  # Write raw data to file

print(f"Trips have been written to {output_path}")
