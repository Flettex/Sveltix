use std::env;

use actix_web::{
    self, cookie::Key,
    get, App, HttpServer, Responder,
};
use actix_files::Files;


#[get("/")]
async fn index() -> impl Responder {
    "works!?!?"
}

const IS_DEBUG: bool = option_env!("PROD").is_none();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let dev_server_addr = match option_env!("SERVER_ADDRESS") {
        Some(addr) => &(addr as &str),
        None => "127.0.0.1"
    };

    if IS_DEBUG {println!("\nStarting server on address http://{}:8080/\n", dev_server_addr)};

    // let pool = PgPool::connect(&env::var("DATABASE_URL").expect("DATABASE_URL env var not set"))
    //     .await
    //     .expect("Failed to create postgres pool");


    HttpServer::new(move || {
        let _current_dir = env::current_dir().unwrap();
        let source_directory = _current_dir.parent().unwrap();

        let _key: Vec<u8> = env::var("SECRET_KEY")
            .unwrap()
            .replace("'", "")
            .split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.parse::<u8>().unwrap())
            .collect();
    
        let _secret_key = Key::from(&_key);


        App::new()
            // .app_data(web::Data::new(pool.clone()))
            .service(Files::new("/", source_directory.join("sveltekit_app/build/")).index_file(source_directory.join("sveltekit_app/build/index.html").to_str().unwrap().to_owned()))
            .service(index)
    })
    .workers(2)
    .bind((if IS_DEBUG { dev_server_addr } else { "0.0.0.0" }, 8080))?
    .run()
    .await
}
