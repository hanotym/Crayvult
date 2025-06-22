use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::env;

fn search_duckduckgo(query: &str) -> Option<String> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (compatible; search_bot/0.1)")
        .build()
        .unwrap();

    let url = format!(
        "https://duckduckgo.com/html/?q={}",
        urlencoding::encode(query)
    );

    let resp = client.get(&url).send().ok()?.text().ok()?;
    let document = Html::parse_document(&resp);
    let selector = Selector::parse("a.result__a").unwrap();

    document.select(&selector).next().map(|el| {
        let title = el.text().collect::<Vec<_>>().join("");
        let link = el.value().attr("href").unwrap_or("");
        format!("{} — {}", title, link)
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = args.get(1).unwrap_or(&"انشاء قناة يوتيوب".to_string());

    println!("🔍 البحث عن: {}\n", query);

    match search_duckduckgo(query) {
        Some(result) => println!("✅ النتيجة:\n{}", result),
        None => println!("❌ لم يتم العثور على نتائج."),
    }
}