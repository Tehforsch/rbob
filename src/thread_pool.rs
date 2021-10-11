use std::thread::JoinHandle;
use std::thread::{self};

pub struct ThreadPool<T> {
    handles: Vec<JoinHandle<T>>,
    max_num_threads: usize,
}

impl<T> ThreadPool<T>
where
    T: Send + 'static,
{
    pub fn new(max_num_threads: usize) -> Self {
        Self {
            handles: vec![],
            max_num_threads,
        }
    }

    pub fn add_job<F>(&mut self, closure: F)
    where
        F: std::ops::FnOnce() -> T + Send + 'static,
    {
        self.handles.push(thread::spawn(closure));
    }

    pub fn get_results(self) -> Vec<T> {
        self.handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .collect()
    }
}
