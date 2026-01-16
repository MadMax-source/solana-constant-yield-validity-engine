import PythPriceService from "@pythnetwork/price-service-client";

const { PriceServiceConnection } = PythPriceService;

const connection = new PriceServiceConnection(
  "https://hermes.pyth.network" 
);

const SOL_PRICE_ID =
  "ef0d8b6fda2ceba41da15d4095d1da392c1d6edda6c2fdc2b5b6c4b7e1e2e5b2";


async function getSolPrice() {
  try {
    const priceFeeds = await connection.getLatestPriceFeeds([SOL_PRICE_ID]);

    if (!priceFeeds || priceFeeds.length === 0) {
      console.log("No price data available");
      return;
    }

    const priceFeed = priceFeeds[0];
    const price = priceFeed.getPriceNoOlderThan(60);

    if (!price) {
      console.log("Price too old");
      return;
    }

    const solPrice = price.price * Math.pow(10, price.expo);
    console.log(`SOL/USD Price: $${solPrice}`);
  } catch (err) {
    console.error("Error fetching SOL price:", err);
  }
}

setInterval(getSolPrice, 1000);







/*


import { Connection, PublicKey } from '@solana/web3.js';
import { parsePriceData, PriceStatus } from '@pythnetwork/client';

const connection = new Connection('https://api.mainnet-beta.solana.com');

const SOL_USD_PRICE_ACCOUNT = new PublicKey(
  'J83GJ8v8XLNNF5CVxY66vZcU2f4JkqH5XoM2jA2qZ5uR' ,
);

async function getSolPrice() {
  try {
    const accountInfo = await connection.getAccountInfo(SOL_USD_PRICE_ACCOUNT);
    if (!accountInfo) {
      console.log('Price account not found!');
      return;
    }

    const priceData = parsePriceData(accountInfo.data);
    if (priceData.status !== PriceStatus.Trading) {
      console.log('SOL/USD price not ready yet.');
      return;
    }

    const solPrice = priceData.price;
    console.log(`Current SOL/USD Price: $${solPrice}`);
  } catch (err) {
    console.error('Error fetching SOL price:', err);
  }
}

setInterval(getSolPrice, 1000);


*/