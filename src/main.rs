use actix_web::{get, post, web, App, HttpServer, Responder};
use actix_cors::Cors;
use std::collections::HashMap;

#[get("/")]
async fn intro() -> impl Responder {
    "Hello!\n
    > '/get?query=value': receive 'GET'\n
    > '/post/json': receive 'POST' in Json\n
    > '/post/form': receive 'POST' in Form".to_string()
}

#[get("/get")]
async fn get_func(query: web::Query<HashMap<String, String>>) -> impl Responder {
    let mut response = String::from("Received query params:\n");
    for (key, value) in query.iter() {
        response.push_str(&format!("{key}: {value}\n"));
    }
    response
}

#[post("/post/json")]
async fn post_json_func(json: web::Json<HashMap<String, String>>) -> impl Responder {
    let mut response = String::from("Received json params:\n");
    for (key, value) in json.iter() {
        response.push_str(&format!("{key}: {value}\n"));
    }
    response
}

#[post("/post/form")]
async fn post_form_func(form: web::Form<HashMap<String, String>>) -> impl Responder {
    let mut response = String::from("Received form params:\n");
    for (key, value) in form.iter() {
        response.push_str(&format!("{key}: {value}\n"));
    }
    response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .service(intro)
            .service(get_func)
            .service(post_json_func)
            .service(post_form_func)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
