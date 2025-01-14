use steam_rs::{Steam, steam_id::SteamId};

mod common;

const EXAMPLE_STEAM_ID: SteamId = SteamId(76561197960435530); // Robin Walker
const EXAMPLE_VANITY_URLS: [&'static str; 2] = [
    "gabelogannewell", // Represents a working vanity URL
    "!@#$%^&*()" // Represents a broken vanity URL that would fail
];

#[test]
pub fn get_friend_list() {
    async_test!(async {
        let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
        println!("{:?}", steam.get_friend_list(EXAMPLE_STEAM_ID, None).await.unwrap());
    });
}

#[test]
pub fn get_player_bans() {
    async_test!(async {
        let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
        println!("{:?}", steam.get_player_bans(vec![EXAMPLE_STEAM_ID]).await.unwrap());
    });
}

#[test]
pub fn get_player_summaries() {
    async_test!(async {
        let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
        println!("{:?}", steam.get_player_summaries(vec![EXAMPLE_STEAM_ID]).await.unwrap());
    });
}

#[test]
pub fn get_user_group_list() {
    async_test!(async {
        let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
        println!("{:?}", steam.get_user_group_list(EXAMPLE_STEAM_ID).await.unwrap());
    });
}

#[test]
pub fn resolve_vanity_url() {
    async_test!(async {
        for url in EXAMPLE_VANITY_URLS {
            let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));
            println!("{:?}", steam.resolve_vanity_url(url, None).await.unwrap());
        }
    });
}