use mangadex_api_schema_rust::v5::ChapterCollection;
use std::{
    fs::File,
    io::{Read, Write},
};

use reqwest::Client;
use url::Url;

#[tokio::test]
#[ignore]
async fn chapter_serializing_test() {
    let client = Client::new();
    let res = client
        .get(Url::parse("https://api.mangadex.org/chapter").unwrap())
        .send()
        .await
        .unwrap();
    let response_text = res.text().await.unwrap();
    let mangadata: ChapterCollection = serde_json::from_str(response_text.as_str()).unwrap();
    let json_mangadata = serde_json::to_string(&mangadata).unwrap();
    let mut file1: File = File::create("test-output/chapter/1.json").unwrap();
    let mut file2: File = File::create("test-output/chapter/2.json").unwrap();
    file1.write_all(json_mangadata.as_bytes()).unwrap();
    file2.write_all(response_text.as_bytes()).unwrap();
}

#[tokio::test]
#[ignore]
async fn compare_1_2() {
    let mut file1: File = File::open("test-output/chapter/1.json").unwrap();
    let mut file2: File = File::open("test-output/chapter/2.json").unwrap();
    let mut file1_data = String::default();
    file1.read_to_string(&mut file1_data).unwrap();
    let mut file2_data = String::default();
    file2.read_to_string(&mut file2_data).unwrap();
    assert_eq!(file1_data, file2_data);
}

#[tokio::test]
#[ignore]
async fn test_des_and_ser() {
    let mut file1: File = File::open("test-output/chapter/1.json").unwrap();
    let mut file1_data = String::default();
    file1.read_to_string(&mut file1_data).unwrap();
    let mangadata: ChapterCollection = serde_json::from_str(file1_data.as_str()).unwrap();
    assert_eq!(serde_json::to_string(&mangadata).unwrap(), file1_data);
}
