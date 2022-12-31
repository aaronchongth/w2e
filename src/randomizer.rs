use rand::seq::SliceRandom;
use serde::{Serialize, Deserialize};
use structopt::StructOpt;
use zenoh::config::Config;
use zenoh::prelude::r#async::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "w2e randomizer", about = "list of places to eat")]
struct Args {
    #[structopt(short = "f", long)]
    file: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Inputs {
    food_places: Vec<String>,
}

#[async_std::main]
async fn main() {
    env_logger::init();

    let args = Args::from_args();
    let f = std::fs::File::open(args.file).unwrap();
    let deserialized_inputs: Inputs = serde_yaml::from_reader(f).unwrap();

    let choice = deserialized_inputs.food_places.choose(
        &mut rand::thread_rng()).unwrap();

    println!("Opening session...");
    let session = zenoh::open(Config::default()).res().await.unwrap();

    let key_expr: String = String::from("magic_mirror/w2e");
    println!("Declaring Publisher on '{}'...", key_expr);
    let publisher = session.declare_publisher(&key_expr).res().await.unwrap();

    let buf = format!("{}", choice);
    println!("Putting Data ('{}': '{}')...", &key_expr, buf);
    publisher.put(buf).res().await.unwrap();
}
