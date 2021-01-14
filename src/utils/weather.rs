use crate::types::Config;

// will make a GET request from wttr.in
pub fn get_weather(config: &Config) -> String {
    let format = if config.weather.format.is_empty() {
        String::from("%l:+%t")
    } else {
        config.weather.format.clone()
    };

    let url = format!("http://wttr.in/{}?format=\"{}", config.weather.city, format);
    let err_string = String::from("Error");
    let res = match minreq::get(url).send() {
        Ok(resp) => match resp.as_str() {
            Ok(res_str) => res_str.trim_matches('"').to_string(),
            Err(_) => err_string,
        },
        Err(_) => err_string,
    };

    format!("  {}  {}  {}", config.weather.icon, res, config.seperator)
}
