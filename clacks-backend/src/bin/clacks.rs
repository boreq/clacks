use clacks_backend::adapters::{ConfigLoader, Metrics};
use clacks_backend::app::add_message_to_queue::AddMessageToQueueHandler;
use clacks_backend::app::update_clacks::UpdateClacksHandler;
use clacks_backend::config::Config;
use clacks_backend::errors::Result;
use clacks_backend::ports::http;
use clacks_backend::ports::timers;
use clacks_backend::{adapters, app, domain};
use clap::{Command, arg};
use env_logger::Env;
use log::error;
use prometheus::Registry;

fn cli() -> Command {
    Command::new("clacks")
        .about("Software which controls clacks.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("run")
                .about("Runs the program")
                .arg(arg!(<CONFIG> "Path to the configuration file")),
        )
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().filter_or("RUST_LOG", "info")).init();

    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("run", sub_matches)) => {
            let config_file_path = sub_matches.try_get_one::<String>("CONFIG")?.unwrap();
            run(config_file_path).await?;
        }
        _ => unreachable!(),
    }

    Ok(())
}

async fn run(config_file_path: &str) -> Result<()> {
    let config_loader = ConfigLoader::new(config_file_path);
    let config = config_loader.load()?;

    let metrics = Metrics::new()?;

    let queue = domain::Queue::new(config.queue_size())?;
    let clacks = domain::Clacks::new(config.timing().clone(), queue.clone());
    let encoding = domain::Encoding::default();

    let update_clacks_handler = UpdateClacksHandler::new(clacks, metrics.clone());
    let add_message_to_queue_handler =
        AddMessageToQueueHandler::new(queue, metrics.clone(), encoding);

    let mut timer = timers::UpdateClacksTimer::new(update_clacks_handler);
    let server = http::Server::new();

    tokio::spawn({
        async move {
            timer.run().await;
        }
    });

    let http_deps = HttpDeps::new(add_message_to_queue_handler, metrics);

    server_loop(&server, &config, http_deps).await;
    Ok(())
}

async fn server_loop<D>(server: &http::Server, config: &Config, deps: D)
where
    D: http::Deps + Sync + Send + Clone + 'static,
{
    loop {
        match server.run(config, deps.clone()).await {
            Ok(_) => {
                error!("the server exited without returning any errors")
            }
            Err(err) => {
                error!("the server exited with an error: {err}")
            }
        }
    }
}

#[derive(Clone)]
struct HttpDeps<AMTQH> {
    add_message_to_queue_handler: AMTQH,
    metrics: adapters::Metrics,
}

impl<AMTQH> HttpDeps<AMTQH> {
    pub fn new(add_message_to_queue_handler: AMTQH, metrics: adapters::Metrics) -> Self {
        Self {
            add_message_to_queue_handler,
            metrics,
        }
    }
}

impl<AMTQH> http::Deps for HttpDeps<AMTQH>
where
    AMTQH: app::AddMessageToQueueHandler,
{
    fn add_message_to_queue_handler(&self) -> &impl app::AddMessageToQueueHandler {
        &self.add_message_to_queue_handler
    }

    fn metrics(&self) -> &Registry {
        self.metrics.registry()
    }
}
