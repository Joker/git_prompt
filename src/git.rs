#![deny(warnings)]
use git2::{DescribeOptions, Repository, RepositoryState as rs, Status as st, StatusOptions};

pub static RED: &str = "%{\x1b[0;31m%}";
pub static RED_L: &str = "%{\x1b[1;31m%}";
pub static GREEN: &str = "%{\x1b[0;32m%}";
pub static GREEN_L: &str = "%{\x1b[38;5;28m%}";
pub static BLUE_L: &str = "%{\x1b[1;94m%}";
pub static YELLOW: &str = "%{\x1b[0;33m%}";
pub static YELLOW_L: &str = "%{\x1b[33;1m%}";
pub static MAGENTA: &str = "%{\x1b[0;35m%}";
pub static MAGENTA_L: &str = "%{\x1b[1;35m%}";
pub static CYAN: &str = "%{\x1b[0;36m%}";
pub static RESET: &str = "%{\x1b[0m%}";

macro_rules! unwrap_or_return {
	( $e:expr ) => {
		match $e {
			Ok(x) => x,
			Err(_) => return String::new(),
		}
	};
}

// git describe --all HEAD
fn print_describe(repo: &Repository) -> String {
	// let head = unwrap_or_return!(repo.head());
	// let oid = match head.target() {
	// 	Some(id) => id,
	// 	None => return String::new(),
	// };
	// let obj = unwrap_or_return!(repo.find_object(oid, None));

	let des = unwrap_or_return!(repo.describe(
		DescribeOptions::new()
			.describe_all()
			.show_commit_oid_as_fallback(true),
	));
	des.format(None).unwrap_or(" head ".to_string())
}

fn print_branch(repo: &Repository) -> String {
	let mut out = String::from(&format!("{}(", MAGENTA));
	if repo.is_shallow() {
		out.push_str(&format!("{}shallow {}", GREEN, MAGENTA))
	}
	if repo.is_worktree() {
		out.push_str(&format!("{}worktree {}", GREEN, MAGENTA))
	}
	if let Ok(head) = repo.head() {
		match head.shorthand() {
			Some("HEAD") => out.push_str(&print_describe(repo)),
			Some(name) => out.push_str(name),
			None => (),
		}
	}
	match repo.state() {
		rs::Clean => (),
		rs::Merge => out.push_str(&format!("{} merge", CYAN)),
		rs::Revert => out.push_str(&format!("{} revert", CYAN)),
		rs::RevertSequence => out.push_str(&format!("{} revert-sq", CYAN)),
		rs::CherryPick => out.push_str(&format!("{} cherry-pick", CYAN)),
		rs::CherryPickSequence => out.push_str(&format!("{} cherry-pick-sq", CYAN)),
		rs::Bisect => out.push_str(&format!("{} bisect", CYAN)),
		rs::Rebase => out.push_str(&format!("{} rebase", CYAN)),
		rs::RebaseMerge => out.push_str(&format!("{} rebase-m", CYAN)),
		rs::RebaseInteractive => out.push_str(&format!("{} rebase-i", CYAN)),
		rs::ApplyMailbox => out.push_str(&format!("{} am", CYAN)),
		rs::ApplyMailboxOrRebase => out.push_str(&format!("{} am-rebase", CYAN)),
	}
	if let Ok(sub) = repo.submodules() {
		if sub.len() > 0 {
			out.push_str(&format!("{} submodule-{}", YELLOW, sub.len()))
		}
	}
	out.push_str(&format!("{})", MAGENTA));
	out
}

fn print_count(statuses: &git2::Statuses) -> String {
	let mut ix = 0;
	let mut wt = 0;
	let mut ups = 0;
	let mut ignored = 0;
	let mut untracked = 0;

	for entry in statuses.iter().filter(|e| e.status() != st::CURRENT) {
		match entry.status() {
			s if s.is_wt_new() => untracked += 1,
			s if s.is_ignored() => ignored += 1,
			s if s.is_conflicted() => ups += 1,
			_ => (),
		};
		match entry.status() {
			s if s.is_wt_modified() => wt += 1,
			s if s.is_wt_deleted() => wt += 1,
			s if s.is_wt_renamed() => wt += 1,
			s if s.is_wt_typechange() => wt += 1,
			_ => (),
		};
		match entry.status() {
			s if s.is_index_new() => ix += 1,
			s if s.is_index_modified() => ix += 1,
			s if s.is_index_deleted() => ix += 1,
			s if s.is_index_renamed() => ix += 1,
			s if s.is_index_typechange() => ix += 1,
			_ => (),
		};
		// println!("{:?}", entry.status())
	}
	let mut out = String::new();
	match (ix, wt) {
		(0, 0) => (),
		(m, 0) => out.push_str(&format!("{} [{}]", GREEN, m)),
		(0, n) => out.push_str(&format!("{} [{}]", RED, n)),
		(m, n) => out.push_str(&format!("{} [{}, {}{}]", GREEN, m, RED, n)),
	}
	if ups > 0 {
		out.push_str(&format!("{} ~{}~", CYAN, ups))
	}
	if untracked > 0 {
		out.push_str(&format!("{} -{}-", RED_L, untracked))
	}
	if ignored > 0 {
		out.push_str(&format!("{} _{}_", MAGENTA_L, ignored))
	}
	out
}

fn print_stash(mrepo: &mut Repository) -> String {
	let mut count = 0;
	unwrap_or_return!(mrepo.stash_foreach(|_, _, _| {
		count += 1;
		true
	}));

	let mut out = String::new();
	if count > 0 {
		out.push_str(&format!("{} {{{}}}", YELLOW_L, count))
	}
	out
}

pub fn prompt() -> String {
	let mut out = String::new();
	if let Ok(repo) = &mut Repository::discover(".") {
		if repo.is_bare() {
			out.push_str("( git:bare )");
			return out;
		}

		if let Ok(empty) = repo.is_empty() {
			if empty {
				out.push_str(&format!(
					"{}({} 0 commits {}){}",
					MAGENTA, CYAN, MAGENTA, RESET
				));
				return out;
			}
		}
		//
		{
			let mut opts = StatusOptions::new();
			opts.include_untracked(true)
				.renames_from_rewrites(true)
				.renames_head_to_index(true)
				.include_unmodified(true);
			let statuses = unwrap_or_return!(repo.statuses(Some(&mut opts)));

			out.push_str(&print_branch(repo));
			out.push_str(&print_count(&statuses));
		}
		out.push_str(&print_stash(repo));
	}
	out.push_str(RESET);
	out
}
