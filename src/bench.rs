#[macro_use]
extern crate bencher;
use tonic;
use tokio::runtime::{Builder};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use bencher::Bencher;

use tokio::net::UnixStream;
use tonic::transport::{Uri};
use tower::service_fn;


pub mod echo {
    tonic::include_proto!("echo");
}
use echo::echo_client::EchoClient;
use echo::{EchoRequest};

fn random_string(n: usize) -> String {
    thread_rng().sample_iter(&Alphanumeric)
                .take(n)
                .collect()
}

macro_rules! bench {
    ($name:ident, $message_size:expr, $count:expr) => {
        fn $name(b: &mut Bencher) {
            let mut rt = Builder::new()
                .basic_scheduler()
                .enable_all()
                .build()
                .expect("runtime");
            
            let message = random_string($message_size);

            b.iter(|| {
                for _ in 0..$count {
                    let _result = rt.block_on( async {
                                    let mut client = EchoClient::connect("http://[::1]:50051").await?;
                                    // let channel = tonic::transport::Endpoint::from_static("lttp://[::]:50051")
                                    //                 .connect_with_connector(service_fn(|_: Uri| {
                                    //                     let path = "/tmp/tonic/helloworld";

                                    //                     // Connect to a Uds socket
                                    //                     UnixStream::connect(path)
                                    //                 }))
                                    //                 .await?;
                                    // let mut client = EchoClient::new(channel);
                                    let request = tonic::Request::new(EchoRequest {
                                        message: message.clone(),
                                    });
                                    client.unary_echo(request).await?;
                                    Ok::<(), Box<dyn std::error::Error>>(())
                                });
                    }
                });
        }
    }
}

bench!(message_size_1kb, 1024, 1);
bench!(message_size_100kb, 100 * 1024, 1);
bench!(message_size_1mb, 1000 * 1024, 1);

benchmark_group!(message_size, message_size_1kb, message_size_100kb, message_size_1mb);

benchmark_main!(message_size);