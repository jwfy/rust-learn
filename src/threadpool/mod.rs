pub mod threadpool;
// 这个threadpool指的是其文件夹下面的threadpool.rs 文件

// 主要参考的线上地址是 https://www.bookstack.cn/read/trpl-zh-cn/src-ch20-02-multithreaded.md

// 其刨去具体的实现语法细节，和一般的线程池实现类似，有一组worker工作现场，然后最外界的丢入一个闭包任务即可，就java的runnable