use std::thread;

#[derive(Debug)]
pub struct PoolCreationError;

pub struct ThreadPool {
    workers: Vec<Worker>,
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

        let mut workers: Vec<Worker> = Vec::with_capacity(size);

        for i in 0..size {
            workers.push(Worker::new(i));
        }

        ThreadPool { workers }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }

    // pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
    //     if size <= 0 {
    //         return Err(PoolCreationError);
    //     }

    //     let threads: Vec<thread::JoinHandle<()>> = Vec::with_capacity(size);

    //     for _ in 0..size {}

    //     Ok(ThreadPool { threads })
    // }
}

struct Worker {
    id: usize,
    join_hanlde: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker {
            id,
            join_hanlde: thread,
        }
    }
}
