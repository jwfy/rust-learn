mod threads {
    use std::thread;
    use std::cell::RefCell;
    use std::thread::spawn;
    use std::io::Read;


    pub fn test1() {
        thread_local! {
            static Foo : RefCell<u32> = RefCell::new(1)
        };
        // 会生成一个thread::LocalKey的Foo实例

        Foo.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 2;
            // 可以传入一个闭包，然后操作线程本地存储的数据
        });

        thread::spawn(|| {
            Foo.with(|f| {
                assert_eq!(*f.borrow(), 1);
                *f.borrow_mut() = 3;
            })
        });

        Foo.with(|f| {
            assert_eq!(*f.borrow(), 2);
        });
    }

    pub fn test2() {
        use std::sync::mpsc::channel;
        use std::path::PathBuf;
        use std::fs::File;

        let documents: Vec<PathBuf> = Vec::new();

        let (sender, receiver) = channel();
        let handle = spawn(move || {
            for filename in documents {
                let mut f = File::open(filename).unwrap();
                let mut text = String::new();
                f.read_to_string(&mut text).unwrap();

                if sender.send(text).is_err() {
                    break;
                }
            }
            Ok(())
        });

    }
}

pub fn test() {
    threads::test1();
}