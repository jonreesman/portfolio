#[allow(dead_code)]
use std::io::Read;
use std::sync::Arc;
use std::fs;
use once_cell::sync::Lazy;
use regex::Regex;
use axum::{
    routing::get,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    Router,
    extract::{Path, State},
};
use std::fs::File;
use tokio::sync::Mutex;
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use askama::Template;
use inflector::Inflector;

struct AppState {
    todos: Mutex<Vec<String>>,
    note_routes: Vec<Route>
}

#[derive(Clone)]
pub struct Route {
    name: String,
    route: String,
    shorthand: String
}

impl Route {
    fn new(name: &str, route: &str, shorthand: Option<&str>) -> Self {
        let mut shorthand_route = "";
        if shorthand.is_some() {
            shorthand_route = shorthand.unwrap();
        }
        Route {
            name: name.to_string(),
            route: route.to_string(),
            shorthand: shorthand_route.to_string(),
        }
    }
}

pub fn get_routes() -> [Route; 3] {
    [
        Route::new("About", "/about", None),
        Route::new("Notes", "/notes", None),
        Route::new("Code", "/code", None),
    ]
}


#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "with_axum_htmx_askama=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    info!("initializing router...");

    let app_state = Arc::new(AppState {
        todos: Mutex::new(vec![]),
        note_routes: get_notes()
    });



    let assets_path = std::env::current_dir().unwrap();

    let api_router = Router::new()
        .route("/hello", get(hello_from_the_server));

    let app = Router::new()
        .nest("/api", api_router)
        .route("/", get(hello))
        .route("/notes", get(notes))
        .route("/notes/:id", get(notes_md))
        .with_state(app_state)
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> impl IntoResponse {
    let template = HelloTemplate {};
    HtmlTemplate(template)
}

async fn notes(State(state): State<Arc<AppState>>) -> impl IntoResponse {

    let template = NotesTemplate {
        notes: state.note_routes.clone(),
    };
    HtmlTemplate(template)
}


async fn notes_md(Path(id): Path<String>) -> impl IntoResponse {
    let template = MarkdownTemplate { safe_html: get_markdown(id)};
    HtmlTemplate(template)
}

async fn hello_from_the_server() -> &'static str {
    "Hello!"
}


#[derive(Template)]
#[template(source = "<div>{{ safe_html }}</div>", ext = "txt")]
struct MarkdownTemplate {
    safe_html: String,
}


#[derive(Template)]
#[template(path = "another-page.html")]
struct AnotherPageTemplate;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate;

#[derive(Template)]
#[template(path = "notes.html")]
struct NotesTemplate {
    notes: Vec<Route>
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

fn read_file(filename: String) -> String {
    let assets_path = std::env::current_dir().unwrap();
    let mut file = File::open(format!("{}/assets/markdown/{}", assets_path.to_str().unwrap(), filename)).unwrap();
    println!("{}", format!("{}/assets/{}", assets_path.to_str().unwrap(), filename));
    let mut buffer = String::new();
    file.read_to_string(&mut buffer);

    buffer
}

fn get_markdown(filepath: String) -> String {
    let file = read_file(filepath);
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\((.+)\)\^(.+)").unwrap());
    // Latex *'s dont jive well with pulldown_cmark since it
    // just assumes our converted mathml is markdown, making
    // our *'s just italics operators
    let cleaned_file = file.lines().map(|line| {
        let mut new_line = line.to_owned();
        if line.contains("$$") {
            if line.contains("*") {
                let first_dollar_sign = line.find("$").unwrap_or(line.len());
                let updated_line: String = line.split("").enumerate().map(|(idx, char)| {
                    if idx < first_dollar_sign {
                        return char;
                    }
                    if char == "*" {
                        return "\\cdot";
                    }
                    return char;
                }).collect::<Vec<&str>>().join("");
                
                new_line = updated_line;
            }
            if RE.is_match(&new_line) {
                new_line = new_line.replace("(", " \\left( ");
                new_line = new_line.replace(")", " \\right) ");
            }
        }
        new_line
    }).collect::<Vec<String>>().join("\n");
    let mathml = latex2mathml::replace(&cleaned_file).unwrap();
    let parser = pulldown_cmark::Parser::new(&mathml);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output
}

fn get_notes() -> Vec<Route> {
    let paths = fs::read_dir("./assets/markdown").unwrap();

    let mut routes = vec![];
    for path in paths {
        let route_str = path.unwrap().path().display().to_string().to_owned();
        if route_str.contains(".md") {
            let name_with_filetype = route_str.split("/").last().unwrap();
            let name_without_filetype = name_with_filetype.replace(".md", "");
            println!("{}", name_with_filetype);
            let name = name_with_filetype.replace("-", " ").replace(".md", "").to_title_case();
            routes.push(
                Route::new(&name, &format!("/notes/{}", name_with_filetype), Some(&format!("/notes/{}", name_without_filetype)))
            );
        }
    }
    routes
}
