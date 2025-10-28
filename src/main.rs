use reqwest::StatusCode;
use std::time::{Duration, Instant};
use tokio::time::sleep;
async fn heartbeat(mut num: u32) {
    loop {
        println!("Beating ...{}", num);
        sleep(Duration::from_millis(25)).await;
        num += 1;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    tokio::spawn(heartbeat(0));
    // Waits on multiple concurrent branches, returning when all branches complete.
    // The join! macro must be used inside of async functions, closures, and blocks.
    // The join! macro takes a list of async expressions and evaluates them concurrently on the same task. Each async expression evaluates to a future and the futures from each expression are multiplexed on the current task.
    let (status_one, status_two) = tokio::join!(
        get_status("https://docs.rs/reqwest/latest/reqwest/index.html"),
        get_status("https://docs.rs/dioxus/latest/dioxus/struct.LaunchBuilder.html")
    );
    // Waits on multiple concurrent branches, returning when the first branch completes, cancelling the remaining branches.
    // The select! macro must be used inside of async functions, closures, and blocks.
    // The select! macro accepts one or more branches with the following pattern:
    // <pattern> = <async expression> (, if <precondition>)? => <handler>,
    //`
    // tokio::select! {
    //   status = get_status("https://docs.rs/reqwest/latest/reqwest/index.html") => println!("[+] Status 1 {:?}", status),
    //   status = get_status("https://docs.rs/dioxus/latest/dioxus/struct.LaunchBuilder.html") => println!("[+] Status 1 {:?}", status),
    // };
    // `

    println!("[+] Status 1 {:?}", status_one);
    println!("[+] Status 2 {:?}", status_two);
    println!(
        "[+] Overall execution time: {}ms",
        start_time.elapsed().as_millis()
    );
    Ok(())
}

async fn get_status(url: &str) -> Result<StatusCode, Box<dyn std::error::Error>> {
    let start_time = Instant::now();

    let status_code = reqwest::get(url).await?.status();
    let duration = start_time.elapsed().as_millis();
    println!("took {}ms to feach url [+] `{}`", duration, url);
    Ok(status_code)
}
