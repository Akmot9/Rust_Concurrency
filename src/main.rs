mod async_mutex;
mod mutex;
mod fifo;

fn main() {
    println!("Asynchronous mutex:");
    async_mutex::main();
    println!("");
    println!("Simple mutex:");
    mutex::main();
    println!("");
    println!("FIFO:");
    fifo::main();

}
