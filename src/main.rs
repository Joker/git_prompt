#![deny(warnings)]

use git2::{Repository, StatusOptions};
// use std::str;
// use std::fmt;

fn print_branch(repo: &Repository) {
	let head = match repo.head() {
		Ok(head) => Some(head),
		// Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => {
		// 	None
		// }
		Err(e) => {
			println!("{:?}", e);
			None
		}
	};
	let head = head.as_ref().and_then(|h| h.shorthand());

	print!("\x1b[0;35m({}", head.unwrap_or("HEAD (no branch)"));

	let sub = repo.submodules().unwrap().len();
	if sub > 0 {
		print!("\x1b[0;33m sub-{}\x1b[0;35m) ", sub);
	} else {
		print!("\x1b[0;35m) ");
	}
}

fn print_count(statuses: &git2::Statuses) {
	let mut ix = 0;
	let mut wt = 0;
	let mut ignr = 0;
	let mut ups = 0;
	let mut unt = 0;

	for entry in statuses
		.iter()
		.filter(|e| e.status() != git2::Status::CURRENT)
	{
		match entry.status() {
			s if s.contains(git2::Status::WT_NEW) => unt += 1,

			s if s.contains(git2::Status::WT_MODIFIED) => wt += 1,
			s if s.contains(git2::Status::WT_DELETED) => wt += 1,
			s if s.contains(git2::Status::WT_TYPECHANGE) => wt += 1,
			s if s.contains(git2::Status::WT_RENAMED) => wt += 1,

			s if s.contains(git2::Status::INDEX_NEW) => ix += 1,
			s if s.contains(git2::Status::INDEX_MODIFIED) => ix += 1,
			s if s.contains(git2::Status::INDEX_DELETED) => ix += 1,
			s if s.contains(git2::Status::INDEX_TYPECHANGE) => ix += 1,
			s if s.contains(git2::Status::INDEX_RENAMED) => ix += 1,

			s if s.contains(git2::Status::IGNORED) => ignr += 1,
			s if s.contains(git2::Status::CONFLICTED) => ups += 1,
			_ => (),
		};
		// println!("{:?}", entry.status())
	}
	if ix > 0 || wt > 0 {
		print!("\x1b[0;32m[{}, \x1b[0;31m{}] ", ix, wt);
	}
	if unt > 0 {
		print!("\x1b[1;31m-{}-", unt)
	}
	if ups > 0 {
		print!("\x1b[0;35mUps_{}", ups)
	}
	if ignr > 0 {
		print!("\x1b[0;36m~{}~", ignr)
	}
}

fn print_stash(mrepo: &mut Repository) {
	let mut count = 0;
	mrepo
		.stash_foreach(|_, _, _| {
			count += 1;
			true
		})
		.unwrap();

	if count > 0 {
		print!(" \x1b[33;1m{{{}}}", count)
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
				print_branch(repo);
				print_count(&statuses);
			}
			print_stash(repo);
			print!("\x1b[0m")
		}
		_ => (),
	}
}
