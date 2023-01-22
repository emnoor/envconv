use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_enum, default_value_t = Type::Deployment)]
    type_: Type,

    #[arg(default_value_t = String::from(".env"))]
    file: String,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Type {
    Configmap,
    Deployment,
}

fn main() {
    let cli = Cli::parse();

    let items = dotenvy::from_path_iter(cli.file).unwrap();
    for item in items {
        if item.is_err() {
            continue;
        }
        let (key, val) = item.unwrap();

        match cli.type_ {
            Type::Configmap => println!("{key}: {val:?}"),
            Type::Deployment => println!("- name: {key:?}\n  value: {val:?}"),
        };
    }
}
