#![deny(warnings)]

use git2::{Repository, RepositoryState as rs, Status as st, StatusOptions};

fn print_branch(repo: &Repository) {
	// {
	// 	let head = repo.head().unwrap();
	// 	let oid = head.target().unwrap();
	// 	let commit = repo.find_commit(oid).unwrap();
	// }

	let head = match repo.head() {
		Ok(head) => Some(head),
		Err(ref e) => {
			println!("{:?}", e.code());
			None
		}
	};
	let shead = head.as_ref().and_then(|h| h.shorthand());

	match shead {
		Some(h) => println!("=== {} ===", h),
		None => ()
	}

	print!("\x1b[0;35m({}", shead.unwrap_or("(no branch)"));

	match repo.state() {
		rs::Clean => (),
		rs::Merge => print!("|merge"),
		rs::Revert => print!("|revert"),
		rs::RevertSequence => print!("|revert-s"),
		rs::CherryPick => print!("|cherry-pick"),
		rs::CherryPickSequence => print!("|cherry-pick-s"),
		rs::Bisect => print!("|bisect"),
		rs::Rebase => print!("|rebase"),
		rs::RebaseInteractive => print!("|rebase-i"),
		rs::RebaseMerge => print!("|rebase-m"),
		rs::ApplyMailbox => print!("|am"),
		rs::ApplyMailboxOrRebase => print!("|am-rebase"),
	}
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

	for entry in statuses.iter().filter(|e| e.status() != st::CURRENT) {
		match entry.status() {
			s if s.contains(st::WT_NEW) => unt += 1,

			s if s.contains(st::WT_MODIFIED) => wt += 1,
			s if s.contains(st::WT_DELETED) => wt += 1,
			s if s.contains(st::WT_TYPECHANGE) => wt += 1,
			s if s.contains(st::WT_RENAMED) => wt += 1,

			s if s.contains(st::INDEX_NEW) => ix += 1,
			s if s.contains(st::INDEX_MODIFIED) => ix += 1,
			s if s.contains(st::INDEX_DELETED) => ix += 1,
			s if s.contains(st::INDEX_TYPECHANGE) => ix += 1,
			s if s.contains(st::INDEX_RENAMED) => ix += 1,

			s if s.contains(st::IGNORED) => ignr += 1,
			s if s.contains(st::CONFLICTED) => ups += 1,
			_ => (),
		};
		// println!("{:?}", entry.status())
	}
	if ix > 0 || wt > 0 {
		print!("\x1b[0;32m[{}, \x1b[0;31m{}] ", ix, wt);
	}
	if ups > 0 {
		print!("\x1b[0;36m~{}~ ", ups)
	}
	if unt > 0 {
		print!("\x1b[1;31m-{}- ", unt)
	}
	if ignr > 0 {
		print!("\x1b[1;35mIgnor_{} ", ignr)
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
		print!("\x1b[33;1m{{{}}}", count)
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
