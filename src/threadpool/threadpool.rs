use std::thread;
use std::thread::JoinHandle;
use std::sync::{mpsc, Arc, Mutex};

struct Worker {
    id : usize,
    thread: JoinHandle<()>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                receiver.lock().unwrap().recv().unwrap()();
                // 这就相当于执行了闭包，并且是执行的死循环操作
            }
        });
        Worker {
            id,
            thread
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}


impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();

        // 因为需要把receiver分配给多个具体的线程，又为了保证线程安全，所以加了Mutex锁控制
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0 .. size {
            // 这里创建的线程数
            workers.push(Worker::new(id, Arc::clone(&receiver)));
            // 这里直接写receiver肯定是有问题的，因为第一次所有权转移了，后面就无法使用了，但是又必须每个worker都得使用，可以使用Rc
            // 使用Rc不能保证多线程场景下的线程安全，所以还得是Arc，具备原子性（需要在线程直接传递数据的必须实现Send 特质，但是恰恰Rc没有使用，所以编译器就会被发现错误
            // 为了竞争，还得加上Mutex
            // 这样多个worker就能做到共享同一个receiver，通过竞争式的从receiver获取对应的job数据
        }

        ThreadPool {
            workers,
            sender
        }
    }

    pub fn execute<F>(&self, f : F)
        where F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
        // 这里可以随机的由线程循环获取到任务么？还是说使用channel，单生产多消费的模式竞争
        // 确定了就是使用channel，往sender中丢数据，丢的应该也是闭包，类似java的Runnable
    }
}