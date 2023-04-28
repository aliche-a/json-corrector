use clap::Parser;

mod chunker;

#[derive(Parser, Debug)]
struct IoArgs {
    filepath: Option<String>,
    #[arg(long)]
    input: Option<String>,
    #[arg(long)]
    output: Option<String>,
}

impl IoArgs {
    pub fn file_override(&self) -> Option<String> {
        if self.input.is_some() {
            self.input.clone()
        } else if self.filepath.is_some() {
            self.filepath.clone()
        } else {
            None
        }
    }
}

fn main() {
    let args = IoArgs::parse();

    let mut chunker = if let Some(filepath) = args.file_override() {
        chunker::Chunker::from_file(&filepath).expect("Failed to initialize chunker")
    } else {
        chunker::Chunker::new().expect("Failed to initialize chunker")
    };

    match chunker.process() {
        Ok(_) => {
            if args.output.is_some() {
                if let Err(e) = chunker.write_to_file(&args.output.unwrap()) {
                    println!("Failed to write file contents to file: {}", e);
                } else {
                    println!("Successfully wrote results to file");
                }
            } else {
                println!("{}", chunker.output().unwrap());
            }
        },
        Err(e) => println!("Failed to process JSON: {:?}", e),
    }
}
