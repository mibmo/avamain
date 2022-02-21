use eyre::{Result as EResult, WrapErr};
use futures::stream::{self, StreamExt};
use seqset::Sequence;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use whois_rust::{WhoIs, WhoIsLookupOptions};

/// amount of concurrent tasks in any given stream handler
const CONCURRENCY_LIMIT: usize = 4;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> EResult<()> {
    let whois = Arc::new(
        WhoIs::from_string(include_str!("../whois-servers.json"))
            .wrap_err("loading whois configuration")?,
    );

    println!("domain,available");

    let tld = std::env::var("TLD")?;
    let charset = std::env::var("CHARSET")?;
    let length = std::env::var("LENGTH")?.parse()?;
    let names = Sequence::with_charset(length, &charset);
    stream::iter(names)
        .map(|name| (Arc::clone(&whois), format!("{}.{}", name, tld)))
        .for_each_concurrent(CONCURRENCY_LIMIT, check_domain_availability)
        .await;

    Ok(())
}

async fn check_domain_availability((whois, domain): (Arc<WhoIs>, String)) {
    sleep(Duration::from_millis(1000)).await;

    let lookup_options = WhoIsLookupOptions::from_string(domain.to_string()).unwrap();
    let whois_response = whois.lookup_async(lookup_options).await;
    match whois_response {
        Err(err) => eprintln!("whois lookup failed: {}", err),
        Ok(response) => {
            let available = response.contains("No Data Found");
            println!("{},{}", domain, available);
        }
    };
}
