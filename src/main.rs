use std::{
    env,
    os::unix::process::CommandExt,
    process::{self, Command},
};

fn main() {
    let current = match env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            eprintln!("Failed to get current directory: {}", err);
            process::exit(1);
        }
    };

    for dir in current.ancestors() {
        let candidate = dir.join("x.py");

        if candidate.exists() {
            let error = Command::new(&candidate)
                .args(env::args())
                .current_dir(dir)
                .exec();

            eprintln!("Failed to invoke `{}`: {}", candidate.display(), error);
            process::exit(1);
        }
    }

    eprintln!(
        "x.py not found. Please run inside of a checkout of `https://github.com/rust-lang/rust`."
    );
    process::exit(1);
}
