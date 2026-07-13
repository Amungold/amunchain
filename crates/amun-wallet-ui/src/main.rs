use axum::http::Method;
use axum::{response::Html, routing::get, Router};
use tower_http::cors::{Any, CorsLayer};

async fn wallet_page() -> Html<&'static str> {
    Html(
        r#"<!DOCTYPE html>
<html lang="ar">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>AmunChain Wallet</title>
<style>
body { font-family: monospace; background: #0a0a1a; color: #00ff88; padding: 20px; }
input, button { padding: 10px; margin: 5px; border-radius: 5px; border: 1px solid #00ff88; background: #111; color: #00ff88; }
button { cursor: pointer; background: #005522; }
button:hover { background: #007733; }
.card { border: 1px solid #00ff88; padding: 15px; margin: 10px 0; border-radius: 8px; }
</style>
</head>
<body>
<h1>AmunChain Wallet</h1>

<div class="card">
  <h3>Create Wallet</h3>
  <button onclick="createWallet()">Generate New Address</button>
  <p id="address"></p>
</div>

<div class="card">
  <h3>Faucet - Get Test Tokens</h3>
  <input id="faucetAddr" placeholder="Your wallet address" size="66">
  <button onclick="requestFaucet()">Request 100,000 Tokens</button>
  <p id="faucetResult"></p>
</div>

<div class="card">
  <h3>Check Balance</h3>
  <input id="balanceAddr" placeholder="Wallet address" size="66">
  <button onclick="checkBalance()">Check Balance</button>
  <p id="balanceResult"></p>
</div>

<div class="card">
  <h3>Send Transaction</h3>
  <input id="fromAddr" placeholder="From address" size="66"><br>
  <input id="toAddr" placeholder="To address" size="66"><br>
  <input id="amount" placeholder="Amount" type="number"><br>
  <button onclick="sendTx()">Send</button>
  <p id="txResult"></p>
</div>

<script>
const FAUCET_URL = "http://127.0.0.1:9073";
const RPC_URL = "http://127.0.0.1:9070";

function createWallet() {
  const chars = "0123456789abcdef";
  let addr = "";
  for (let i = 0; i < 64; i++) addr += chars[Math.floor(Math.random() * 16)];
  document.getElementById("address").innerText = "Address: " + addr;
  document.getElementById("faucetAddr").value = addr;
  document.getElementById("balanceAddr").value = addr;
  document.getElementById("fromAddr").value = addr;
}

async function requestFaucet() {
  const addr = document.getElementById("faucetAddr").value;
  if (!addr || addr.length !== 64) {
    document.getElementById("faucetResult").innerText = "Invalid address!";
    return;
  }
  try {
    const res = await fetch(FAUCET_URL + "/faucet/request", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ address: addr, amount: 100000 })
    });
    const data = await res.json();
    document.getElementById("faucetResult").innerText = JSON.stringify(data, null, 2);
  } catch (e) {
    document.getElementById("faucetResult").innerText = "Error: " + e.message;
  }
}

async function checkBalance() {
  const addr = document.getElementById("balanceAddr").value;
  if (!addr || addr.length !== 64) {
    document.getElementById("balanceResult").innerText = "Invalid address!";
    return;
  }
  try {
    const res = await fetch(RPC_URL + "/account/" + addr);
    const data = await res.json();
    document.getElementById("balanceResult").innerText = JSON.stringify(data, null, 2);
  } catch (e) {
    document.getElementById("balanceResult").innerText = "Error: " + e.message;
  }
}

async function sendTx() {
  const from = document.getElementById("fromAddr").value;
  const to = document.getElementById("toAddr").value;
  const amount = parseInt(document.getElementById("amount").value);
  if (!from || !to || !amount) {
    document.getElementById("txResult").innerText = "Fill all fields!";
    return;
  }
  try {
    const res = await fetch(RPC_URL + "/tx/send", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ from: "0x" + from, to: "0x" + to, value: amount })
    });
    const data = await res.json();
    document.getElementById("txResult").innerText = JSON.stringify(data, null, 2);
  } catch (e) {
    document.getElementById("txResult").innerText = "Error: " + e.message;
  }
}
</script>
</body>
</html>"#,
    )
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(wallet_page))
        .route("/wallet", get(wallet_page))
        .layer(cors);

    println!("Wallet UI listening on http://127.0.0.1:8080");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
