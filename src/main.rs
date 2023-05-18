use ammonia::clean;
use serde::Serialize;
use warp::Filter;

#[derive(Serialize)]
struct Sanitized {
    sanitized_html: String,
}

async fn sanitize_html(input: String) -> Result<impl warp::Reply, warp::Rejection> {
    let sanitized = clean(&input);
    let sanitized_str = sanitized.to_string();
    let response = Sanitized {
        sanitized_html: sanitized_str,
    };
    Ok(warp::reply::json(&response))
}

#[tokio::main]
async fn main() {
    let sanitize_route = warp::path!("api" / "sanitization_of_html")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 32)) // 32kb limit
        .and(warp::body::json())
        .and_then(sanitize_html);

    warp::serve(sanitize_route).run(([0, 0, 0, 0], 8080)).await;
}
