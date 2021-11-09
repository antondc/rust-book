use super::threadpool_error::ThreadPoolError;

pub struct Pool;
pub struct ThreadPool;

/// Create a new thread pool
///
/// The size is the number of threads in the pool.
///
/// # Panics
///
/// The `new` function will panic if the size is zero.
impl ThreadPool {
  pub fn new(size: usize) -> Result<ThreadPool, ThreadPoolError> {
    if size <= 0 {
      let error = ThreadPoolError(String::from("ThreadPool size not valid"));
      return Err(error);
    }

    Ok(ThreadPool)
  }

  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
    // Pending implementation
  }
}
