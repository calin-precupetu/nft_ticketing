import { SmartContractTransactionsFactory, Transaction, TransactionsFactoryConfig, U32Value, ApiNetworkProvider } from "@multiversx/sdk-core";
import { useGetAccount } from "hooks/sdkDappHooks";
import { Address } from "@multiversx/sdk-core";
import { smartContract } from "utils/smartContract";
import { signAndSendTransactions } from "helpers/signAndSendTransactions";
import { WalletProvider, WALLET_PROVIDER_DEVNET } from "@multiversx/sdk-web-wallet-provider";

const provider = new WalletProvider(WALLET_PROVIDER_DEVNET);

const factoryConfig = new TransactionsFactoryConfig({ chainID: "D" });

let factory = new SmartContractTransactionsFactory({
    config: factoryConfig
});

const BUY_TRANSACTION_INFO = {
    processingMessage: 'Processing buy transaction',
    errorMessage: 'An error has occured during buy',
    successMessage: 'Buy transaction successful'
  };

export const useBuyTicket = () => {
    const { address } = useGetAccount();
    const apiNetworkProvider = new ApiNetworkProvider("https://devnet-api.multiversx.com", { clientName: "multiversx-your-client-name" });

    return async (id: string, price: number) => {
        console.log('Buying ticket for trip ID:', id);
        console.log('Price:', price);

        const gasLimit = BigInt(50000000);
        const args = [new U32Value(id)];

        const callbackUrl = encodeURIComponent("https://localhost:3000/dashboard");
        
        const transaction = factory.createTransactionForExecute({
            sender: Address.fromBech32(address),
            contract: smartContract.getAddress(),
            nativeTransferAmount: BigInt(price * 100),
            function: "buyTicket",
            gasLimit: gasLimit,
            arguments: args
        });

        const sessionId = await signAndSendTransactions({
            transactions: [transaction],
            callbackRoute: callbackUrl,
            transactionsDisplayInfo: BUY_TRANSACTION_INFO
          });

    };
};