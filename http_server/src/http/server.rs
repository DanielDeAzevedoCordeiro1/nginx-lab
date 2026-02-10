use axum::extract::State;
use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;

#[derive(Clone)]
pub struct ServerState {
    pub instance_id: String,
    pub instance_name: String
}

impl ServerState {
    fn new(instance_name: String) -> Self {
        Self { instance_id: uuid::Uuid::new_v4().to_string(), instance_name }
    }
}

#[tokio::main]
pub async fn start_server() -> Result<(), Box<dyn std::error::Error>>{
    let instance_name = std::env::var("INSTANCE_NAME")
        .unwrap();
    let server_state_id = ServerState::new(instance_name);

    let router = Router::new()
        .route("/teste", get(handler))
        .with_state(server_state_id.clone());

    axum::serve(
        TcpListener::bind("0.0.0.0:3000").await?,
        router
    ).await?;

    Ok(())
}

async fn handler(State(server): State<ServerState>) -> String {
    format!("Response from Instance {}: \r\n ID: {}",
                 server.instance_name, server.instance_id)
}