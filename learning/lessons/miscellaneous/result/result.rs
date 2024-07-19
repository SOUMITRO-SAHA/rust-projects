#[derive(Debug)]
enum Menu {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(input: &str) -> Result<Menu, String> {
    match input {
        "main" => Ok(Menu::MainMenu),
        "start" => Ok(Menu::Start),
        "quit" => Ok(Menu::Quit),
        _ => Err(format!("Invalid choice: {}", input)),
    }
}

fn main() {
    let choice = get_choice("main");
    println!("choice = {:?}", choice);
}
