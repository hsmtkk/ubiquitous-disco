use thirtyfour::prelude::*;
use tokio;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:4444", &caps).await?;

    driver.get("https://wikipedia.org").await?;

    driver.quit().await?;

    Ok(())
}