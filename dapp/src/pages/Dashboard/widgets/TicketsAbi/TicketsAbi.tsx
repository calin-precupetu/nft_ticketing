import { faMoneyBill, faRefresh } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';

import { Button } from 'components/Button';
import { ContractAddress } from 'components/ContractAddress';
import { OutputContainer } from 'components/OutputContainer';
import { useGetNetworkConfig, useBuyTicket, useGetTrips } from 'hooks';
import { useEffect, useState } from 'react';
import { WidgetProps } from 'types';

export const TicketsAbi = ({ callbackRoute }: WidgetProps) => {
  const { network } = useGetNetworkConfig();
  const buyTicket = useBuyTicket();
  const getTrips = useGetTrips();

  const [trips, setTrips] = useState<any>(null);
  const [selectedTrip, setSelectedTrip] = useState<any>(null);
  const [sourceFilter, setSourceFilter] = useState<string>('');
  const [destinationFilter, setDestinationFilter] = useState<string>('');
  const [sortOrder, setSortOrder] = useState<'asc' | 'desc'>('asc');

  const handleGetTrips = async () => {
    const tripsData = await getTrips();
    console.log(tripsData);
    setTrips(tripsData);
  };

  const handleSelectTrip = (trip: any) => {
    setSelectedTrip(trip);
  };

  const handleBuyTicket = async () => {
    if (selectedTrip) {
      const { id, price } = selectedTrip;
      await buyTicket(id, price);
      console.log(`Ticket bought for trip with ID: ${id} at price: ${price}`);
    } else {
      console.log("No trip selected.");
    }
  };

  useEffect(() => {
    handleGetTrips();
  }, []);

  const filteredAndSortedTrips = trips
    ?.filter(
      (trip: any) =>
        (!sourceFilter || trip.source.toLowerCase().includes(sourceFilter.toLowerCase())) &&
        (!destinationFilter || trip.destination.toLowerCase().includes(destinationFilter.toLowerCase()))
    )
    .sort((a: any, b: any) => {
      const dateA = new Date(a.date).getTime();
      const dateB = new Date(b.date).getTime();
      return sortOrder === 'asc' ? dateA - dateB : dateB - dateA;
    });

  return (
    <div className="flex flex-col gap-6">
      <div className="flex flex-col gap-2">
      <div className="flex gap-4">
        <input
          type="text"
          placeholder="Filter by source"
          value={sourceFilter}
          onChange={(e) => setSourceFilter(e.target.value)}
          className="p-2 border rounded w-50"
        />
        <input
          type="text"
          placeholder="Filter by destination"
          value={destinationFilter}
          onChange={(e) => setDestinationFilter(e.target.value)}
          className="p-2 border rounded w-50"
        />
        <select
          value={sortOrder}
          onChange={(e) => setSortOrder(e.target.value as 'asc' | 'desc')}
          className="p-2 border rounded w-50"
        >
          <option value="asc">Sort by Date (Asc)</option>
          <option value="desc">Sort by Date (Desc)</option>
        </select>
      </div>

        <div className="flex justify-start gap-2">
          <Button
            onClick={handleGetTrips}
            className="inline-block rounded-lg px-3 py-2 text-center hover:no-underline my-0 bg-blue-600 text-white hover:bg-blue-700 mr-0 disabled:bg-gray-200 disabled:text-black disabled:cursor-not-allowed"
          >
            <FontAwesomeIcon icon={faRefresh} className="mr-1" />
            Refresh Trips
          </Button>

          <Button
            onClick={handleBuyTicket}
            className="inline-block rounded-lg px-3 py-2 text-center hover:no-underline my-0 bg-blue-600 text-white hover:bg-blue-700 mr-0 disabled:bg-gray-200 disabled:text-black disabled:cursor-not-allowed"
            disabled={!selectedTrip}
          >
            <FontAwesomeIcon icon={faMoneyBill} className="mr-1" />
            Buy Ticket
          </Button>
        </div>
      </div>

      <OutputContainer>
        <ContractAddress />
      </OutputContainer>
      <div>
        <h3>Trips:</h3>
        {filteredAndSortedTrips && filteredAndSortedTrips.length > 0 ? (
          <table className="min-w-full table-auto border-collapse border border-gray-300">
            <thead>
              <tr>
                <th className="border border-gray-300 px-4 py-2">ID</th>
                <th className="border border-gray-300 px-4 py-2">Source</th>
                <th className="border border-gray-300 px-4 py-2">Destination</th>
                <th className="border border-gray-300 px-4 py-2">Date</th>
                <th className="border border-gray-300 px-4 py-2">Time</th>
                <th className="border border-gray-300 px-4 py-2">Price</th>
                <th className="border border-gray-300 px-4 py-2">Tickets Left</th>
              </tr>
            </thead>
            <tbody>
              {filteredAndSortedTrips.map((trip: any) => (
                <tr
                  key={trip.id}
                  className={`cursor-pointer ${selectedTrip?.id === trip.id ? 'bg-blue-100' : ''}`} // Highlight selected trip
                  onClick={() => handleSelectTrip(trip)} // Pass the whole trip object
                >
                  <td className="border border-gray-300 px-4 py-2">{String(trip.id)}</td>
                  <td className="border border-gray-300 px-4 py-2">{String(trip.source)}</td>
                  <td className="border border-gray-300 px-4 py-2">{String(trip.destination)}</td>
                  <td className="border border-gray-300 px-4 py-2">{String(trip.date)}</td>
                  <td className="border border-gray-300 px-4 py-2">{String(trip.time)}</td>
                  <td className="border border-gray-300 px-4 py-2">{String(trip.price / 10 ** 16)} EGLD</td>
                  <td className="border border-gray-300 px-4 py-2">{String(trip.ticket_count)}</td>
                </tr>
              ))}
            </tbody>
          </table>
        ) : (
          <p>No trips available.</p>
        )}
      </div>
    </div>
  );
};
