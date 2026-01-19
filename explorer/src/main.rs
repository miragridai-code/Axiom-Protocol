use actix_web::{web, App, HttpServer, HttpResponse};
use qubit_core::block::Block;
use qubit_core::state::State;
use std::sync::Mutex;

struct AppState {
    state: Mutex<State>,
    blocks: Mutex<Vec<Block>>,
}

async fn get_blocks(data: web::Data<AppState>) -> HttpResponse {
    let blocks = data.blocks.lock().unwrap();
    HttpResponse::Ok().json(&*blocks)
}

async fn get_state(data: web::Data<AppState>) -> HttpResponse {
    let state = data.state.lock().unwrap();
    HttpResponse::Ok().json(&*state)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        state: Mutex::new(State::new()),
        blocks: Mutex::new(vec![]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/blocks", web::get().to(get_blocks))
            .route("/state", web::get().to(get_state))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
