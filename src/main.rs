use std::{borrow::Cow, env, path::PathBuf};
mod rasm;
fn main() {
    let envargs = match handle_args(env::args()) {
        Ok(f) => f,
        Err(e) => {
            println!("{}",&e);
            println!("Exiting due to above errors");
            println!("{}",USAGE);
            std::process::exit(1)
        }
    };

    rasm::run(envargs);

}

const USAGE : &'static str =
r"
USAGE:
    ./rasm-cli.exe <options> <Path to .rasm file>
OPTIONS:
    -h | --help : help
    -b | --binary : show acc and ix in binary 
    -x | --hex : show acc and ix in hexadecimal
Note:
    Vertical bar '|' means 'or'

";
pub enum DisplayStyle {
    Denary,
    Binary,
    Hex,
}
pub struct EnvArgs {
    file : PathBuf,
    style : DisplayStyle,

}
fn handle_args<'a>(mut args : env::Args) -> Result<EnvArgs,Cow<'static,str>> {
    
    let mut file = PathBuf::default();
    let mut style = DisplayStyle::Denary;
    args.next().unwrap();
    for arg in args {
        if arg.starts_with("-") {
            style = match &arg[1..] {
                "h" | "-help" => {
                    println!("{}",USAGE);
                    std::process::exit(1);
                },
                "b" | "-bin"  => {
                    DisplayStyle::Binary
                },
                "x" | "-hex"  => {
                    DisplayStyle::Hex
                },
                _   => {
                    return Err(Cow::Borrowed("Unkown command line flag"));
                    
                }
            }
        }else if arg.ends_with(".rasm") || arg.ends_with(".asm") {
            file = arg.into();
        }
        
    }
    if !file.exists() {
        Err(Cow::Borrowed("File specified could not be found or unknown commandline argument"))
    }else {
        Ok(EnvArgs {file,style})
    }

}
