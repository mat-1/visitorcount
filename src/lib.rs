mod render;

use std::io::Cursor;

use url::Url;

use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    let router = Router::new();

    console_error_panic_hook::set_once();

    router.get_async("/", index).run(req, env).await
}

pub async fn index(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let referer = req.headers().get("referer").ok().flatten();

    let Some(referer) = referer else {
        return Response::from_html(
                "<h1>mat's visitor counter</h1>\
                <p>just put this url in an img tag and it'll work (it gets the host from the referer)</p>
                <img src='https://counter.matdoes.dev'>"
            );
    };

    let Ok(referer) = Url::parse(&referer) else {
        return Response::from_body(ResponseBody::Body(
            format!("invalid referer: {referer}").into(),
        ));
    };
    let Some(referer) = referer.host_str() else {
        return Response::from_body(ResponseBody::Body(
            format!("invalid referer: {referer}").into(),
        ));
    };

    let kv = ctx.kv("visitorcount")?;

    let count: u64 = match kv.get(referer).text().await? {
        Some(number) => number.parse().unwrap_or_default(),
        None => 0,
    } + 1;
    kv.put(referer, count.to_string())?.execute().await?;

    let image = render::render(count);

    let mut buf = Cursor::new(Vec::new());
    image
        .write_to(&mut buf, image::ImageOutputFormat::Png)
        .unwrap();

    let mut response = Response::from_bytes(buf.into_inner())?;

    let headers = response.headers_mut();
    headers.set("Content-Type", "image/png")?;
    headers.set("Access-Control-Allow-Origin", "*")?;
    headers.set("Cache-Control", "max-age=86400")?;

    Ok(response)
}
