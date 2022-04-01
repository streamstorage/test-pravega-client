use std::env;
use std::time::Instant;

use pravega_client::client_factory::ClientFactory;
use pravega_client_config::ClientConfigBuilder;
use pravega_client_shared::{Scope, Stream, StreamConfiguration, ScopedStream, Scaling, ScaleType};


fn main() {
    let args: Vec<String> = env::args().collect();
    let host = if args.len() > 1 {
        args[1].clone()
    } else {
        "127.0.0.1:9090".to_string()
    };
    let scope = if args.len() > 2 {
        args[2].clone()
    } else {
        "hello".to_string()
    };
    let stream = if args.len() > 3 {
        args[3].clone()
    } else {
        "world".to_string()
    };
    let client_config = ClientConfigBuilder::default()
        .controller_uri(host)
        .build()
        .expect("creating config");
    let client_factory = ClientFactory::new(client_config);
    let runtime = client_factory.runtime();
    let scope = Scope::from(scope);
    let stream = Stream::from(stream);
    let controller_client = client_factory.controller_client();
    let scoped_stream = ScopedStream {
        scope: scope.clone(),
        stream: stream.clone(),
    };
    let stream_config = StreamConfiguration {
        scoped_stream: ScopedStream {
            scope: scope.clone(),
            stream: stream.clone(),
        },
        scaling: Scaling {
            scale_type: ScaleType::FixedNumSegments,
            min_num_segments: 1,
            ..Default::default()
        },
        retention: Default::default(),
        tags: None,
    };

    runtime.block_on(async move{
        let res = controller_client.create_scope(&scope).await.expect("create scope");
        println!("create scope {}: {}", scope.to_string(), res);
        let res = controller_client.create_stream(&stream_config).await.expect("create stream");
        println!("create stream {}: {}", stream.to_string(), res);
    });

    let data: Box<[u8; 1048576]> = Box::new([0; 1048576]);
    let request_num = 1024;
    
    let mut writer = runtime.block_on(client_factory.create_byte_writer(scoped_stream));
    runtime.block_on(writer.seek_to_tail());
    
    let start = Instant::now();
    runtime.block_on(async move {
        for _ in 0..request_num {
            writer.write(data.as_ref()).await.expect("write event");
        }
        writer.flush().await.expect("flush");
    });
    let duration = start.elapsed();
    println!("in total: {:?}ms, ms/req: {}", duration.as_millis(), duration.as_millis() as f64 / request_num as f64);
}
