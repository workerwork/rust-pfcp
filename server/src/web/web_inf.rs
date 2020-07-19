use actix_redis::{Command, Error, RedisActor};
use actix_web::{web, App, Error as AWError, HttpResponse, HttpServer, Responder, middleware};
use redis_async::resp::RespValue;

// This struct represents state
struct AppState {
    app_name: String,
}

async fn index5(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {}!", app_name) // <- response with app_name
}

async fn set(redis: web::Data<Addr<RedisActor>>) -> Result<HttpResponse, AWError> {
    //    let result:Result<RespValue,Error> = redis.send(Command(resp_array!["SET","myname","myvalue"])).await?;
    let result = redis
        .send(Command(resp_array!["set", "name", "myvalue"]))
        .await?;

    match result {
        Ok(RespValue::SimpleString(s)) if s == "OK" => {
            Ok(HttpResponse::Ok().body("Set values success!"))
        }
        _ => {
            println!("---->{:?}", result);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

async fn get(redis: web::Data<Addr<RedisActor>>) -> Result<HttpResponse, AWError> {
    let result = redis.send(Command(resp_array!["get", "name"])).await?;
    match result {
        Ok(RespValue::BulkString(s)) => Ok(HttpResponse::Ok().body(s)),
        _ => {
            println!("---->{:?}", result);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[actix_rt::main]
pub async fn http_server() -> std::io::Result<()> {
    HttpServer::new(move || {
        let redis_addr = RedisActor::start("localhost:6379");
        App::new().service(
            web::scope("/app")
                .data(redis_addr)
                .route("/set", web::get().to(set))
                .route("/get", web::get().to(get))
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
