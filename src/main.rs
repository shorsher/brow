extern crate clap;

use std::process::Command;
use clap::{Arg, App};

fn main() {
    let matches = App::new("brow")
        .version("0.1")
        .author("Alex Shorsher <alex.shorsher@gmail.com>")
        .about("enter your docker containers :)")
        .arg(Arg::with_name("id")
            .required(true)
            .index(1))
        .get_matches();


    let docker_id = matches.value_of("id").unwrap();


    let mut docker_list = Command::new("docker");
    docker_list.arg("inspect")
               .arg(docker_id);
    docker_list.status()
               .expect("ls command failed to start");
}
