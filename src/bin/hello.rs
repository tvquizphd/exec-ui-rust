#![feature(rustc_private)]

use tokio::time::{sleep, Duration};
use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    // Navigate to https://wikipedia.org.
    driver.goto("http://wikipedia.org").await?;

    sleep(Duration::from_millis(2000)).await;

    driver.quit().await?;
    Ok(())
}
