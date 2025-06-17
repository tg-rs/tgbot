use super::*;

#[test]
fn api() {
    let client = Client::new("token").unwrap();
    assert_eq!(client.token, "token");
    assert_eq!(client.host, DEFAULT_HOST);

    let client = Client::new("token")
        .unwrap()
        .with_host("https://example.com")
        .with_max_retries(NonZero::new(1).unwrap());
    assert_eq!(client.token, "token");
    assert_eq!(client.host, "https://example.com");
    assert_eq!(client.max_retries, 1);
}
