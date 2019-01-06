# Shenlong
Shenlong is a Rust CLI Tool for Kubernetes Secret files. Shenlong allows the user to not create a file, and instead allows for the piping of the yaml file to the kube apply function.

This allows for no weird SED injections of template files in the build process,
or storing secret files in version control or secret files on the server.  Instead the user gets it on the fly.

![Shenlong](shenlong.jpg?raw=true "Shenlong")

## Installation
As of right now, Shenlong is NOT part of cargo.  Its on my todo list.  Right now just do cargo build in the root of the the project, and then access the shenlong in target/debug/shenlong
```bash
./target/debug/shenlong --namespace my-kube-namespace --name my-kube-secrets --secret FOO:BOI | kubectl apply -f -
```

## CLI Arguments
```
shenlong 1.0
Eric McBride <ericmcbridedeveloper@gmail.com>
Kube Secret Updater

USAGE:
    shenlong [OPTIONS] --name <NAME> --namespace <NAMESPACE> --secret <SECRET>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --delete <DELETE>          Delete Kube Secret (true or false)
        --name <NAME>              Secrets File Name
    -n, --namespace <NAMESPACE>    Namespace for secrets
        --secret <SECRET>...       Key:Value secrets (multiple can be set)
```

Shenlong requires a name, namespace, and secret.  You can have multiple secrets.  When passing in a secret, please do so in the following format ```secret:value ```.  The semicolon seperates the key to value.  This will write to the kube value as `secret = value` in the yaml string.

The Delete flag will delete the secret file based off the namespace and name
you pass in the CLI.  Shenlong will use your CURRENT KUBECTL CONTEXT to delete
the secret, and regenerate the secret file.  This is useful, since you have to
blow out a secret to update it.

## Example Usage
```
shenlong [master●] % ./target/debug/shenlong --namespace=test-space --secret pilot:Heero --secret gundam:Wing --name=my-gundam-secret | kubectl apply -f -
```

```
---
apiVersion: v1
kind: Secret
metadata:
  name: my-gundam-secret
  namespace: test-space
type: Opaque
data:
  pilot: SGVlcm8=
  gundam: V2luZw==
```

As you can see, the secrets get base64 encoded for you.  As of right now if you
dont pipe shenlong to the kubectl apply -f - command, you will just get a
representation of the yaml file printed out to stdout.

## The Future
    I plan on adding Cloud Service Support, such as passing in --cloud=AWS.  This
way Shenlong would be available to deploy secrets from CI services such as CIRCLE or
TRAVIS.  If you are running in the machine executor, and you are trying to talk
to AWS EKS, you may have to use the heptio-IAM-AUTHENTICATOR to do so.  I plan
on building the authentication into shenlong in the future.

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)
