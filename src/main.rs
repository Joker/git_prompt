#![deny(warnings)]

use git2::{Repository, StatusOptions};
use std::fmt;

fn count_stash(mrepo: &mut Repository) -> usize {
	let mut count = 0;
	mrepo
		.stash_foreach(|_, _, _| {
			count += 1;
			true
		})
		.unwrap();
	count
}

fn print_count(repo: &Repository, statuses: &git2::Statuses) {
	#[derive(Default)]
	#[allow(non_snake_case)]
	struct Counts {
		A: usize,
		M: usize,
		D: usize,
		R: usize,
		T: usize,
	}
	impl fmt::Display for Counts {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			if self.A > 0 {
				write!(f, "\x1b[31m{}\x1b[0m", self.A)?
			}
			if self.M > 0 {
				write!(f, "{}", self.M)?
			}
			if self.D > 0 {
				write!(f, "{}", self.D)?
			}
			if self.R > 0 {
				write!(f, "{}", self.R)?
			}
			if self.T > 0 {
				write!(f, "{}", self.T)?
			}
			return Ok(());
		}
	}

	let mut ix = Counts {
		..Default::default()
	};
	let mut wt = Counts {
		..Default::default()
	};
	let mut ignor = 0;
	let mut ups = 0;

	for entry in statuses
		.iter()
		.filter(|e| e.status() != git2::Status::CURRENT)
	{
		match entry.status() {
			git2::Status::INDEX_NEW => ix.A += 1,
			git2::Status::INDEX_MODIFIED => ix.M += 1,
			git2::Status::INDEX_DELETED => ix.D += 1,
			git2::Status::INDEX_RENAMED => ix.R += 1,
			git2::Status::INDEX_TYPECHANGE => ix.T += 1,

			// status of the file in the working directory relative to the index
			git2::Status::WT_NEW => wt.A += 1,
			git2::Status::WT_MODIFIED => wt.M += 1,
			git2::Status::WT_DELETED => wt.D += 1,
			git2::Status::WT_RENAMED => wt.R += 1,
			git2::Status::WT_TYPECHANGE => wt.T += 1,

			//
			git2::Status::IGNORED => ignor += 1,
			_ => ups += 1,
		};
	}
	print!("{} {} ~{}~", ix, wt, repo.submodules().unwrap().len());
	if ups > 0 {
		println!("Ups {}", ups)
	}
	if ignor > 0 {
		println!("Ignor {}", ignor)
	}
}

fn main() {
	match &mut Repository::open("../lint_test") {
		Ok(repo) => {
			{
				if repo.is_bare() {
					println!("bare");
					return;
				}
				let mut opts = StatusOptions::new();
				opts.include_ignored(false);
				opts.include_untracked(true).recurse_untracked_dirs(true);

				let statuses = repo.statuses(Some(&mut opts)).unwrap();
				print_count(&repo, &statuses);
			}
			count_stash(repo);
		}
		_ => (),
	}
}
