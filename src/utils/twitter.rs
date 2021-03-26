pub fn process_twitter_handler(twitter_extract: &String) -> Option<String> {
    Some(twitter_extract.replace("@", "").trim().parse().unwrap())
}
