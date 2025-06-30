pub struct ThreadPool;

impl ThreadPool {
    //create a new ThreadPool
    //the size is the number of threads in the pool
    //#panics
    //The 'new' function will panix if the size is zero
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        ThreadPool
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
