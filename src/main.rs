use std::collections::HashMap;

use actix_web::{actix::System, http, server, App, HttpResponse, Query, Result};
use askama::Template;

#[derive(Template)]
#[template(path = "user.html")]
struct UserTemplate<'a> {
    name: &'a str,
    text: &'a str,
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

fn index(query: Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let s = if let Some(name) = query.get("name") {
        UserTemplate {
            name: name,
            text: "欢迎！",
        }.render()
            .unwrap()
    }
    else {
        Index.render().unwrap()
    };

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

fn main() {
    let sys = System::new("template-askama");

    // start http server
    let addr = "127.0.0.1:5555";
    server::new(move || {
        App::new().resource("/",
                            |r| r.method(http::Method::GET)
                                .with(index))
    }).bind(addr)
        .unwrap()
        .start();

    println!("Started http server: {}", addr);
    let _ = sys.run();
}
