use askama::Result;
use chrono::{DateTime, Utc};
use conduit::chrono;
use pulldown_cmark::{html, Options, Parser};

pub fn date(dt: &DateTime<Utc>, format: &str) -> Result<String> {
    Ok(dt.format(format).to_string())
}

pub fn md(content: &str) -> Result<String> {
    let parser = Parser::new_ext(content, Options::all());
    let mut out = String::with_capacity(content.len());
    html::push_html(&mut out, parser);

    Ok(out)
}
