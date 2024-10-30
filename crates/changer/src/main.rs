use std::{net::TcpStream, process};

use clap::Parser;
use poly_project::{
    protocol::{bincode, Request, Response},
    Data,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "127.0.0.1:7878")]
    host: String,

    #[arg(short, long)]
    leaf_index: u32,
    #[arg(short, long)]
    new_data: Data,

    #[arg(short, long)]
    show_proof: bool,
}

fn main() {
    let Args {
        host,
        leaf_index,
        new_data,
        show_proof,
    } = Args::parse();

    let stream = match TcpStream::connect(host) {
        Ok(stream) => stream,
        Err(err) => {
            eprintln!("error while connect with: {err:?}");
            process::exit(1);
        }
    };

    if let Err(err) = bincode::serialize_into(
        &stream,
        &Request::UpdateLeaf {
            leaf_index,
            new_data,
        },
    ) {
        eprint!("err while serialize requeset: {err:?}");
        process::exit(1);
    }

    match bincode::deserialize_from(stream) {
        Ok(Response::Updated { proof }) => {
            if !proof.verify() {
                eprintln!("error while verify proof from the tree keeper");
                process::exit(1);
            } else {
                let old_root = proof.root().old;
                let new_root = proof.root().new;
                if show_proof {
                    println!("change tree successfully {old_root} -> {new_root}: {proof:?}");
                } else {
                    println!("change tree successfully {old_root} -> {new_root}");
                }
            }
        }
        Ok(Response::Err { msg }) => {
            eprint!("error: {msg:?}");
            process::exit(1);
        }
        Err(err) => {
            eprint!("error while handle request: {err:?}");
            process::exit(1);
        }
    }
}
