use std::{borrow::Cow, env, path::Path};
mod rasm;
fn main() {
    let args : Vec<_> = env::args().collect();
    let file = match handle_args(args) {
        Ok(f) => f,
        Err(e) => {
            println!("{}",&e);
            println!("Exiting due to above errors");
            println!("{}",USAGE);
            std::process::exit(1)
        }
    };
    rasm::run(file);

}

const USAGE : &'static str =
r"
USAGE:
    ./rasm-cli.exe <options> <Path to .rasm file>
OPTIONS:
    -h : help
";

fn handle_args<'a>(args : Vec<String>) -> Result<&'a Path,Cow<'static,str>> {
    if args.len() - 1 != 1 {
        return Err(Cow::Borrowed("Path to rasm file must be specified!"));
    }
    let file_path : &Path = Path::new(&args[1]); // since first element is the executables name
    if !file_path.exists() {
        return Err(Cow::Owned(format!("{} does not exist!",file_path.to_str().unwrap())));
    }
    return Ok(file_path.clone())
}
