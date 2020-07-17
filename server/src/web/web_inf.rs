use actix_web::{web, App, HttpResponse, HttpServer, Responder};

// This struct represents state
struct AppState {
    app_name: String,
}


async fn index5(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {}!", app_name) // <- response with app_name
}

#[actix_rt::main]
pub async fn http_server() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new().service(
            web::scope("/app")
                .data(AppState {
                    app_name: String::from("Actix-web"),
                })
                //.app_data(counter.clone()) // <- register the created data
                //.route("/", web::get().to(index))
                //.route("/again", web::get().to(index2))
                //.route("/test", web::get().to(index3))
                //.route("/index.html", web::get().to(index4))
                .route("/name", web::get().to(index5)), //.route("/dongfeng", web::get().to(_index)),
        )
    })  
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

