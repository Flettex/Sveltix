use std::{collections::HashMap, env};

use actix_utils::future::{ready, Ready};
use actix_web::{
    cookie::Key, dev, error, middleware, web, App, FromRequest, HttpRequest, HttpServer, Responder,
};
use actix_web_lab::respond::Html;

use actix_files::Files;
use minijinja_autoreload::AutoReloader;
// use serde_json::json;

pub mod db;
pub mod server_props;

struct MiniJinjaRenderer {
    tmpl_env: web::Data<minijinja_autoreload::AutoReloader>,
}

impl MiniJinjaRenderer {
    fn render(
        &self,
        tmpl: &str,
        ctx: impl Into<minijinja::value::Value>,
    ) -> actix_web::Result<Html> {
        self.tmpl_env
            .acquire_env()
            .map_err(|_| error::ErrorInternalServerError("could not acquire template env"))?
            .get_template(tmpl)
            .map_err(|_| error::ErrorInternalServerError("could not find template"))?
            .render(ctx.into())
            .map(Html)
            .map_err(|_err| error::ErrorInternalServerError("template error"))
    }
}

impl FromRequest for MiniJinjaRenderer {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _pl: &mut dev::Payload) -> Self::Future {
        let tmpl_env = <web::Data<minijinja_autoreload::AutoReloader>>::extract(req)
            .into_inner()
            .unwrap();

        ready(Ok(Self { tmpl_env }))
    }
}

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
            // .app_data(web::Data::new(pool.clone()))
            .app_data(tmpl_reloader.clone())
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(index)))
            // Put this after the index route or it won't work
            // We need actix-files to serve the static JS, CSS and image files
            .service(Files::new(
                "/",
                source_directory.join("sveltekit_app/build"),
            ))
    })
    .workers(2)
    .bind((if IS_DEBUG { dev_server_addr } else { "0.0.0.0" }, 8080))?
    .run()
    .await
}
