// This node routes a message, to a worker on a different node, over the tcp transport.

use ockam::{node, route, Context, Result, TcpConnectionOptions};
use ockam_transport_tcp::TcpTransportExtension;

#[ockam::node]
async fn main(ctx: Context) -> Result<()> {
    // Create a node with default implementations
    let mut node = node(ctx);

    // Initialize the TCP Transport.
    let tcp = node.create_tcp_transport().await?;

    // Create a TCP connection to a different node.
    let connection_to_responder = tcp.connect("localhost:4000", TcpConnectionOptions::new()).await?;

    // Send a message to the "echoer" worker on a different node, over a tcp transport.
    // Wait to receive a reply and print it.
    let r = route![connection_to_responder, "echoer"];
    let reply = node.send_and_receive::<String>(r, "Hello Ockam!".to_string()).await?;

    println!("App Received: {}", reply); // should print "Hello Ockam!"

    // Stop all workers, stop the node, cleanup and return.
    node.stop().await
}
