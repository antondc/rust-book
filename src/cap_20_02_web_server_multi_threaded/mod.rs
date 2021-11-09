mod handle_stream;
pub mod threadpool;
use handle_stream::handle_stream;
use std::net::TcpListener;
use threadpool::ThreadPool;

const ITERATIONS: usize = 3;

/// # Multi threading server
///
/// http server able to handle a specific number of tasks asynchronously.
/// Important components are `ThreadPool`, `Workers`, `Tasks` and `Messages`.
///
pub fn run() {
  println!("\n• cap_20_02_web_server_multi_threaded");

  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
  let thread_pool = ThreadPool::new(4);

  match thread_pool {
    Ok(thread_pool_received) => {
      // Idle, listening for tasks
      // Will break after `ITERATIONS` cicles, quiting program
      for stream in listener.incoming().take(ITERATIONS) {
        let stream = stream.unwrap(); // Some actual task coming

        thread_pool_received.execute(|| {
          handle_stream(stream);
        });
      }
    }
    Err(e) => println!("{:?}", e),
  }
}
