use std::thread;

#[no_mangle]
pub extern fn process() {
    let handles: Vec<_> = (0..10).map(|_| {
        thread::spawn(|| {
            let mut x = 0;
            for _ in (0..5_000_000) {
                x += 1
            }
            x
        })
    }).collect();

    for handle in handles {
        println!("Thread finished with count={}", handle.join().map_err(|_| "Could not join a thread!").unwrap());
    }

    println!("done!");
}
