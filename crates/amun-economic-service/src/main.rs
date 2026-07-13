use amun_tokenomics::EpochEconomics;
use amun_tokenomics_ledger::EconomicLedger;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn fetch_height() -> Option<u64> {
    let resp = ureq::get("http://127.0.0.1:9070/status").call().ok()?;
    let body = resp.into_string().ok()?;
    let json: serde_json::Value = serde_json::from_str(&body).ok()?;
    json["height"].as_u64()
}

fn main() {
    let ledger = Arc::new(Mutex::new(EconomicLedger::new()));
    let ledger_clone = ledger.clone();

    thread::spawn(move || {
        let mut last_height = 0u64;
        loop {
            thread::sleep(Duration::from_secs(12));
            if let Some(current_height) = fetch_height() {
                let blocks_missed = current_height.saturating_sub(last_height);
                let mut economics = EpochEconomics::new();
                economics.distribute_epoch(8_000_000_000);
                for _ in 0..blocks_missed {
                    ledger_clone.lock().unwrap().on_block_finalized(&economics);
                }
                last_height = current_height;
            }
        }
    });

    let server = tiny_http::Server::http("0.0.0.0:9074").expect("Failed to bind port 9074");
    println!("Economic Observer API on http://0.0.0.0:9074/tokenomics");

    for request in server.incoming_requests() {
        if request.url() == "/tokenomics" {
            let ledger = ledger.lock().unwrap();
            let json = serde_json::json!({
                "treasury_balance": ledger.treasury_balance,
                "total_issued_ntr": ledger.total_issued_ntr,
                "current_epoch": ledger.current_epoch,
                "blocks_in_current_epoch": ledger.blocks_in_current_epoch,
                "last_epoch_reward": ledger.last_epoch_reward,
                "validator_reward_pool": ledger.validator_reward_pool,
                "ecosystem_reward_pool": ledger.ecosystem_reward_pool,
                "ledger_root": hex::encode(ledger.compute_ledger_root()),
                "status": "ok"
            })
            .to_string();
            let response = tiny_http::Response::from_string(json);
            let header =
                tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..])
                    .unwrap();
            let response = response.with_header(header);
            let _ = request.respond(response);
        }
    }
}
