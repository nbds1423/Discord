use crate::{cmds as cmd, handler::Command};

pub async fn execute(command: Command) {
    match command.command_name.as_str() {
        "roles" => cmd::roles::roles(command).await,
        _ => println!("[Commands] -> Command not found"),
    }
}
