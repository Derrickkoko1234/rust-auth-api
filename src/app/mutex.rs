use super::mongo::AppInstance;
use scopeguard::defer;

impl AppInstance{
    pub fn mutexer<T,F: FnOnce() -> T>(&self,closure: F)->T{
        let l = self.mutex.try_lock().expect("Failed to lock mutex");
        defer! {
            drop(l);
        }
        closure()
    }
}