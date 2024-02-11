use anyhow::Result;
use serde_json::json;

// run: cargo watch -c -w tests/ -x "test --target-dir=target/test"
#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000").unwrap();

    hc.do_get("/path/test").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
          "username":"pieter",
          "password":"dsscsev"
        }),
    );

    let res = req_login.await?;
    let body = res.text_body()?;

    println!("--> {:<12} - {:?}", "TEST", body);

    Ok(())
}
