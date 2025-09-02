mod settings;
mod ingester;

fn main() {
    dotenv::dotenv().ok();

    let mut settings = settings::Settings::new();
    settings.load("MYNAME").expect("Could not load variable");
    settings.load("STRAVA_KEY").expect("Could not load variable");

    let value = settings.get_value("MYNAME").unwrap();
    println!("{}", value);
}

