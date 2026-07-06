pub struct ThreadPool {
    handles: Vec<std::thread::JoinHandle<()>>,
}
impl ThreadPool {
    pub fn new(threads_count: u8) -> Self {
        let handles=(0..threads_count).map(|_|{
            std::thread::spawn(||{})
        }).collect();
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
