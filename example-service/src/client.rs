// Copyright 2018 Google LLC
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tarpc::{client, context};
use tokio_serde::formats::Json;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let transport = tarpc::serde_transport::tcp::connect(
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000),
        Json::default,
    );
    // transport.config_mut().max_frame_length(4294967296);

    // WorldClient is generated by the service attribute. It has a constructor `new` that takes a
    // config and any Transport as input.
    let mut client =
        service::WorldClient::new(client::Config::default(), transport.await?).spawn()?;

    // The client has an RPC method for each RPC defined in the annotated trait. It takes the same
    // args as defined, with the addition of a Context, which is always the first arg. The Context
    // specifies a deadline and trace information which can be helpful in debugging requests.
    let hello = client.hello(context::current(), "Ella".to_string()).await?;

    println!("{}", hello);

    Ok(())
}
