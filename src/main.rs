#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use std::process::Command;

mod kube;
mod utils;

fn main() {
    match run() {
        Ok(_) => println!("Secrets Updated"),
        Err(e) => {
            eprintln!("Error {}", e);
        }
    }
}

fn run() -> Result<(), Box<::std::error::Error>> {
    let args = clap_app!(primarch =>
        (version: "1.0")
        (author: "Eric McBride <ericmcbridedeveloper@gmail.com>")
        (about: "Kube Secret Updater")
        (@arg NAMESPACE: -n --namespace +required +takes_value "Namespace for secrets")
        (@arg NAME: --name +required +takes_value "Secrets File Name")
        (@arg SECRET: ... --secret +required +takes_value "Key:Value secrets (multiple can be set")
    )
    .get_matches();

    let kube_secrets = utils::set_args(&args);
    match kube_secrets {
        Ok(_) => update_secrets(),
        Err(_) => Ok(()),
    }
}

fn login_eks() -> Result<(), Box<::std::error::Error>> {
    Ok(())
}

fn update_secrets() -> Result<(), Box<::std::error::Error>> {
    let delete_cmd = Command::new("kubectl")
        .args(&["delete secret", ""])
        .output();
    println!("Kubectl Delete Output {:?}", delete_cmd);

    Ok(())
}
