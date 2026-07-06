use std::sync::mpsc::channel;
use std::sync::Mutex;
use std::sync::Arc;
pub struct ThreadPool {
    handles: Vec<std::thread::JoinHandle<()>>,
}
impl ThreadPool {
    pub fn new(threads_count: u8) -> Self {
        let (sender,receiver)=channel::<Box<dyn Fn()+Send>>();
        let receiver=Arc::new(Mutex::new(receiver));
        let mut handles=vec![];
            for _ in 0..threads_count{
            let clone=receiver.clone();
            let handle=std::thread::spawn(move||{
                loop{
                    let work= clone.lock().unwrap().recv().unwrap();
                    work();
                }
            });
            handles.push(handle);
        }
        Self { handles }
    }
    pub fn execute<T: Fn()>(&self, work: T) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let pool = ThreadPool::new(10);
        pool.execute(|| println!("Hello from the 1st thread"));
        pool.execute(|| println!("Hello from the 2nd thread"));
    }
}
