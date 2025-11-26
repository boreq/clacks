use clacks_backend::adapters::{ConfigLoader, Metrics, PubSub};
use clacks_backend::app::add_message_to_queue::AddMessageToQueueHandler;
use clacks_backend::app::get_config::GetConfigHandler;
use clacks_backend::app::get_state::GetStateHandler;
use clacks_backend::app::update_clacks::UpdateClacksHandler;
use clacks_backend::config::Config;
use clacks_backend::domain::servos;
use clacks_backend::errors::Result;
use clacks_backend::ports::http;
use clacks_backend::ports::http::EventSubscriber;
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
    let pubsub = PubSub::new();

    #[cfg(not(feature = "raspberry_pi"))]
    let servo_controller = adapters::MockServoController::new();

    #[cfg(feature = "raspberry_pi")]
    let servo_controller = adapters::raspberrypi::ServoController::new()?;

    let shutters_controller = servos::ShuttersController::new(servo_controller);

    let queue = domain::Queue::new(config.queue_size())?;
    let encoding = domain::Encoding::default();

    let messages_to_inject = config
        .messages_to_inject()
        .iter()
        .map(|v| encoding.encode(v))
        .collect::<Result<Vec<_>>>()?;
    let messages_to_inject = domain::MessagesToInject::new(messages_to_inject);

    let clacks = domain::Clacks::new(config.timing().clone(), queue.clone(), messages_to_inject);

    let update_clacks_handler = UpdateClacksHandler::new(
        clacks.clone(),
        metrics.clone(),
        pubsub.clone(),
        shutters_controller,
    );
    let add_message_to_queue_handler = AddMessageToQueueHandler::new(
        queue.clone(),
        metrics.clone(),
        encoding.clone(),
        pubsub.clone(),
    );
    let get_state_handler = GetStateHandler::new(clacks.clone(), queue.clone(), metrics.clone());
    let get_config_handler = GetConfigHandler::new(encoding.clone(), metrics.clone());

    let mut timer = timers::UpdateClacksTimer::new(update_clacks_handler);
    let server = http::Server::new();

    tokio::spawn({
        async move {
            timer.run().await;
        }
    });

    let http_deps = HttpDeps::new(
        get_state_handler,
        add_message_to_queue_handler,
        get_config_handler,
        metrics,
        pubsub,
    );

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
struct HttpDeps<GSH, AMTQH, GCH> {
    get_state_handler: GSH,
    add_message_to_queue_handler: AMTQH,
    get_config_handler: GCH,
    metrics: adapters::Metrics,
    pubsub: adapters::PubSub,
}

impl<GSH, AMTQH, GCH> HttpDeps<GSH, AMTQH, GCH> {
    pub fn new(
        get_state_handler: GSH,
        add_message_to_queue_handler: AMTQH,
        get_config_handler: GCH,
        metrics: adapters::Metrics,
        pubsub: PubSub,
    ) -> Self {
        Self {
            get_state_handler,
            add_message_to_queue_handler,
            get_config_handler,
            metrics,
            pubsub,
        }
    }
}

impl<GSH, AMTQH, GCH> http::Deps for HttpDeps<GSH, AMTQH, GCH>
where
    GSH: app::GetStateHandler,
    AMTQH: app::AddMessageToQueueHandler,
    GCH: app::GetConfigHandler,
{
    fn get_state_handler(&self) -> &impl app::GetStateHandler {
        &self.get_state_handler
    }

    fn add_message_to_queue_handler(&self) -> &impl app::AddMessageToQueueHandler {
        &self.add_message_to_queue_handler
    }

    fn get_config_handler(&self) -> &impl app::GetConfigHandler {
        &self.get_config_handler
    }

    fn metrics(&self) -> &Registry {
        self.metrics.registry()
    }

    fn subscriber(&self) -> &impl EventSubscriber {
        &self.pubsub
    }
}
