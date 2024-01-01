#[allow(dead_code)]

use std::sync::Arc;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use tokio::sync::Mutex;

use crate::templates;
use crate::config;
use crate::markdown::get_markdown;


pub struct AppState {
    pub note_state: NoteState
}

#[derive(Clone)]
pub struct Route {
    pub name: String,
    pub path: String,
}

impl Route {
    fn new(name: &str, path: &str) -> Self {
        Route {
            name: name.to_string(),
            path: path.to_string(),
        }
    }
}

#[derive(Clone)]
pub struct Note {
    pub route: Route,
    pub name: String,
    pub category: String,
    pub shorthand: String,
}

impl Note {
    fn new(route: Route, category: String, shorthand: String) -> Self {
        let name = route.clone().name;
        Note {
            route,
            name,
            category,
            shorthand,
        }
    }
}

#[derive(Clone)]
pub struct NoteCategory {
    pub name: String,
    pub notes: Vec<Note>
}

#[derive(Clone)]
pub struct NoteState {
    pub categories: Vec<NoteCategory>,
}

impl NoteState {
    fn new(categories: Vec<NoteCategory>) -> Self {
        NoteState {
            categories
        }
    }
}

pub fn get_routes() -> [Route; 3] {
    [
        Route::new("About", "/about"),
        Route::new("Notes", "/notes"),
        Route::new("Code", "/code"),
    ]
}

pub async fn hello() -> impl IntoResponse {
    let template = templates::HelloTemplate {};
    templates::HtmlTemplate(template)
}

pub async fn notes(State(state): State<Arc<AppState>>) -> impl IntoResponse {

    let template = templates::NotesTemplate {
        note_state: state.note_state.clone(),
    };
    templates::HtmlTemplate(template)
}


pub async fn notes_md(Path((category, file)): Path<(String, String)>) -> impl IntoResponse {
    let template = templates::MarkdownTemplate { safe_html: get_markdown(format!("{}/{}", category, file))};
    templates::HtmlTemplate(template)
}

pub async fn hello_from_the_server() -> &'static str {
    "Hello!"
}

pub fn get_notes() -> NoteState {
    let data = config::get_config();
    let mut categories: Vec<NoteCategory> = vec![];
    for category in data.notes.categories.iter() {
        let mut notes: Vec<Note> = vec![];
        for route in category.notes.iter() {
            let name_without_filetype = route.name.split("/").last().unwrap().replace(".md", "");
            notes.push(Note::new(Route::new(&route.name, &route.route), category.name.clone(), format!("/notes/{}", name_without_filetype),))
        }
        categories.push(NoteCategory {
            name: category.name.clone(),
            notes
        })
    }

    NoteState::new(categories)
}
