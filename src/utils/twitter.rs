pub fn process_twitter_handler(twitter_extract: Option<String>) -> Option<String> {
    match &twitter_extract.unwrap()[..] {
        "" => None,
        v => Some(v.replace("@", "").trim().parse().unwrap()),
    }
}
