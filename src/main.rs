use entity::todo::Entity as Todo;
use entity::todo::{self, ActiveModel};

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
use maud::{html, Markup};
use sea_orm::prelude::DateTimeUtc;
use sea_orm::{ActiveModelTrait, ActiveValue, Database, DatabaseConnection, EntityTrait, Set};
use std::env;
use std::sync::Arc;
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

struct AppState {
    db: DatabaseConnection,
    navbar: Vec<Navigation>,
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

    let home_nav = Navigation {
        title: "Home".to_string(),
        href: "/".to_string(),
    };
    let settings = Navigation {
        title: "Settings".to_string(),
        href: "/".to_string(),
    };
    let tw_forms_ex = Navigation {
        title: "Tailwind Forms Example".to_string(),
        href: "/docs/tailwind/forms/examples".to_string(),
    };
    let tw_forms_sink = Navigation {
        title: "Tailwind Forms Sink".to_string(),
        href: "/docs/tailwind/forms/sink".to_string(),
    };
    let tw_buttons = Navigation {
        title: "Tailwind Buttons".to_string(),
        href: "/docs/tailwind/buttons".to_string(),
    };
    let navbar: Vec<Navigation> = vec![home_nav, settings, tw_forms_ex, tw_forms_sink, tw_buttons];

    let app_state = Arc::new(AppState { db, navbar });

    let assets_path = std::env::current_dir().unwrap();

    let api_router = Router::new()
        .route("/hello", get(inline_html))
        .route("/todos", post(add_todo))
        .route("/todos/new", post(todo_form));

    let docs_router = Router::new()
        .route("/tailwind/forms/examples", get(tailwind_forms_examples))
        .route("/tailwind/forms/sink", get(tailwind_forms_sink))
        .route("/tailwind/buttons", get(tailwind_buttons));

    let router = Router::new()
        .nest("/api", api_router)
        .nest("/docs/", docs_router)
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

async fn inline_html() -> Markup {
    let resp = "great".to_string();
    html!(
        h1 { (resp) }
    )
}

#[debug_handler]
async fn tailwind_buttons(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    HtmlTemplate(TailwindButtons {
        navbar: state.navbar.clone(),
    })
}

#[debug_handler]
async fn tailwind_forms_sink(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    HtmlTemplate(TailwindFormsSink {
        navbar: state.navbar.clone(),
    })
}

#[debug_handler]
async fn tailwind_forms_examples(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    HtmlTemplate(TailwindFormsExample {
        navbar: state.navbar.clone(),
    })
}

#[debug_handler]
async fn home(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let todos: Vec<todo::Model> = Todo::find().all(&state.db).await.unwrap();
    let todo = todo::Model {
        id: Uuid::new_v4().to_string(),
        text: "some".to_string(),
        title: "other".to_string(),
        due_date: DateTimeUtc::default().to_string(),
    };

    let navbar = state.navbar.clone();

    HtmlTemplate(HomeTemplate {
        todos,
        todo,
        navbar,
    })
}

#[derive(Clone, Debug)]
pub struct Navigation {
    pub title: String,
    pub href: String,
}

#[derive(Template)]
#[template(path = "tailwind_buttons.html")]
struct TailwindButtons {
    navbar: Vec<Navigation>,
}

#[derive(Template)]
#[template(path = "tailwind_forms_examples.html")]
struct TailwindFormsExample {
    navbar: Vec<Navigation>,
}

#[derive(Template)]
#[template(path = "tailwind_forms_sink.html")]
struct TailwindFormsSink {
    navbar: Vec<Navigation>,
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    todos: Vec<todo::Model>,
    todo: todo::Model,
    navbar: Vec<Navigation>,
}

#[debug_handler]
async fn todo_form(
    State(state): State<Arc<AppState>>,
    Json(todo): Json<todo::Model>,
) -> impl IntoResponse {
    info!("todo: {:?}", todo);
    let todo: todo::ActiveModel = todo.into();
    info!("todo: {:?}", todo);
    todo.insert(&state.db).await.unwrap();

    let todos: Vec<todo::Model> = Todo::find().all(&state.db).await.unwrap();
    let todo = todo::Model {
        id: Uuid::new_v4().to_string(),
        text: "some".to_string(),
        title: "other".to_string(),
        due_date: DateTimeUtc::default().to_string(),
    };
    HtmlTemplate(TodoFormTemplate { todos, todo })
}

#[derive(Template)]
#[template(path = "todo-form.html")]
struct TodoFormTemplate {
    todos: Vec<todo::Model>,
    todo: todo::Model,
}

#[derive(Template)]
#[template(path = "todo-list.html")]
struct TodoListTemplate {
    todos: Vec<todo::Model>,
}

#[debug_handler]
async fn add_todo(
    State(state): State<Arc<AppState>>,
    Json(todo): Json<todo::Model>,
) -> impl IntoResponse {
    let todo: todo::ActiveModel = todo.into();
    info!("todo: {:?}", todo);
    todo.insert(&state.db).await.unwrap();

    let todos: Vec<todo::Model> = Todo::find().all(&state.db).await.unwrap();
    let todo = todo::Model {
        id: Uuid::new_v4().to_string(),
        text: "some".to_string(),
        title: "other".to_string(),
        due_date: DateTimeUtc::default().to_string(),
    };
    HtmlTemplate(TodoFormTemplate { todos, todo })
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
