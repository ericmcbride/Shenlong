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

    let mut delete_bool = false;
    if let Some(_) = args.value_of("DELETE") {
        let delete = args.value_of("DELETE").unwrap();
        if delete == "true" {
            delete_bool = true
        }
    }

    Ok(kube::KubeSecret::new(
        name.to_string(),
        namespace.to_string(),
        delete_bool,
        secret_map,
    ))
}

#[cfg(test)]
mod tests {
    use super::create_secret_map;
    use super::str_to_string;

    #[test]
    fn test_create_secret_map() {
        let mut test_vec = Vec::new();
        test_vec.push("foo:boi".to_string());

        let got = create_secret_map(&test_vec);
        assert_eq!("boi".to_string(), got["foo"]);
        assert_eq!(got.contains_key("foo"), true);
    }

    #[test]
    fn test_str_to_string() {
        let mut test_vec = Vec::new();
        test_vec.push("foo");

        let got = str_to_string(test_vec);
        assert_eq!("foo".to_string(), got[0])
    }

}
