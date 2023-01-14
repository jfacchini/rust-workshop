fn main() {
    let pool = ThreadPool::new(10);

    pool.execute(|| {
        println!("Hello from thread");
    });
}

struct ThreadPool {}

impl ThreadPool {
    fn new(number_of_threads: u8) -> Self {
        todo!()
    }
}
