use amun_chain_store::store::ChainStore;
use std::env;

fn main() {
    let dir = env::args().nth(1).expect("need data dir");
    let store = ChainStore::open(&dir).expect("open failed");
    let tip = store.latest_height();
    let mut missing = 0u64;
    let mut first = 0u64;
    let mut last = 0u64;
    for h in 1..=tip {
        if store.load_height(h).is_none() {
            if first == 0 {
                first = h;
            }
            last = h;
            missing += 1;
        }
    }
    println!("{} {} {} {} {}", store.len(), tip, missing, first, last);
}
