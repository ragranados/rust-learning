use std::thread;

#[derive(Debug)]
pub struct PoolCreationError;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// ## Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut threads: Vec<thread::JoinHandle<()>> = Vec::with_capacity(size);

        for _ in 0..size {}

        ThreadPool { threads }
    }

    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size <= 0 {
            return Err(PoolCreationError);
        }

        let threads: Vec<thread::JoinHandle<()>> = Vec::with_capacity(size);

        for _ in 0..size {
            todo!()
        }

        Ok(ThreadPool { threads })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
