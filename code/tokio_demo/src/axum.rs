use axum::{Form, Json, response::Html, Router, routing::get};
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::routing::post;
use serde::Deserialize;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let serve_dir = ServeDir::new("static/assets2")
        .not_found_service(ServeFile::new("static/assets2/index.html"));
    let app = Router::new()
        .route("/", get(|| async { Html("<h1>Hello, World!</h1>") }))
        .route("/query", get(query))
        .route("/form", get(show_form).post(accept_form))
        .route("/json", post(accept_json))
        .nest_service("/assets", ServeDir::new("static/assets"))
        .nest_service("/assets2", serve_dir.clone())
        .fallback_service(serve_dir)
        .layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::debug!("Listening on {}",listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}


async fn show_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/form" method="post">
                    <label for="name">
                        Enter your name:
                        <input type="text" name="name">
                    </label>

                    <label>
                        Enter your email:
                        <input type="text" name="email">
                    </label>

                    <input type="submit" value="Subscribe!">
                </form>
            </body>
        </html>
        "#,
    )
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct InputParams {
    foo: i32,
    bar: String,
    third: Option<i32>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Input {
    name: String,
    email: String,
}

async fn accept_json(Json(input): Json<Input>
) -> Html<&'static str> {
    tracing::debug!("json params {:?}", input);
    Html("Json posted")
}

async fn accept_form(Form(input): Form<Input>
) -> Html<&'static str> {
    tracing::debug!("form params {:?}", input);
    Html("Form posted")
}

async fn query(Query(params): Query<InputParams>) -> impl IntoResponse {
    tracing::debug!("query params {:?}", params);
    Html("<h3>Test query</h3>")
}