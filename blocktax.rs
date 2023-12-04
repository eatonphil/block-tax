use std::os::unix::prelude::FileExt;

const ATTEMPTS: usize = 50;

fn drop_pagecache() {
    std::process::Command::new("bash")
	.arg("-c")
	.arg("free && sync && (echo 3 | sudo tee /proc/sys/vm/drop_caches) && free")
	.stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
	.spawn()
	.unwrap();
}

fn main() {
    let file = std::fs::File::options()
        .create(true)
        .write(true)
	.truncate(true)
        .open("test.dat")
        .expect("Could not open data file.");

    let page_sizes: [usize; 2] = [512, 4096];

    for page_size in page_sizes {
	for _ in 0..ATTEMPTS {
	    let mut partial_buffer: [u8; 20] = [0; 20];
	    for i in 0..partial_buffer.len() {
		partial_buffer[i] = (i % 256) as u8;
	    }

	    for _ in 0..ATTEMPTS {
		file.set_len(0).unwrap();
		drop_pagecache();

		let t1 = std::time::Instant::now();
		file.write_all_at(&partial_buffer, 0).unwrap();
		//file.sync_all().unwrap();

		println!("partial_page_{}_20_bytes,{}", page_size, t1.elapsed().as_secs_f64());
	    }

	    let overpage: usize = page_size + page_size / 2;
	    let mut partial_buffer = vec![0; overpage];
	    for i in 0..partial_buffer.len() {
		partial_buffer[i] = (i % 256) as u8;
	    }

	    for _ in 0..ATTEMPTS {
		file.set_len(0).unwrap();
		drop_pagecache();

		let t1 = std::time::Instant::now();
		file.write_all_at(&partial_buffer, 0).unwrap();
		//file.sync_all().unwrap();

		println!("partial_page_{}_{}_bytes,{}", page_size, overpage, t1.elapsed().as_secs_f64());
	    }

	    let mut full_buffer = vec![0; page_size];
	    for i in 0..full_buffer.len() {
		full_buffer[i] = (i % 256) as u8;
	    }

	    for _ in 0..ATTEMPTS {
		file.set_len(0).unwrap();
		drop_pagecache();

		let t1 = std::time::Instant::now();
		file.write_all_at(&full_buffer, 0).unwrap();
		//file.sync_all().unwrap();

		println!("full_page_{},{}", page_size, t1.elapsed().as_secs_f64());
	    }

	    let mut full_buffer = vec![0; page_size*2];
	    for i in 0..overpage { // Only as much as other.
		full_buffer[i] = (i % 256) as u8;
	    }
	    assert_eq!(full_buffer[0..overpage], partial_buffer[0..overpage]);

	    for _ in 0..ATTEMPTS {
		file.set_len(0).unwrap();
		drop_pagecache();

		let t1 = std::time::Instant::now();
		file.write_all_at(&full_buffer[0..page_size], 0).unwrap();
		file.write_all_at(&full_buffer[page_size..page_size*2], page_size as u64).unwrap();
		//file.sync_all().unwrap();

		println!("2_full_page_{},{}", page_size, t1.elapsed().as_secs_f64());
	    }
	}
    }
}
