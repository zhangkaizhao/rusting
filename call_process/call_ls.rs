use std::process::Command;

fn main() {
    match Command::new("ls").arg("-la").spawn() {
        Ok(mut _child) => {
            println!("call ls successfully");
            match _child.wait() {
                Ok(_status) => println!("exit status {}", _status),
                Err(_err) => {}
            }
        }
        Err(_err) => println!("failed to call ls"),
    }
}
