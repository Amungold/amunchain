use amun_persistent_node::persistent_store::PersistentValidatorStore;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: dump_state_root <data_directory>");
        std::process::exit(1);
    }
    let path = Path::new(&args[1]);
    let store =
        PersistentValidatorStore::open(path.to_str().unwrap()).expect("Failed to open store");
    let root = store.state_root();
    println!("{}", hex::encode(root));
}
