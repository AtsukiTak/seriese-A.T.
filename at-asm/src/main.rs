use at_asm::Assembler;
use std::{fs::OpenOptions, path::PathBuf, process::exit};

pub fn main() {
    let config = Config::init();

    let mut input_file = OpenOptions::new()
        .read(true)
        .open(&config.input_path)
        .unwrap();

    let mut output_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&config.output_path)
        .unwrap();

    Assembler::new()
        .read_from(&mut input_file)
        .write_into(&mut output_file);
}

struct Config {
    output_path: PathBuf,
    input_path: PathBuf,
}

impl Config {
    fn init() -> Self {
        let mut args = std::env::args().collect::<Vec<_>>();

        // "-o" flagで指定されたoutput file
        // もしあればそれらをargsから取り除く
        let output_file_option = match args.iter().position(|arg| arg == "-o") {
            Some(opt_idx) => {
                args.remove(opt_idx);
                // "-o" flagの次のarg
                let path_str = args.remove(opt_idx);
                Some(PathBuf::from(path_str))
            }
            None => None,
        };

        // input file
        // 他のオプション類を消した後に残るもの
        let input_path = {
            if args.len() < 2 {
                eprintln!("error: input file is not specified");
                exit(1);
            }
            if args.len() > 2 {
                eprintln!("error: invalid command format");
                exit(1);
            }
            let last_arg = args.remove(1);
            PathBuf::from(last_arg)
        };

        // output file
        let output_path = match output_file_option {
            Some(f) => f,
            None => {
                // default output file
                // input fileの拡張子だけ変えたもの
                input_path.with_extension("o")
            }
        };

        Config {
            output_path,
            input_path,
        }
    }
}
