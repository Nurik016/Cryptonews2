async function getCryptoData() {
    const symbol = document.getElementById('symbol').value.trim();
    if (!symbol) return;

    document.getElementById('crypto-info').innerHTML = "Loading...";
    document.getElementById('news-list').innerHTML = "";

    try {
        const cryptoRes = await fetch(`/crypto/${symbol}`);
        const newsRes = await fetch(`/news/${symbol}`);

        if (!cryptoRes.ok) throw new Error("Crypto data fetch failed");
        const crypto = await cryptoRes.json();

        const infoHTML = `
            <h2>${crypto.name} (${crypto.symbol})</h2>
            <p>💰 Price: $${crypto.price.toFixed(2)}</p>
            <p>📈 Market Cap: $${(crypto.market_cap / 1e9).toFixed(2)}B</p>
            <p>🔁 24h Volume: $${(crypto.volume_24h / 1e9).toFixed(2)}B</p>
        `;
        document.getElementById('crypto-info').innerHTML = infoHTML;

        if (newsRes.ok) {
            const news = await newsRes.json();
            const newsHTML = news.map(article => `
                <div class="news-item">
                    <h4><a href="${article.url}" target="_blank">${article.title}</a></h4>
                    <p>${article.description}</p>
                    <small>📰 ${article.source} | 🕒 ${new Date(article.published_at).toLocaleString()}</small>
                </div>
            `).join('');
            document.getElementById('news-list').innerHTML = `<h3>Latest News</h3>` + newsHTML;
        }

    } catch (error) {
        document.getElementById('crypto-info').innerHTML = "Error loading data.";
    }
}
