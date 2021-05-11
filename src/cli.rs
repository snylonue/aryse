use clap::App;
use clap::Arg;

pub fn app() -> App<'static, 'static> {
    App::new("crab").arg(Arg::with_name("image").help("image to open").multiple(true))
}
