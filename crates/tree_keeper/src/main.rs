use std::net::TcpListener;

use clap::Parser;
use tracing::*;

use sparse_merkle_tree_server_client::{
    protocol::{bincode, Request, Response},
    Tree,
};

/// Starts tree_keeper single-thread server which creates a sparse-merkle-tree and accepts
/// leaf-change-requests from external connections
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "127.0.0.1:7878")]
    host: String,
}

fn main() {
    tracing_subscriber::fmt::init();

    let Args { host } = Args::parse();

    let listener = TcpListener::bind(host).unwrap();

    let mut tree = Tree::default();

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            Err(err) => {
                error!("error while handle stream: {err:?}; continue listening");
                continue;
            }
        };

        let response = match bincode::deserialize_from(&stream) {
            Ok(request) => handle_request(&mut tree, request),
            Err(err) => {
                let msg = format!("error while parsing request: {err:?}");
                error!(msg);

                Response::Err { msg }
            }
        };

        match bincode::serialize_into(stream, &response) {
            Ok(()) => {
                info!("successfully processed the connection");
            }
            Err(err) => {
                error!("error while encode respose: {err:?}; continue listening");
                continue;
            }
        }
    }
}

fn handle_request(tree: &mut Tree, reqeust: Request) -> Response {
    match reqeust {
        Request::UpdateLeaf {
            leaf_index,
            new_data,
        } => {
            let proof = tree.update_leaf(leaf_index, new_data);
            info!(
                "tree updated: {old} -> {new}",
                old = proof.root().old,
                new = proof.root().new
            );
            Response::Updated {
                proof: Box::new(proof),
            }
        }
    }
}
