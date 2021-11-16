use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread::JoinHandle;
use std::thread::{self};
use std::time::Duration;

enum ThreadState<T, F> {
    Waiting(F),
    Running(JoinHandle<T>, Arc<AtomicBool>),
    Finished(T),
}

impl<T, F> ThreadState<T, F>
where
    T: Send + 'static,
    F: std::ops::FnOnce() -> T + Send + 'static,
{
    fn set_running(&mut self) {
        take_mut::take(self, |state| match state {
            Self::Waiting(closure) => {
                let is_running = Arc::new(AtomicBool::new(true));
                let is_running_cloned = Arc::new(AtomicBool::new(true));
                let wrapper = move || {
                    let result = closure();
                    is_running_cloned.store(false, Ordering::SeqCst);
                    result
                };
                Self::Running(thread::spawn(wrapper), is_running)
            }
            _ => unreachable!(),
        })
    }

    fn set_finished(&mut self) {
        take_mut::take(self, |state| match state {
            Self::Running(handle, _) => {
                Self::Finished(handle.join().expect("Failed to join thread"))
            }
            _ => unreachable!(),
        })
    }

    fn new_running(closure: F) -> Self {
        let mut state = Self::new_waiting(closure);
        state.set_running();
        state
    }

    fn new_waiting(closure: F) -> Self {
        Self::Waiting(closure)
    }

    fn is_completed(&self) -> bool {
        match self {
            Self::Running(_, running) => running.load(Ordering::SeqCst),
            _ => unreachable!(),
        }
    }

    fn unwrap_result(self) -> T {
        match self {
            Self::Finished(result) => result,
            _ => unreachable!(),
        }
    }
}

pub struct ThreadPool<T, F> {
    threads: Vec<ThreadState<T, F>>,
    max_num_threads: usize,
}

impl<T, F> ThreadPool<T, F>
where
    T: Send + 'static,
    F: std::ops::FnOnce() -> T + Send + 'static,
{
    pub fn new(max_num_threads: usize) -> Self {
        Self {
            threads: vec![],
            max_num_threads,
        }
    }

    fn num_running(&self) -> usize {
        self.threads
            .iter()
            .filter(|thread| matches!(thread, ThreadState::Running(..)))
            .count()
    }

    fn get_waiting_mut(&mut self) -> impl Iterator<Item = &mut ThreadState<T, F>> {
        self.threads
            .iter_mut()
            .filter(|thread| matches!(thread, ThreadState::Waiting(..)))
    }

    fn get_running_mut(&mut self) -> impl Iterator<Item = &mut ThreadState<T, F>> {
        self.threads
            .iter_mut()
            .filter(|thread| matches!(thread, ThreadState::Running(..)))
    }

    fn get_finished_indices(&'_ self) -> impl Iterator<Item = usize> + '_ {
        self.threads
            .iter()
            .enumerate()
            .filter(|(_, thread)| matches!(thread, ThreadState::Finished(..)))
            .map(|(num, _)| num)
    }

    pub fn add_job(&mut self, closure: F)
    where
        F: std::ops::FnOnce() -> T + Send + 'static,
    {
        if self.num_running() < self.max_num_threads {
            self.threads.push(ThreadState::new_running(closure));
        } else {
            self.threads.push(ThreadState::new_waiting(closure))
        }
    }

    fn update(&mut self) -> Option<T> {
        let mut num_finished = 0;
        for running_thread in self.get_running_mut() {
            if running_thread.is_completed() {
                running_thread.set_finished();
                num_finished += 1;
            }
        }
        for _ in 0..num_finished {
            if self.num_running() >= self.max_num_threads {
                break;
            }
            let waiting_thread = self.get_waiting_mut().next();
            if let Some(waiting_thread) = waiting_thread {
                waiting_thread.set_running();
            }
        }
        let index = self.get_finished_indices().next()?;
        let finished_thread = self.threads.remove(index);
        return Some(finished_thread.unwrap_result());
    }
}

impl<T, F> Iterator for ThreadPool<T, F>
where
    T: Send + 'static,
    F: std::ops::FnOnce() -> T + Send + 'static,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let finished_thread = self.update();
            if let Some(finished_thread) = finished_thread {
                return Some(finished_thread);
            } else if self.threads.len() == 0 {
                return None;
            }
            thread::sleep(Duration::from_millis(50));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ThreadPool;

    #[test]
    fn thread_pool() {
        let mut pool = ThreadPool::new(10);
        for i in 1..50 {
            pool.add_job(move || i);
        }
        let _: Vec<_> = pool.collect();
    }
}
