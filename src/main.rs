use rand::seq::SliceRandom;
use serde::{Serialize, Deserialize};
use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "structopt example", about = "using structopt")]
struct Args {
    #[structopt(short = "f", long)]
    file: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Inputs {
    host: String,
    username: String,
    password: String,
    food_places: Vec<String>,
}

fn main() {
    let args = Args::from_args();
    let f = std::fs::File::open(args.file).unwrap();
    let deserialized_inputs: Inputs = serde_yaml::from_reader(f).unwrap();

    let choice = deserialized_inputs.food_places.choose(
        &mut rand::thread_rng()).unwrap();

    let tcp = TcpStream::connect(
        deserialized_inputs.host + ":22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password(
        &deserialized_inputs.username,
        &deserialized_inputs.password).unwrap();

    let mut command: String =
        "curl -X POST -H 'Content-Type: application/json' \
        -d '{\"message\": \"".to_owned();
    let command_end: String = "\"}' 'http://localhost:8080/IFTTT'".to_owned();
    command.push_str(&&choice);
    command.push_str(&command_end);
    println!("{}", command);

    let mut channel = sess.channel_session().unwrap();
    channel.exec(&command).unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.wait_close().ok();
    println!("{}", channel.exit_status().unwrap());
}
