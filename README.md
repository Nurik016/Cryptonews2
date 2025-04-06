# 📰 CryptoNews

A simple cryptocurrency news aggregator built in **Rust**, using **CoinMarketCap API** for top-10 cryptocurrencies and **GNews API** for related news.

## 📦 Features

- 🔝 Fetch top 10 cryptocurrencies from CoinMarketCap
- 🗞️ Retrieve latest news articles for each crypto from GNews
- 💾 SQLite caching of API responses (with expiration time)
- 🌐 Simple frontend in HTML/CSS/JS
- 🧠 Built using `reqwest`, `serde`, `tokio`, `rusqlite`, and `chrono`

---

## 🛠️ How It Works

1. On startup, the app fetches top 10 cryptocurrencies from CoinMarketCap.
2. For each cryptocurrency, it looks for related news using GNews.
3. All responses are cached in a local SQLite DB (`cache.db`) for performance:
   - Cached data expires after a configurable time (default: 10 minutes).
4. The frontend displays cryptocurrencies and their related news in a simple UI.

---

## 🚀 Getting Started

### 1. Clone the repo

```bash
git clone https://github.com/Nurik016/CryptoNews1.git
cd CryptoNews1\backend
```

### 2. Add your API keys

Create a `.env` file in the project root:

```env
COINMARKETCAP_API_KEY=your_coinmarketcap_api_key
GNEWS_API_KEY=your_gnews_api_key
```

### 3. Add Dependencies

```bash
[dependencies]
actix-web = "4"
actix-cors = "0.7"
actix-files = "0.6"
reqwest = { version = "0.11", features = ["json"] }  
serde = { version = "1.0", features = ["derive"] }   
serde_json = "1.0"                                   
tokio = { version = "1", features = ["full"] }       
rusqlite = { version = "0.30", features = ["bundled"] }
chrono = "0.4"
dotenv = "0.15"
```


### 4. Build and run

```bash
cargo build
cargo run
```

Server will start (typically at `http://127.0.0.1:8080`).

---

## 🧪 Project Structure

```bash
CryptoNews1/  
│── backend/                 
│   │── src/  
│   │   │── main.rs          
│   │   │── api.rs          
│   │   │── handlers.rs        
│   │   │── db.rs
│   │── static/  
│   │   │── index.html
│   │   │── script.js
│   │   │── styles.css
│   │── Cargo.toml           
│  │  
│── .env                
│── README.md                
│── LICENSE 
│── .gitignore             
```

---

## 🧠 Technologies Used

- [Rust](https://www.rust-lang.org/)
- [Actix Web](https://github.com/actix/actix-web/) — web server and routing
- [Actix Files](https://docs.rs/actix-files/) — static HTML/JS/CSS distribution
- [Actix CORS](https://docs.rs/actix-cors/) — CORS configuration
- [Tokio](https://tokio.rs/) — asynchronous runtime
- [Reqwest](https://docs.rs/reqwest/) — HTTP requests to API
- [Serde / Serde JSON](https://serde.rs/) — JSON serialization/deserialization
- [Rusqlite](https://docs.rs/rusqlite/) — SQLite for caching
- [Chrono](https://docs.rs/chrono/) — time management
- [dotenv](https://docs.rs/dotenv/) — API key loading from `.env`


---

## 📸 Preview

![alt text](image.png)
![alt text](image-1.png)
![alt text](image-2.png)

---

## 📃 License

MIT License. See [LICENSE](LICENSE) for details.
