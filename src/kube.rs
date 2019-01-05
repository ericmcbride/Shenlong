use std::collections::HashMap;
extern crate serde;
extern crate serde_yaml;

#[derive(Serialize, Deserialize, Debug)]
pub struct KubeSecret {
    api_version: String,
    kind: String,
    metadata: MetaData,
    resource_type: String,
    data: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MetaData {
    name: String,
    namespace: String,
}

impl KubeSecret {
    pub fn new(
        name: String,
        namespace: String,
        secrets: HashMap<String, String>,
    ) -> KubeSecret {
        KubeSecret {
            api_version: "v1".to_string(),
            kind: "Secret".to_string(),
            metadata: MetaData { name, namespace },
            resource_type: "Opaque".to_string(),
            data: KubeSecret::encode_secrets(&secrets),
        }
    }

    fn encode_secrets(secrets: &HashMap<String, String>) -> HashMap<String, String> {
        let mut hashed_secrets = HashMap::new();
        for (k, v) in secrets {
            let hashed_value = base64::encode(&v);
            hashed_secrets.insert(k.to_owned(), hashed_value);
        }
        hashed_secrets
    }

    pub fn yaml(&self) -> String {
        serde_yaml::to_string(&self).unwrap()
    }
}
