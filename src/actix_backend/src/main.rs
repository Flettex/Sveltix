use std::{collections::HashMap, env};

use actix_web::{
    cookie::Key, middleware, web, App, HttpServer, Responder,
};
use actix_files::Files;
use actix_web::dev::{ServiceRequest, ServiceResponse, fn_service};
use minijinja_autoreload::AutoReloader;
// use serde_json::json;

pub mod db;
pub mod server_props;
pub mod template_engine;

use template_engine::MiniJinjaRenderer;

async fn index(tmpl_env: MiniJinjaRenderer) -> actix_web::Result<impl Responder> {
    return tmpl_env.render(
        "index.html",
        minijinja::context! {
            SSR_DATA => HashMap::from([("data", "Deez")])
        },
    );
}

const IS_DEBUG: bool = option_env!("PROD").is_none();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let dev_server_addr = match option_env!("SERVER_ADDRESS") {
        Some(addr) => &(addr as &str),
        None => "127.0.0.1",
    };

    if IS_DEBUG {
        println!(
            "\nStarting server on address http://{}:8080/\n",
            dev_server_addr
        )
    };

    let pool =
        sqlx::PgPool::connect(&env::var("DATABASE_URL").expect("DATABASE_URL env var not set"))
            .await
            .expect("Failed to create postgres pool");

    let enable_template_autoreload = env::var("TEMPLATE_AUTORELOAD").as_deref() == Ok("true");

    let tmpl_reloader = AutoReloader::new(move |notifier| {
        let mut env: minijinja::Environment<'static> = minijinja::Environment::new();

        let tmpl_path = std::env::current_dir().unwrap();
        let tmpl_path = tmpl_path.parent().unwrap().join("sveltekit_app/build");

        // if watch_path is never called, no fs watcher is created
        if enable_template_autoreload {
            notifier.watch_path(&tmpl_path, true);
        }

        env.set_source(minijinja::Source::from_path(tmpl_path));
    
        Ok(env)
    });

    let tmpl_reloader = web::Data::new(tmpl_reloader);

    HttpServer::new(move || {
        // let _current_dir = env::current_dir().unwrap();
        // let source_directory = _current_dir.parent().unwrap();

        let _key: Vec<u8> = env::var("SECRET_KEY")
            .unwrap()
            .replace("'", "")
            .split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.parse::<u8>().unwrap())
            .collect();

        let _secret_key = Key::from(&_key);

        let source_directory = std::env::current_dir().unwrap();
        let source_directory = source_directory.parent().unwrap();


        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(tmpl_reloader.clone())
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(index)))
            // Put this after the index route or it won't work
            // We need actix-files to serve the static JS, CSS and image files
            .service(Files::new(
                "/flutter/",
                source_directory.join("flutter_app/build/web"),
            ).index_file(source_directory.join("flutter_app/build/web/index.html").to_str().unwrap()))
            .service(Files::new(
                "/",
                source_directory.join("sveltekit_app/build"),
            ).default_handler(web::route().to(|req: actix_web::HttpRequest, tmpl_env: MiniJinjaRenderer| async move {
                let filename = req.uri().path();
                let filename = &filename[1..filename.len()];
                println!("file name is: {}", filename);
                tmpl_env.render(format!("{}.html", filename).as_str(), minijinja::context! {})          
            }))
        )
    })
    .workers(2)
    .bind((if IS_DEBUG { dev_server_addr } else { "0.0.0.0" }, 8080))?
    .run()
    .await
}
