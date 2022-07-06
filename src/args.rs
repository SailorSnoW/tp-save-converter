use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the GCI file containing the quest logs.
    pub path: String,

    /// Name of the 1st quest log .bin file
    #[clap(long, default_value_t = String::from("save1"))]
    pub save1: String,
    /// Name of the 2nd quest log .bin file
    #[clap(long, default_value_t = String::from("save2"))]
    pub save2: String,
    /// Name of the 3rd quest log .bin file
    #[clap(long, default_value_t = String::from("save3"))]
    pub save3: String,
}