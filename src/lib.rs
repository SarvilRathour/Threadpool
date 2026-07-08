use std::sync::mpsc::{channel,Sender};
use std::sync::Mutex;
use std::sync::Arc;

pub struct ThreadPool{
    handles:Vec<std::thread::JoinHandle<()>>,
    sender:Sender<Box<dyn Fn()+Send>>,
}
impl ThreadPool{
    fn new(thread_count:u8)->Self{
        let mut handles=vec![];
        let (sender,reciever)=channel::<Box<dyn Fn()+Send>>();
        let reciever=Arc::new(Mutex::new(reciever));
        for _ in 0..thread_count{
            let clone=Arc::clone(&reciever);
            let handle=std::thread::spawn(move||{
                let work=clone.lock().unwrap().recv().unwrap();
                work();
            });
            handles.push(handle);
        }
        Self{
            handles,
            sender
        }
    }
    fn execute<T:Fn()+Send+'static>(&self,work:T){
        self.sender.send(Box::new(work)).unwrap();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let pool=ThreadPool::new(10);
        pool.execute(||println!("Thread executing "));
    }
}
