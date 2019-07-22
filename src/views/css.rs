use tide::{http, Context};

pub(crate) async fn view(_cx: Context<()>) -> tide::EndpointResult {
    let mut css = String::from(tachyons::TACHYONS_DEFAULT);
    css.push_str(tachyons::TACHYONS);
    let res = http::Response::builder()
        .header(http::header::CONTENT_TYPE, mime::TEXT_CSS.as_ref())
        .status(http::StatusCode::OK)
        .body(css.as_bytes().into()).unwrap();
    Ok(res)
}
