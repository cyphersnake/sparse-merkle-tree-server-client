use std::net::TcpStream;

use poly_project::protocol::{bincode, Request, Response};

fn main() {
    let stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    bincode::serialize_into(
        &stream,
        &Request::UpdateLeaf {
            leaf_index: 10,
            new_data: 10,
        },
    )
    .unwrap();

    let response: Response = bincode::deserialize_from(stream).unwrap();
    match response {
        Response::Err { msg } => eprint!("error: {msg:?}"),
        Response::Updated { proof } => {
            assert!(proof.verify());
            println!("proof: {proof:?}");
        }
    }
}
