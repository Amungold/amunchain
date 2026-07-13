mod commands;

use clap::{Parser, Subcommand};
use commands::{deploy, health, scale, service, services, validator};

/// AmunChain Orchestrator CLI — self-managing blockchain platform.
#[derive(Parser)]
#[command(name = "amunctl")]
#[command(version = "0.1.0")]
#[command(about = "Manage your AmunChain network", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start all services (validators, RPC, Explorer, WebSocket)
    Up,
    /// Stop all services gracefully
    Down,
    /// Restart all services
    Restart,
    /// Show network status overview
    Status,
    /// Show health report with scores
    Health,
    /// Show logs for a service
    Logs {
        /// Service name (e.g., validator-1, rpc, explorer)
        service: String,
        /// Number of lines to show
        #[arg(short, long, default_value = "50")]
        lines: usize,
    },

    /// Manage validators
    #[command(subcommand)]
    Validator(ValidatorCommand),

    /// Manage services
    #[command(subcommand)]
    Service(ServiceCommand),

    /// Deploy to an environment
    #[command(subcommand)]
    Deploy(DeployCommand),

    /// Auto-scaling operations
    #[command(subcommand)]
    Scale(ScaleCommand),
}

#[derive(Subcommand)]
enum ValidatorCommand {
    /// Add a new validator
    Add {
        /// Validator name
        name: String,
        /// Listen port
        #[arg(short, long, default_value = "0")]
        port: u16,
        /// Voting power
        #[arg(long, default_value = "100")]
        power: u64,
    },
    /// Remove a validator
    /// Start a created validator
    Start {
        /// Validator name
        name: String,
    },
    Remove {
        /// Validator name
        name: String,
    },
    /// Restart a specific validator
    Restart {
        /// Validator name
        name: String,
    },
    /// List all validators
    List,
}

#[derive(Subcommand)]
enum ServiceCommand {
    /// Start RPC service
    RpcStart,
    /// Stop RPC service
    RpcStop,
    /// Start Explorer API
    ExplorerStart,
    /// Stop Explorer API
    ExplorerStop,
    /// Start WebSocket service
    WsStart,
    /// Stop WebSocket service
    WsStop,
}

#[derive(Subcommand)]
enum DeployCommand {
    /// Deploy to testnet
    Testnet,
    /// Deploy to mainnet
    Mainnet,
    /// Show deployment history
    History,
    /// Rollback last deployment
    Rollback,
}

#[derive(Subcommand)]
enum ScaleCommand {
    /// Show current scaling policy
    Policy,
    /// Scale up by N validators
    Up {
        /// Number of validators to add
        count: usize,
    },
    /// Scale down by N validators
    Down {
        /// Number of validators to remove
        count: usize,
    },
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .try_init()
        .ok();

    let cli = Cli::parse();

    match cli.command {
        Commands::Up => {
            println!("🚀 Starting AmunChain network...");
            service::start_all().await;
            validator::start_all().await;
            health::wait_for_healthy().await;
            let services = services::init_services("./data", "./target/debug");
            services::persist_state(&services);
            println!("✅ Network is ready");
        }
        Commands::Down => {
            println!("🛑 Stopping AmunChain network...");
            service::stop_all().await;
            validator::stop_all().await;
            let services = services::init_services("./data", "./target/debug");
            services::persist_state(&services);
            println!("✅ Network stopped");
        }
        Commands::Restart => {
            println!("🔄 Restarting AmunChain network...");
            service::stop_all().await;
            validator::stop_all().await;
            let services = services::init_services("./data", "./target/debug");
            services::persist_state(&services);
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            service::start_all().await;
            validator::start_all().await;
            println!("✅ Network restarted");
        }
        Commands::Status => {
            validator::list().await;
            service::status().await;
        }
        Commands::Health => {
            health::report().await;
        }
        Commands::Logs { service, lines } => {
            println!("📋 Logs for {} (last {} lines):", service, lines);
        }
        Commands::Validator(cmd) => match cmd {
            ValidatorCommand::Add { name, port, power } => {
                validator::add(&name, port, power).await;
            }
            ValidatorCommand::Remove { name } => {
                validator::remove(&name).await;
            }
            ValidatorCommand::Start { name } => {
                validator::start(&name).await;
            }
            ValidatorCommand::Restart { name } => {
                validator::restart(&name).await;
            }
            ValidatorCommand::List => {
                validator::list().await;
            }
        },
        Commands::Service(cmd) => match cmd {
            ServiceCommand::RpcStart => service::rpc_start().await,
            ServiceCommand::RpcStop => service::rpc_stop().await,
            ServiceCommand::ExplorerStart => service::explorer_start().await,
            ServiceCommand::ExplorerStop => service::explorer_stop().await,
            ServiceCommand::WsStart => service::ws_start().await,
            ServiceCommand::WsStop => service::ws_stop().await,
        },
        Commands::Deploy(cmd) => match cmd {
            DeployCommand::Testnet => deploy::testnet().await,
            DeployCommand::Mainnet => deploy::mainnet().await,
            DeployCommand::History => deploy::history().await,
            DeployCommand::Rollback => deploy::rollback().await,
        },
        Commands::Scale(cmd) => match cmd {
            ScaleCommand::Policy => scale::show_policy().await,
            ScaleCommand::Up { count } => scale::scale_up(count).await,
            ScaleCommand::Down { count } => scale::scale_down(count).await,
        },
    }
}
