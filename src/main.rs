use std::process::Command;

fn main() {
    let mut docker_list = Command::new("docker");
    docker_list.arg("ps");
    docker_list.status()
               .expect("ls command failed to start");
}
