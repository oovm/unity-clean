use unity_utils::UnityProject;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_walk() {
    let project = UnityProject::new("C:\\P4Root\\project\\OtherPlanet").unwrap();
    for file in project.find_meta().take(10) {
        println!("{:#?}", file);
    }
}
