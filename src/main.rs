use entity::todo;
use entity::todo::Entity as Todo;

use anyhow::Context;
use askama::Template;
use axum::debug_handler;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Form, Router,
};
use maud::{html, Markup};
use sea_orm::{ActiveModelTrait, ActiveValue, Database, DatabaseConnection, EntityTrait, Set};
use serde::Deserialize;
use std::env;
use std::sync::Arc;
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

struct AppState {
    db: DatabaseConnection,
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
    dotenvy::dotenv()?;

    let db: DatabaseConnection =
        Database::connect(env::var("DATABASE_URL").context("DATABASE_URL not found")?).await?;

    let app_state = Arc::new(AppState { db });

    let assets_path = std::env::current_dir().unwrap();

    let api_router = Router::new()
        .route("/hello", get(hello_from_srv))
        .route("/todos", post(add_todo));

    let router = Router::new()
        .nest("/api", api_router)
        .route("/", get(home))
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

async fn hello_from_srv() -> Markup {
    let resp = "great".to_string();
    html!(
        h1 { (resp) }
    )
}

async fn home(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let todos: Vec<todo::Model> = Todo::find().all(&state.db).await.unwrap();
    HtmlTemplate(HomeTemplate { todos })
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    todos: Vec<todo::Model>,
}

#[derive(Template)]
#[template(path = "todo-list.html")]
struct TodoListTemplate {
    todos: Vec<todo::Model>,
}

#[debug_handler]
async fn add_todo(
    State(state): State<Arc<AppState>>,
    Form(todo): Form<todo::Model>,
) -> impl IntoResponse {
    let mut todo: todo::ActiveModel = todo.into();
    // todo: figure out if there is a better way to let the pk be auto-gen
    todo.id = ActiveValue::NotSet;
    todo.insert(&state.db).await.unwrap();

    let todos: Vec<todo::Model> = Todo::find().all(&state.db).await.unwrap();
    HtmlTemplate(TodoListTemplate { todos })
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
