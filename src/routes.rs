#[allow(dead_code)]

use std::sync::Arc;
use std::fs;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use inflector::Inflector;
use tokio::sync::Mutex;

use crate::templates;
use crate::markdown::get_markdown;
use crate::routes;


pub struct AppState {
    pub todos: Mutex<Vec<String>>,
    pub note_routes: Vec<routes::Route>
}

#[derive(Clone)]
pub struct Route {
    pub name: String,
    pub route: String,
    pub shorthand: String
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

pub async fn hello() -> impl IntoResponse {
    let template = templates::HelloTemplate {};
    templates::HtmlTemplate(template)
}

pub async fn notes(State(state): State<Arc<AppState>>) -> impl IntoResponse {

    let template = templates::NotesTemplate {
        notes: state.note_routes.clone(),
    };
    templates::HtmlTemplate(template)
}


pub async fn notes_md(Path(id): Path<String>) -> impl IntoResponse {
    let template = templates::MarkdownTemplate { safe_html: get_markdown(id)};
    templates::HtmlTemplate(template)
}

pub async fn hello_from_the_server() -> &'static str {
    "Hello!"
}

pub fn get_notes() -> Vec<Route> {
    let paths = fs::read_dir("./assets/markdown").unwrap();

    let mut routes = vec![];
    for path in paths {
        let route_str = path.unwrap().path().display().to_string().to_owned();
        if route_str.contains(".md") {
            let name_with_filetype = route_str.split("/").last().unwrap();
            let name_without_filetype = name_with_filetype.replace(".md", "");
            let name = name_with_filetype.replace("-", " ").replace(".md", "").to_title_case();
            routes.push(
                Route::new(&name, &format!("/notes/{}", name_with_filetype), Some(&format!("/notes/{}", name_without_filetype)))
            );
        }
    }
    routes
}
