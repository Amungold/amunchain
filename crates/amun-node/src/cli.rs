pub fn config_path() -> String {
    let args: Vec<String> = std::env::args().collect();

    let mut i = 1;
    while i < args.len() {
        if args[i] == "--config" {
            if i + 1 >= args.len() {
                panic!("Missing value after --config");
            }
            return args[i + 1].clone();
        }
        i += 1;
    }

    if args.len() > 1 && !args[1].starts_with("--") {
        return args[1].clone();
    }

    "crates/amun-node/data/config.toml".to_string()
}
