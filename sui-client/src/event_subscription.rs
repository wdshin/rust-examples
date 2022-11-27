use futures::StreamExt;
use sui_sdk::rpc_types::SuiEventFilter;
use sui_sdk::SuiClient;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // todo url 을 argument 로 가져오기
    // todo event filter 도 argument 로 가져오기
    let sui = SuiClient::new("http://127.0.0.1:5001", Some("ws://127.0.0.1:9001"), None).await?;
    let mut subscribe_all = sui
        .event_api()
        .subscribe_event(SuiEventFilter::All(vec![]))
        .await?;
    loop {
        println!("{:?}", subscribe_all.next().await);
    }
}