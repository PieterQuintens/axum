use anyhow::Result;
use serde_json::json;

// Because Windows, we need to run the tests like this...
// run: cargo watch -c -w tests/ -x "test --target-dir=target/test"
#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000").unwrap();

    hc.do_get("/path/test").await?.print().await?;

    let req_create_tickets = hc.do_post(
        "/api/tickets",
        json!({
          "title": "Test ticket"
        }),
    );
    req_create_tickets.await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}
