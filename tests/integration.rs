static HELP_CLI_OUTPUT: &'static str = "shenlong 1.0
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
        --secret <SECRET>...       Key:Value secrets (multiple can be set";

static WITHOUT_ARGS_OUTPUT: &'static str =
    "error: The following required arguments were not provided:
    --name <NAME>
    --namespace <NAMESPACE>
    --secret <SECRET>...

USAGE:
    shenlong [OPTIONS] --name <NAME> --namespace <NAMESPACE> --secret <SECRET>...

For more information try --help
";

static WITHOUT_NAME_OUTPUT: &'static str =
    "error: The following required arguments were not provided:
    --name <NAME>

USAGE:
    shenlong [OPTIONS] --name <NAME> --namespace <NAMESPACE> --secret <SECRET>...

For more information try --help
";

static WITHOUT_SECRET_OUTPUT: &'static str =
    "error: The following required arguments were not provided:
    --secret <SECRET>...

USAGE:
    shenlong [OPTIONS] --name <NAME> --namespace <NAMESPACE> --secret <SECRET>...

For more information try --help
";

static WITHOUT_NAMESPACE_OUTPUT: &'static str =
    "error: The following required arguments were not provided:
    --namespace <NAMESPACE>

USAGE:
    shenlong [OPTIONS] --name <NAME> --namespace <NAMESPACE> --secret <SECRET>...

For more information try --help
";

#[cfg(test)]
mod integration {
    use crate::HELP_CLI_OUTPUT;
    use crate::WITHOUT_ARGS_OUTPUT;
    use crate::WITHOUT_NAMESPACE_OUTPUT;
    use crate::WITHOUT_NAME_OUTPUT;
    use crate::WITHOUT_SECRET_OUTPUT;
    use std::process::Command;

    #[test]
    fn calling_shenlong_without_args() {
        let output = Command::new("./target/debug/shenlong")
            .output()
            .expect("failed to execute process");

        assert_eq!(String::from_utf8_lossy(&output.stderr), WITHOUT_ARGS_OUTPUT);
    }

    #[test]
    fn calling_shenlong_help() {
        let output = Command::new("./target/debug/shenlong")
            .arg("--help")
            .output()
            .unwrap();

        assert!(String::from_utf8_lossy(&output.stdout).contains(HELP_CLI_OUTPUT));
    }

    #[test]
    fn calling_shenlong_without_name() {
        let output = Command::new("./target/debug/shenlong")
            .args(&["--namespace", "test-space"])
            .args(&["--secret", "foo:boi"])
            .output()
            .expect("failed to execute process");

        assert_eq!(String::from_utf8_lossy(&output.stderr), WITHOUT_NAME_OUTPUT);
    }

    #[test]
    fn calling_shenlong_without_secret() {
        let output = Command::new("./target/debug/shenlong")
            .args(&["--namespace", "test-space"])
            .args(&["--name", "test-name"])
            .output()
            .expect("failed to execute process");

        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            WITHOUT_SECRET_OUTPUT
        );
    }

    #[test]
    fn calling_shenlong_without_namespace() {
        let output = Command::new("./target/debug/shenlong")
            .args(&["--secret", "FOO:BOI"])
            .args(&["--name", "test-name"])
            .output()
            .expect("failed to execute process");

        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            WITHOUT_NAMESPACE_OUTPUT
        );
    }
}
