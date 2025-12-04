extern crate ureq;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Response {
    query: Tokens
}

#[derive(Deserialize, Debug)]
struct Tokens {
    tokens: LoginToken
}

#[derive(Deserialize, Debug)]
struct LoginToken {
    logintoken: String,
}

pub async fn dustloop_connection() -> ureq::Agent {

    // Dustloop bot login
    let agent = ureq::agent();
    let dustloop_api = "https://www.dustloop.com/wiki/api.php?";
    let lg: String = agent.get("https://www.dustloop.com/wiki/api.php?action=query&format=json&meta=tokens&formatversion=2&type=login").call().unwrap().body_mut().read_json::<Response>().unwrap().query.tokens.logintoken;
    let username: String = std::env::var("DUSTLOOP_USERNAME").unwrap();
    let pass: String = std::env::var("DUSTLOOP_PASSWORD").unwrap();

    let body = [
        ("action", "login"),
        ("lgname", &username),
        ("lgpassword", &pass),
        ("lgtoken", &lg),
        ("format", "json"),
        ("formatversion", "2")
    ];

    agent.post(dustloop_api).send_form(body).unwrap().into_body().read_to_string().unwrap();

    agent
}
