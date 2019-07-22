use tide::{http, Context};

pub(crate) async fn view(_cx: Context<()>) -> tide::EndpointResult {
    let template = r#"
        <body>
            <h1 class="sans-serif">Soupstagram</h1>
        <body>
    "#;

    let html = html_index::Builder::new()
        .raw_body(template)
        .style("/bundle.css")
        .build();

    let res = http::Response::builder()
        .header(http::header::CONTENT_TYPE, mime::TEXT_HTML.as_ref())
        .status(http::StatusCode::OK)
        .body(html.as_bytes().into()).unwrap();
    Ok(res)
}
