use selenium_rs::webdriver::{Browser,WebDriver};

const PIXIV_URL: &str = "https://www.pixiv.net/";

fn main() {
    let mut driver= WebDriver::new(Browser::Chrome);
    driver.start_session().unwrap();
    driver.navigate(PIXIV_URL).unwrap();
}
