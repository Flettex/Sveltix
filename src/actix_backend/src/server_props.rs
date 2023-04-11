use std::{future::Future, pin::Pin};

use actix_web::HttpRequest;

async fn homePageProps(_request: HttpRequest) -> String {
    "okokok".to_string()
}

async fn aboutPageProps(_req: HttpRequest) -> String {"HAHA ABOUT PAGE".to_owned()}


async fn map_route_to_callback(route: &str, request: HttpRequest) -> String {
    match route {
        "_page.svelte"  => homePageProps(request).await,

        _ => "".to_owned(),
    }
}