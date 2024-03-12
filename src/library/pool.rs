use threadpool::ThreadPool;
use std::sync::{Mutex, Arc};

use super::constant::N_WORKERS;
use std::marker::Send;

#[derive(Debug, Clone)]
pub struct Pool{
    pub pool: Arc<Mutex<ThreadPool>>
}

impl Pool {
    pub fn new()->Self{
        Self{
            pool:  Arc::new(Mutex::new(ThreadPool::new(N_WORKERS)))
        }
    }

    pub fn run_job<F: FnOnce()->() + Send + 'static>(&self,closure: F){
        self.pool.lock().unwrap().execute(closure);
    }
}