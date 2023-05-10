use inquire::Text;

pub fn input() -> String {
    let value = Text::new("Type a Project name: ")
        .with_placeholder("project_name")
        .prompt()
        .unwrap();

    if !value.is_empty() {
        value
    } else {
        println!("Invalid project name");
        input()
    }
}
