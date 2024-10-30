use std::{io, net::TcpStream};

use clap::Parser;
use poly_project::{
    protocol::{bincode, Request, Response},
    Data, Proof,
};

/// The command connects to tree-keeper and updates the sparse-merkle-tree
/// Gets back proof that the tree has been updated
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "127.0.0.1:7878")]
    host: String,

    /// Leaf index to be updated
    #[arg(short, long)]
    leaf_index: u32,
    /// New merkle-tree leaf data
    #[arg(short, long)]
    new_data: Data,

    /// Show the full proof received from the tree keeper
    #[arg(short, long)]
    show_proof: bool,
}

// `dead_code` because rust doesn't realize that `main` returning this type
#[allow(dead_code)]
#[derive(Debug)]
enum Error {
    CantConnectToTreeKeeper(io::Error),
    WhileSerializeRequest(bincode::Error),
    WhileDeserializeResponse(bincode::Error),
    CorruptedProofReceived(Box<Proof>),
    ServerSide(String),
}

fn main() -> Result<(), Error> {
    let Args {
        host,
        leaf_index,
        new_data,
        show_proof,
    } = Args::parse();

    let stream = TcpStream::connect(host).map_err(Error::CantConnectToTreeKeeper)?;

    bincode::serialize_into(
        &stream,
        &Request::UpdateLeaf {
            leaf_index,
            new_data,
        },
    )
    .map_err(Error::WhileSerializeRequest)?;

    match bincode::deserialize_from(stream).map_err(Error::WhileDeserializeResponse)? {
        Response::Updated { proof } => {
            if !proof.verify() {
                Err(Error::CorruptedProofReceived(proof))
            } else {
                let old_root = proof.root().old;
                let new_root = proof.root().new;

                if show_proof {
                    println!("change tree successfully {old_root} -> {new_root}: {proof:?}");
                } else {
                    println!("change tree successfully {old_root} -> {new_root}");
                }

                Ok(())
            }
        }
        Response::Err { msg } => Err(Error::ServerSide(msg)),
    }
}
