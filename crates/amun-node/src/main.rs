mod bootstrap;
mod certificate_loader;
mod cli;
mod cluster_builder;
mod config;
mod error;
mod genesis;
mod identity;
mod network;
mod peer_handshake;
mod peer_registry;
mod runtime;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .try_init()
        .ok();
    let config_path = cli::config_path();
    let ctx = bootstrap::initialize(&config_path).unwrap_or_else(|e| {
        eprintln!("Fatal: {e}");
        std::process::exit(1);
    });
    if ctx.config.validator.is_some() {
        let validator_cfg = cluster_builder::ClusterBuilder::build(&ctx).unwrap_or_else(|e| {
            eprintln!("Fatal: Failed to build LiveValidator configuration: {e}");
            std::process::exit(1);
        });

        let validator = amun_live_cluster::validator::LiveValidator::new(validator_cfg)
            .unwrap_or_else(|e| {
                eprintln!("Fatal: Failed to create LiveValidator: {e}");
                std::process::exit(1);
            });

        validator.start().unwrap_or_else(|e| {
            eprintln!("Fatal: Failed to start LiveValidator: {e}");
            std::process::exit(1);
        });

        loop {
            std::thread::park();
        }
    } else {
        runtime::run(
            ctx.transport,
            ctx.node,
            ctx.cert,
            ctx.genesis_hash,
            ctx.peer_id_bytes,
            ctx.config,
        );
    }
}
