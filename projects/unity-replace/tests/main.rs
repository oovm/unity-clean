use unity_replace::UnityProject;

#[test]
fn ready() {
    println!("it works!")
}


#[test]
fn test_walk() {
    let project = UnityProject::new("C:\\P4Root\\project\\OtherPlanet").unwrap();
    for file in project.find_meta() {
        println!("{:?}", to_unix_path(&file));
    }
}