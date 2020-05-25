#![deny(warnings)]

use git2::{DescribeOptions, Repository, RepositoryState as rs, Status as st, StatusOptions};

macro_rules! unwrap_or_return {
	( $e:expr ) => {
		match $e {
			Ok(x) => x,
			Err(_) => return,
			}
	};
}

// git describe --all HEAD
fn print_describe(repo: &Repository) {
	let head = unwrap_or_return!(repo.head());
	let oid = match head.target() {
		Some(id) => id,
		None => return,
	};
	let obj = unwrap_or_return!(repo.find_object(oid, None));

	let des = unwrap_or_return!(obj.describe(
		DescribeOptions::new()
			.describe_all()
			.show_commit_oid_as_fallback(true),
	));
	print!("{}", des.format(None).unwrap_or(" head ".to_string()));
}

fn print_branch(repo: &Repository) {
	print!("\x1b[0;35m(");

	match repo.head() {
		Ok(head) => match head.shorthand() {
			Some("HEAD") => print_describe(repo),
			Some(name) => print!("{}", name),
			None => (),
		},
		Err(_) => (),
	};
	match repo.state() {
		rs::Clean => (),
		rs::Merge => print!("\x1b[0;36m merge"),
		rs::Revert => print!("\x1b[0;36m revert"),
		rs::RevertSequence => print!("\x1b[0;36m revert-s"),
		rs::CherryPick => print!("\x1b[0;36m cherry-pick"),
		rs::CherryPickSequence => print!("\x1b[0;36m cherry-pick-s"),
		rs::Bisect => print!("\x1b[0;36m bisect"),
		rs::Rebase => print!("\x1b[0;36m rebase"),
		rs::RebaseInteractive => print!("\x1b[0;36m rebase-i"),
		rs::RebaseMerge => print!("\x1b[0;36m rebase-m"),
		rs::ApplyMailbox => print!("\x1b[0;36m am"),
		rs::ApplyMailboxOrRebase => print!("\x1b[0;36m am-rebase"),
	}
	let sub = match repo.submodules() {
		Ok(vec) => vec.len(),
		Err(_) => 0,
	};
	if sub > 0 {
		print!("\x1b[0;33m sub-{}", sub);
	}
	print!("\x1b[0;35m)");
}

fn print_count(statuses: &git2::Statuses) {
	let mut ix = 0;
	let mut wt = 0;
	let mut ups = 0;
	let mut ignored = 0;
	let mut untracked = 0;

	for entry in statuses.iter().filter(|e| e.status() != st::CURRENT) {
		match entry.status() {
			s if s.contains(st::WT_NEW) => untracked += 1,
			s if s.contains(st::IGNORED) => ignored += 1,
			s if s.contains(st::CONFLICTED) => ups += 1,
			_ => (),
		};
		match entry.status() {
			s if s.contains(st::WT_MODIFIED) => wt += 1,
			s if s.contains(st::WT_DELETED) => wt += 1,
			s if s.contains(st::WT_RENAMED) => wt += 1,
			s if s.contains(st::WT_TYPECHANGE) => wt += 1,
			_ => (),
		};
		match entry.status() {
			s if s.contains(st::INDEX_NEW) => ix += 1,
			s if s.contains(st::INDEX_MODIFIED) => ix += 1,
			s if s.contains(st::INDEX_DELETED) => ix += 1,
			s if s.contains(st::INDEX_RENAMED) => ix += 1,
			s if s.contains(st::INDEX_TYPECHANGE) => ix += 1,
			_ => (),
		};
		// println!("{:?}", entry.status())
	}
	match (ix, wt) {
		(0, 0) => (),
		(m, 0) => print!("\x1b[0;32m [{}]", m),
		(0, n) => print!("\x1b[0;31m [{}]", n),
		(m, n) => print!("\x1b[0;32m [{}, \x1b[0;31m{}]", m, n),
	}
	if ups > 0 {
		print!("\x1b[0;36m ~{}~", ups)
	}
	if untracked > 0 {
		print!("\x1b[1;31m -{}-", untracked)
	}
	if ignored > 0 {
		print!("\x1b[1;35m _{}_", ignored)
	}
}

fn print_stash(mrepo: &mut Repository) {
	let mut count = 0;
	unwrap_or_return!(mrepo.stash_foreach(|_, _, _| {
		count += 1;
		true
	}));

	if count > 0 {
		print!("\x1b[33;1m {{{}}}", count)
	}
}

pub fn prompt() {
	match &mut Repository::discover(".") {
		Ok(repo) => {
			{
				if repo.is_bare() {
					print!("( bare )");
					return;
				}
				let mut opts = StatusOptions::new();
				opts.include_ignored(false);
				opts.include_untracked(true).recurse_untracked_dirs(true);

				let statuses = unwrap_or_return!(repo.statuses(Some(&mut opts)));
				print_branch(repo);
				print_count(&statuses);
			}
			print_stash(repo);
			print!("\x1b[0m");
		}
		_ => (),
	}
}