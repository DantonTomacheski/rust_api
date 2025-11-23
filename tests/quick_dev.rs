use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    let req_login = hc.do_post(
        "/api/login", 
        json!({
            "name": "Danton",
            "pwd": "1234"
        }
    ));

    req_login.await?.print().await?;

    Ok(())
}