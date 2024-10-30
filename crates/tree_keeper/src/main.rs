use std::net::TcpListener;

use clap::Parser;
use tracing::*;

use poly_project::{
    protocol::{bincode, Request, Response},
    Tree,
};

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
        } => Response::Updated {
            proof: Box::new(tree.update_leaf(leaf_index, new_data)),
        },
    }
}
