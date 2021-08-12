mod threadpool;
use threadpool::threadpool::ThreadPool;
// 这两个是在一起的，通过第一个mod操作，导入threadpool文件夹中的mod.rs包
// mod threadpool 具体有两种含义，一种是导入当前相对路径下面的threadpool.rs 文件，再一种是导入当前相对路径下面的threadpool文件夹下面的mod.rs 文件
// mod文件可以依次管理其文件夹下面的各个文件以及需要对外pub的功能点
// 然后利用use具体的引入需要被调用的方法


mod list;

fn main() {
    let pool = ThreadPool::new(23);
    pool.execute(|| {
        println!("{}", "sdfsdfs");
    });
}