use anyhow::{Context, Result};
use selenium_rs::webdriver::{Browser,WebDriver, Selector};
use std::env;

// const PIXIV_URL: &str = "https://www.pixiv.net/";
const PIXIV_LOGIN_URL: &str = "https://www.pixiv.net/login.php?ref=wwwtop_accounts_index";
const PIXIV_USERNAME: &str = "PIXIV_USERNAME";
const PIXIV_PASSWORD: &str = "PIXIV_PASSWORD";

fn main() {
    let pixiv_username = get_env_var(PIXIV_USERNAME).unwrap();
    let pixiv_password = get_env_var(PIXIV_PASSWORD).unwrap();
    let mut driver= WebDriver::new(Browser::Firefox);
    driver.start_session().unwrap();
    driver.navigate(PIXIV_LOGIN_URL).unwrap();
    std::thread::sleep(std::time::Duration::from_secs(10));
    for input_element in driver.find_elements(Selector::TagName, "input").unwrap(){
        dbg!(input_element);
    }
    /*
    let username_input = driver.find_element(Selector::CSS, "input[type=\"text\"]").unwrap();
    username_input.type_text(&pixiv_username).unwrap();
    let password_input = driver.find_element(Selector::CSS, "input[type=\"password\"]").unwrap();
    password_input.type_text(&pixiv_password).unwrap();
     */
}

fn get_env_var(key:&str) -> Result<String> {
    let var = env::var(key).with_context(|| format!("environment variable {} is not defined", key))?;
    Ok(var)
}