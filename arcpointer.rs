use std::sync::Arc;

trait Logger {
    fn log(&self, msg: &str);
}

struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, msg: &str) {
        println!("{}", msg);
    }
}

fn main() {
    let logger: Arc<dyn Logger + Send + Sync> = Arc::new(ConsoleLogger);
    let logger2 = Arc::clone(&logger);

    std::thread::spawn(move || {
        logger2.log("Hello from another thread");
    })
    .join()
    .unwrap();

    logger.log("hello from main");
}
