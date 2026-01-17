use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct WordEntry {
    word: String,
    topic: String,
    difficulty: String,
}

struct AppState {
    words: Mutex<Vec<WordEntry>>,
}

#[derive(Deserialize)]
struct QueryParams {
    topic: Option<String>,
    difficulty: Option<String>,
    exclude: Option<String>,
}

#[get("/api/word")]
async fn get_random_word(data: web::Data<AppState>, params: web::Query<QueryParams>) -> impl Responder {
    let words = data.words.lock().unwrap();

    let mut filtered_words: Vec<&WordEntry> = words.iter().filter(|w| {
        let topic_match = params.topic.as_ref().map_or(true, |t| w.topic.eq_ignore_ascii_case(t));
        let difficulty_match = params.difficulty.as_ref().map_or(true, |d| w.difficulty.eq_ignore_ascii_case(d));
        topic_match && difficulty_match
    }).collect();

    // If there are multiple words, exclude the one specified in params to ensure a "new" word is picked.
    // If there is only 1 (or 0) word, we don't exclude it, so the user still sees the result.
    if let Some(exclude) = &params.exclude {
        if filtered_words.len() > 1 {
            filtered_words.retain(|w| !w.word.eq_ignore_ascii_case(exclude));
        }
    }

    if let Some(word) = filtered_words.choose(&mut rand::thread_rng()) {
        HttpResponse::Ok().json(word)
    } else {
        HttpResponse::NotFound().body("No words found matching criteria")
    }
}

#[get("/api/topics")]
async fn get_topics(data: web::Data<AppState>) -> impl Responder {
    let words = data.words.lock().unwrap();
    let mut topics: Vec<String> = words.iter().map(|w| w.topic.clone()).collect();
    topics.sort();
    topics.dedup();
    HttpResponse::Ok().json(topics)
}

#[get("/api/difficulties")]
async fn get_difficulties(data: web::Data<AppState>, params: web::Query<QueryParams>) -> impl Responder {
    let words = data.words.lock().unwrap();
    let mut difficulties: Vec<String> = words.iter()
        .filter(|w| params.topic.as_ref().map_or(true, |t| w.topic.eq_ignore_ascii_case(t)))
        .map(|w| w.difficulty.clone())
        .collect();

    difficulties.sort();
    difficulties.dedup();

    // Custom sorting logic: Easy/Medium/Hard first, then Kids/Adults, then others
    let order = ["Easy", "Medium", "Hard", "Kids", "Adults"];
    difficulties.sort_by_key(|d| {
        order.iter().position(|&x| x == d).unwrap_or(999)
    });

    HttpResponse::Ok().json(difficulties)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let file = File::open("words.json").expect("Could not open words.json");
    let reader = BufReader::new(file);
    let words: Vec<WordEntry> = serde_json::from_reader(reader).expect("Could not parse JSON");

    let app_state = web::Data::new(AppState {
        words: Mutex::new(words),
    });

    println!("Server running at http://127.0.0.1:8080/");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(get_random_word)
            .service(get_topics)
            .service(get_difficulties)
            .service(actix_files::Files::new("/", "./static").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
