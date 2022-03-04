// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkOS library.

// The snarkOS library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkOS library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkOS library. If not, see <https://www.gnu.org/licenses/>.

use snarkos::{initialize_logger, Node,
             pubsub_redis::redis_publisher,
             pubsub_redis::redis_subscriber,
             pubsub_redis::message};

use std::time::Duration;
use anyhow::Result;
use structopt::StructOpt;
use tokio::runtime;

#[tokio::main]
async fn main() -> Result<()> {
    if num_cpus::get() < 16 {
        eprintln!("\nWARNING - Your machine must have at least 16-cores to run a node.\n");
    }

    // Parse the provided arguments.
    let node = Node::from_args();

    // Start logging, if enabled.
    if !node.display {
        initialize_logger(node.verbosity, None);
    }

    let (num_tokio_worker_threads, max_tokio_blocking_threads) = (num_cpus::get(), 512); // 512 is tokio's current default

    println!("service started go channel....");

    if let Err(error) = redis_subscriber::subscribe(String::from("go_channel")) {
        println!("{:?}", error);
        panic!("{:?}", error);
    } else {
        println!("connected to queue");
    }
    std::thread::sleep(Duration::from_secs(1));
    let mut i = 0;
    while i <= 100 {
        redis_publisher::publish_message(message::PubSubMessage::new(
            message::Order::new("message from rust".to_string(), 0, i),
            "rust_channel".to_string(),
        )).unwrap();

        std::thread::sleep(Duration::from_secs(1));
        i = i + 1;
    }

    println!("service end go channel....");

    // Initialize the runtime configuration.
    let runtime = runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_stack_size(8 * 1024 * 1024)
        .worker_threads(num_tokio_worker_threads)
        .max_blocking_threads(max_tokio_blocking_threads)
        .build()?;

    let num_rayon_cores_global = num_cpus::get();

    // Initialize the parallelization parameters.
    rayon::ThreadPoolBuilder::new()
        .stack_size(8 * 1024 * 1024)
        .num_threads(num_rayon_cores_global)
        .build_global()
        .unwrap();

    runtime.block_on(async move {
        node.start().await.expect("Failed to start the node");
    });

    Ok(())
}
