pub(crate) use index::view as index;
pub(crate) use css::view as css;

pub(crate) mod index {
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
}

pub(crate) mod css {
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
}
