import { ProxyNetworkProvider } from '@multiversx/sdk-core';
import { useGetNetworkConfig } from "hooks/sdkDappHooks";
import { ResultsParser } from "utils";
import { smartContract } from "utils/smartContract";

const resultsParser = new ResultsParser();
const GET_TRIPS = 'trips';

const decodeBuffer = (buffer: any) => {
    const decoder = new TextDecoder('utf-8');
    return decoder.decode(buffer);
  };

export const useGetTrips = () => {
    const { network } = useGetNetworkConfig();

    return async () => {
        try {
            const query = smartContract.createQuery({
                func: GET_TRIPS,
                args: []
            });
            const provider = new ProxyNetworkProvider(network.apiAddress);
            const queryResponse = await provider.queryContract(query);
            const endpointDefinition = smartContract.getEndpoint(GET_TRIPS);
            const { values } = resultsParser.parseQueryResponse(queryResponse, endpointDefinition);

            const plainJsonValues = values.map((typedValue: any) => typedValue.valueOf());

            const tripsDecoded = plainJsonValues[0].map((trip: any) => {
                return {
                  id: trip.id,
                  source: decodeBuffer(trip.source), 
                  destination: decodeBuffer(trip.destination),
                  date: decodeBuffer(trip.date),
                  time: decodeBuffer(trip.time),
                  price: trip.price / 100,
                  ticket_count: trip.ticket_count
                };
              });   
              
            console.log("Decoded Trips:", tripsDecoded);
            
            return tripsDecoded;
        } catch (err) {
            console.error(`Unable to call ${GET_TRIPS}`, err);
        }
    };
};
