use jup::send_request;
use tokio::{fs::File, io::{AsyncBufReadExt, BufReader}};

mod jup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let file = File::open("addrs.txt").await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let client = reqwest::ClientBuilder::new().build()?;

    // Read lines asynchronously
    while let Some(wallet) = lines.next_line().await? {
        match send_request(&client, &wallet).await {
            Ok(res) => {
                if res.data.total_allocated > 0 {
                    println!("{wallet} {}", res.data.total_allocated);
                }
            }
            Err(err) => {
                println!("failed to send request {err:#?}");
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
    Ok(())
}
