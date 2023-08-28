mod api;

// https://slint.dev/releases/1.1.1/editor/?snippet=import+%7B+Button%2C+VerticalBox+%7D+from+%22std-widgets.slint%22%3B%0Aimport+%22.%2FNotoSans-Regular.ttf%22%3B%0A%0Aexport+component+Main+inherits+Window+%7B%0A++++init+%3D%3E+%7B%0A++++++++debug%28%22hello%22%29%0A++++%7D%0A++++max-height%3A+500px%3B%0A++++max-width%3A+500px%3B%0A++++min-height%3A+500px%3B%0A++++min-width%3A+500px%3B%0A++++default-font-family%3A+%22Noto+Sans%22%3B%0A++++no-frame%3A+false%3B%0A++++title%3A+%22deVOID%27s+Blog%22%3B%0A++++always-on-top%3A+true%3B%0A++++background%3A+black%3B%0A++++GridLayout+%7B%0A++++%0A++++%7D%0A%7D%0A
slint::include_modules!();


#[tokio::main]
async fn main() {
    App::new().unwrap().run().unwrap();
}