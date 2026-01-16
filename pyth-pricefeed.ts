const JUPITER_PRICE_URL =
  "https://api.jup.ag/price/v3?ids=So11111111111111111111111111111111111111112";

const SOL_MINT = "So11111111111111111111111111111111111111112";

async function getSolPrice() {
  try {
    const res = await fetch(JUPITER_PRICE_URL, {
      headers: {
        "accept": "application/json",
       
       "x-api-key": process.env.JUP_API_KEY ?? "c208a0a1-7b2f-4415-9641-d26a474012c2",
      },
    });

    if (!res.ok) {
      throw new Error(`HTTP ${res.status}`);
    }

    const data = await res.json();

    const sol = data[SOL_MINT];
    if (!sol) {
      console.log("SOL price not available");
      return;
    }

    const {
      usdPrice,
      decimals,
      blockId,
      priceChange24h,
    } = sol;

    const formattedPrice = usdPrice.toFixed(decimals);

    console.log({
  //    priceUSD: formattedPrice,
      rawPrice: usdPrice,
 //     decimals,
//      blockId,
  //    priceChange24h,
      timestamp: new Date().toISOString(),
    });

  } catch (err) {
    console.error("Jupiter price error:", err);
  }
}

setInterval(getSolPrice, 1000);
