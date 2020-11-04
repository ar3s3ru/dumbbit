#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let mut app = tide::new();

    app.at("/").get(|_| async move { Ok("Hello, world!\n") });

    Ok(app.listen("0.0.0.0:8080").await?)
}
