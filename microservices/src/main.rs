use std::{env, path::Path, process::Command};

const WORKSPACES: &[&str] = &["auth", "users", "jwt-authorizer"];

// For building and deploying to AWS
// Requires a [lab] credentials in aws config
fn main() {
    dotenv::dotenv().unwrap();

    let manifest = env::var_os("CARGO_MANIFEST_DIR").unwrap();

    let manifest_dir = Path::new(&manifest);

    let mut args = env::args().skip(1);
    let iam_role = args.next();

    Command::new("cargo")
        .args(&["lambda", "build", "--workspace"])
        .current_dir(manifest_dir)
        .status()
        .unwrap();

    for workspace in WORKSPACES {
        let mut deploy = Command::new("cargo");
        deploy
            .args(&["lambda", "deploy", &format!("gamehub-{workspace}")])
            .args(["--profile", "lab"]);

        if let Some(iam_role) = &iam_role {
            println!("Deploying with IAM role: {iam_role}");
            deploy.args(["--iam-role", &iam_role]);
        }

        deploy.current_dir(manifest_dir).status().unwrap();
    }
}
