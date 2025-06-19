use std::{
	borrow::Cow,
	env, fs,
	io::{BufRead, stdin},
};

use regex::Regex;

fn main() {
	let regex = Regex::new("/nix/store/.{32}").unwrap();
	let args = env::args().skip(1).collect::<Vec<_>>();
	if args.is_empty() {
		let stdin = stdin();
		let lines = stdin.lock().lines();
		nuke(&regex, lines.filter_map(|x| x.ok()), |x| println!("{x}"));
	} else {
		for filename in args {
			let content = fs::read_to_string(&filename).expect(&format!("failed to read {filename}"));
			let mut new_content = String::new();
			nuke(&regex, content.split('\n'), |x| {
				new_content += &x;
				new_content.push('\n')
			});
			fs::write(&filename, new_content).expect(&format!("failed to overwrite {filename}"));
		}
	}
}

fn nuke(regex: &Regex, input: impl Iterator<Item = impl AsRef<str>>, mut output: impl FnMut(Cow<'_, str>)) {
	input.for_each(|line| {
		let line = line.as_ref();
		if line.starts_with(r#"@nix { "action": "setPhase", "phase": ""#) {
			return;
		}
		let replaced = regex.replace_all(line, "/nix/store/eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee");
		output(replaced);
	})
}
