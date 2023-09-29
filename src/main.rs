use anyhow::Context;
use askama::Template;
use axum::{debug_handler, Json};
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

struct AppState {
    todos: Mutex<Vec<String>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "dream_stack=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    info!("initializing router...");

    let app_state = Arc::new(AppState {
        todos: Mutex::new(vec![]),
    });

    let assets_path = std::env::current_dir().unwrap();

    let api_router = Router::new()
        .route("/hello", get(hello_from_srv))
        .route("/recipient_input", get(recipient_input))
        .route("/todos", post(add_todo));

    let router = Router::new()
        .nest("/api", api_router)
        .route("/", get(hello))
        .route("/recipients", get(recipient_form))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        )
        .with_state(app_state);
    let port = 8000_u16;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));

    info!("router initialized, now listening on port {port}");

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .context("error while starting server")?;

    Ok(())
}

async fn hello_from_srv() -> String {
    "Hello!".to_string()
}

async fn hello() -> impl IntoResponse {
    let template = HelloTemplate { recipients: vec![] };
    HtmlTemplate(template)
}

async fn recipient_input() -> impl IntoResponse {
    HtmlTemplate(RecipientInput {})
}

#[derive(Deserialize)]
struct Recipient {
    name: String,
    email: String,
}

#[derive(Template)]
#[template(path = "recipient_input.html")]
struct RecipientInput;

async fn recipient_form() -> impl IntoResponse {
    HtmlTemplate(RecipientForm {})
}

#[derive(Template)]
#[template(path = "recipient-form.html")]
struct RecipientForm;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    recipients: Vec<Recipient>,
}

#[derive(Template)]
#[template(path = "todo-list.html")]
struct TodoList {
    todos: Vec<String>,
    recipients: Vec<Recipient>,
}

#[derive(Deserialize)]
pub struct TodoRequest {
    pub todo: String,
    name: Vec<String>,
    email: Vec<String>,
}

impl TodoRequest {
    fn get_recipients(&self) -> Vec<Recipient> {
        self.name
            .iter()
            .zip(self.email.iter())
            .map(|(name, email)| Recipient {
                name: name.to_string(),
                email: email.to_string(),
            })
            .collect()
    }
}

#[debug_handler]
async fn add_todo(
    State(state): State<Arc<AppState>>,
    Json(todo): Json<TodoRequest>,
) -> impl IntoResponse {
    let mut lock = state.todos.lock().unwrap();
    lock.push(todo.todo.clone());

    let template = TodoList {
        todos: lock.clone(),
        recipients: todo.get_recipients(),
    };

    HtmlTemplate(template)
}

/// A wrapper type that we'll use to encapsulate HTML parsed by askama into valid HTML for axum to serve.
struct HtmlTemplate<T>(T);

/// Allows us to convert Askama HTML templates into valid HTML for axum to serve in the response.
impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        // Attempt to render the template with askama
        match self.0.render() {
            // If we're able to successfully parse and aggregate the template, serve it
            Ok(html) => Html(html).into_response(),
            // If we're not, return an error or some bit of fallback HTML
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
