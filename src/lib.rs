use worker::*;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located within {}, {}, {}, {}",
        Date::now().to_string(),
        req.path(),
        req.cf().city().unwrap_or_else(|| "(unknown)".into()),
        req.cf().region().unwrap_or_else(|| "(unknown)".into()),
        req.cf().country().unwrap_or_else(|| "(unknown)".into()),
        req.cf().continent().unwrap_or_else(|| "(unknown)".into()),
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    log_request(&req);
    if let Ok(links) = env.kv("shortify") {
        let called_url = match req.url() {
            Ok(val) => val,
            Err(e) => return html_error(&e.to_string(), 500),
        };
        let path = called_url.path();
        let maybe_destinations = match links.get(path).text().await {
            Ok(val) => val,
            Err(e) => return html_error(&e.to_string(), 500),
        };
        if let Some(destinations) = maybe_destinations {
            let destinations: Vec<&str> = destinations.split('|').collect();
            let mut headers = Headers::new();
            if let Err(e) = headers.append(
                "Location",
                destinations[(js_sys::Math::random() * destinations.len() as f64) as usize],
            ) {
                return html_error(&format!("Location header is invalid: {e}"), 500);
            };
            Ok(Response::empty()?.with_status(302).with_headers(headers))
        } else {
            html_error("404 not found", 404)
        }
    } else {
        html_error("Server incorrectly configured with namespace!", 500)
    }
}

fn html_error(err: &str, statuscode: u16) -> Result<Response> {
    let mut context = tera::Context::new();
    context.insert("error", err);
    if let Ok(resp_html) = tera::Tera::one_off(include_str!("error.html"), &context, true) {
        let mut headers = Headers::new();
        headers.append("Content-Type", "text/html")?;
        return Ok(Response::error(resp_html, statuscode)?.with_headers(headers));
    }
    Response::error(err, statuscode)
}
