use std::{borrow::Cow, env, fs::File, path::Path};
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
r"USAGE:
./rasm-cli.exe <Path to .rasm file>";

fn handle_args(args : Vec<String>) -> Result<File,Cow<'static,str>> {
    if args.len() - 1 != 1 {
        return Err(Cow::Borrowed("Path to rasm file must be specified!"));
    }
    let file_path : &Path = Path::new(&args[1]); // since first element is the executables name
    if !file_path.exists() {
        return Err(Cow::Owned(format!("{} does not exist!",file_path.to_str().unwrap())));
    }
    return Ok(File::open(file_path).expect("Could not open file"))
}
