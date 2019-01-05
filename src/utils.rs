use clap::ArgMatches;
use std::collections::HashMap;

use crate::kube;

pub fn str_to_string(input: Vec<&str>) -> Vec<String> {
    let mut string_vec = Vec::new();
    for x in input {
        string_vec.push(x.to_owned());
    }
    string_vec
}

fn create_secret_map(secrets: &Vec<String>) -> HashMap<String, String> {
    let mut secret_map = HashMap::new();

    for secret in secrets {
        let split_vect: Vec<&str> = secret.split(":").collect();
        secret_map.insert(split_vect[0].to_string(), split_vect[1].to_string());
    }

    secret_map
}

pub fn set_args(args: &ArgMatches) -> Result<kube::KubeSecret, Box<::std::error::Error>> {
    let namespace = args.value_of("NAMESPACE").unwrap();
    let name = args.value_of("NAME").unwrap();

    let new_secrets = args.values_of("SECRET").unwrap().collect();
    let owned_secrets = str_to_string(new_secrets);
    let secret_map = create_secret_map(&owned_secrets);

    Ok(kube::KubeSecret::new(
        name.to_string(),
        namespace.to_string(),
        secret_map,
    ))
}
