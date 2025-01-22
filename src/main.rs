use jup::send_request;
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

mod jup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let file = File::open("addrs.txt").await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let client = reqwest::ClientBuilder::new().build()?;

    // Read lines asynchronously
    while let Some(wallet) = lines.next_line().await? {
        if let Ok(res) = send_request(&client, &wallet).await {
            if res.data.total_allocated > 0 {
                println!("{wallet} {}", res.data.total_allocated);
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
    Ok(())
}
