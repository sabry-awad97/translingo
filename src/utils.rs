use regex::Regex;

pub fn extract_too_many_requests_info(html: &str) -> (String, String, String) {
    let ip_regex = Regex::new(r"IP address: (.+?)<br>").unwrap();
    let ip = ip_regex
        .captures(html)
        .map_or(String::new(), |cap| cap[1].to_string());

    let time_regex = Regex::new(r"Time: (.+?)<br>").unwrap();
    let time = time_regex
        .captures(html)
        .map_or(String::new(), |cap| cap[1].to_string());

    let url_regex = Regex::new(r"URL: (.+?)<br>").unwrap();
    let url = url_regex
        .captures(html)
        .map_or(String::new(), |cap| cap[1].to_string())
        .replace("&amp;", "&");

    (ip, time, url)
}
