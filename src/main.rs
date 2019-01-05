#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;

use std::process::Command;

mod kube;
mod utils;

fn main() {
    match run() {
        Ok(secrets) => {
            println!("{}", secrets.gen_yaml());
        }
        Err(secrets) => {
            eprintln!("Error {}", secrets);
        }
    }
}

fn run() -> Result<kube::KubeSecret, Box<::std::error::Error>> {
    let args = clap_app!(shenlong =>
        (version: "1.0")
        (author: "Eric McBride <ericmcbridedeveloper@gmail.com>")
        (about: "Kube Secret Updater")
        (@arg NAMESPACE: -n --namespace +required +takes_value "Namespace for secrets")
        (@arg NAME: --name +required +takes_value "Secrets File Name")
        (@arg DELETE: -d --delete +takes_value "Delete Kube Secret (true or false)")
        (@arg SECRET: ... --secret +required +takes_value "Key:Value secrets (multiple can be set")
    )
    .get_matches();

    let kube_secrets = utils::set_args(&args);
    match kube_secrets {
        Ok(secrets) => {
            if secrets.delete {
                delete_secrets(&secrets)?;
                return Ok(secrets);
            }
            Ok(secrets)
        }
        Err(secrets) => Err(secrets),
    }
}

fn delete_secrets(secrets: &kube::KubeSecret) -> Result<(), Box<::std::error::Error>> {
    let delete_cmd = Command::new("kubectl")
        .args(&["-n", &secrets.metadata.namespace])
        .args(&["delete secret", &secrets.metadata.name])
        .output()?;
    println!("{}", String::from_utf8_lossy(&delete_cmd.stdout));
    Ok(())
}
