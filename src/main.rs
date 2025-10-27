use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
    sync::mpsc,
    thread,
    time::Instant,
};

fn main() {
    let input_file_name = "input.txt";
    let output_file_name = "output.txt";
    let start_time = Instant::now();
    let (tx_input, rx_input) = mpsc::channel();

    let (tx_output, rx_output) = mpsc::channel::<i128>();
    // read from file is sloww it is good to have this on the separte thead
    let read_builder = thread::Builder::new().name("ReadThread".into());
    let read_thread = read_builder.spawn(move || {
        let handle = thread::current();
        // println!("{:?}", handle.name());
        let input_file = File::open(input_file_name).unwrap();
        let read = BufReader::new(input_file);
        for line in read.lines() {
            // println!("Reading line '{:?}'", line);
            tx_input
                .send(line.unwrap().parse::<i128>().unwrap())
                .unwrap();
        }
    });

    let write_builder = thread::Builder::new().name("WriteThread".into());
    // Write to File is Also Slow so use Separted Therad
    let writing_thread = write_builder.spawn(move || {
        let handle = thread::current();
        // println!("{:?}", handle.name());
        let output_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(output_file_name)
            .unwrap();
        let mut writer = BufWriter::new(output_file);

        for num in rx_output {
            let num_str = num.to_string();
            writer.write_all(num_str.as_bytes()).unwrap();
            writer.write_all(b"\n").unwrap();
            // println!(" [!] Wrote num {}", num);
        }
        writer.flush().unwrap();
    });

    for num in rx_input {
        // println!("[+] Reciverd message: '{}'.", num);
        let compute_nums = num * num * num * num;

        tx_output.send(compute_nums).unwrap();
    }
    // Need to drop tx_output hear, otherwise the loop over rx_output in write_tread will never end
    // Or Fins
    drop(tx_output);

    read_thread.expect("REASON Unknown").join().unwrap();
    writing_thread.expect("REASON UnKnown").join().unwrap();
    // End measuring the execution time.
    let duration = start_time.elapsed();

    println!("\nProcessing finished.");
    println!("Total execution time: {:?}", duration);
}
