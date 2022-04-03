use std::fs::File;

use hacker_news::story::Story;

#[test]
fn deserialize() {
    let pathly = |path| format!("{}/tests/{}", env!("CARGO_MANIFEST_DIR"), path);

    let bug_json = File::open(pathly("title-bug.json")).unwrap();
    let bug_json_2 = File::open(pathly("title-bug-2.json")).unwrap();

    let story: Story = serde_json::from_reader(bug_json).unwrap();
    println!("{story:#?}");

    let story: Story = serde_json::from_reader(bug_json_2).unwrap();
    println!("{story:#?}");
}
