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
