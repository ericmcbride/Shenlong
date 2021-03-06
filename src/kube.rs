use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct KubeSecret {
    #[serde(rename = "apiVersion")]
    api_version: String,
    kind: String,
    pub metadata: MetaData,

    #[serde(skip_serializing)]
    pub delete: bool,

    #[serde(rename = "type")]
    resource_type: String,
    data: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetaData {
    pub name: String,
    pub namespace: String,
}

impl KubeSecret {
    pub fn new(
        name: String,
        namespace: String,
        delete: bool,
        secrets: HashMap<String, String>,
    ) -> KubeSecret {
        KubeSecret {
            api_version: "v1".to_string(),
            kind: "Secret".to_string(),
            metadata: MetaData { name, namespace },
            delete: delete,
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

    pub fn gen_yaml(&self) -> String {
        serde_yaml::to_string(&self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::KubeSecret;

    #[test]
    fn test_kubesecret() {
        let name = "name-boi".to_owned();
        let namespace = "dong-boi".to_owned();
        let secret = [("matt_pike".to_owned(), "god".to_owned())]
            .iter()
            .cloned()
            .collect();

        let expected = "---
apiVersion: v1
kind: Secret
metadata:
  name: name-boi
  namespace: dong-boi
type: Opaque
data:
  matt_pike: Z29k"
            .to_owned();
        assert_eq!(
            expected,
            KubeSecret::new(name, namespace, false, secret).gen_yaml()
        );
    }
}
