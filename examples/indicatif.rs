use indicatif::{HumanDuration, ProgressBar};
use label_logger::{OutputLabel, format_label, label_theme, success};
use rand::Rng;
use std::{
	sync::{Arc, Mutex, mpsc},
	thread,
	time::{Duration, Instant},
};

static CRATES: &[(&str, &str)] = &[
	("console", "v0.14.1"),
	("lazy_static", "v1.4.0"),
	("libc", "v0.2.93"),
	("regex", "v1.4.6"),
	("regex-syntax", "v0.6.23"),
	("terminal_size", "v0.1.16"),
	("libc", "v0.2.93"),
	("unicode-width", "v0.1.8"),
	("lazy_static", "v1.4.0"),
	("number_prefix", "v0.4.0"),
	("regex", "v1.4.6"),
	("rand", "v0.8.3"),
	("getrandom", "v0.2.2"),
	("cfg-if", "v1.0.0"),
	("libc", "v0.2.93"),
	("rand_chacha", "v0.3.0"),
	("ppv-lite86", "v0.2.10"),
	("rand_core", "v0.6.2"),
	("getrandom", "v0.2.2"),
	("rand_core", "v0.6.2"),
	("tokio", "v1.5.0"),
	("bytes", "v1.0.1"),
	("pin-project-lite", "v0.2.6"),
	("slab", "v0.4.3"),
	("indicatif", "v0.15.0"),
];

fn main() {
	const NUM_CPUS: usize = 4;

	let start = Instant::now();

	let pb = ProgressBar::new(CRATES.len() as u64)
		.with_style(label_theme(OutputLabel::Info("Building")));

	let crates = Arc::new(Mutex::new(CRATES.iter()));
	let (tx, rx) = mpsc::channel();

	for index in 0..NUM_CPUS {
		let tx = tx.clone();
		let crates = Arc::clone(&crates);

		thread::spawn(move || {
			let mut rng = rand::thread_rng();

			loop {
				let krate = crates.lock().unwrap().next();

				tx.send((index, krate)).unwrap();
				if let Some(krate) = krate {
					thread::sleep(Duration::from_millis(
						// last compile and linking is always slow, let's mimic that
						if CRATES.last() == Some(krate) {
							rng.gen_range(1_000..2_000)
						} else {
							rng.gen_range(250..1_000)
						},
					));
				} else {
					break;
				}
			}
		});
	}

	drop(tx);

	let mut processing = [None; NUM_CPUS];
	while let Ok((n, krate)) = rx.recv() {
		processing[n] = krate;

		let crates: Vec<&str> = processing
			.iter()
			.filter_map(|t| t.copied().map(|(name, _)| name))
			.collect();

		pb.set_message(crates.join(", "));

		if let Some((name, version)) = krate {
			let msg = format_label!(label: OutputLabel::Success("Compiling"), "{name} {version}");
			pb.println(msg);

			pb.inc(1);
		}
	}

	pb.finish_and_clear();

	success!(
		label: "Finished",
		"dev [unoptimized + debuginfo] target(s) in {}",
		HumanDuration(start.elapsed())
	);
}
