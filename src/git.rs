use git2::{DescribeOptions, Repository, RepositoryState as rs, Status as st, StatusOptions};

#[cfg(feature = "bash")]
use crate::bash::{CYAN, GREEN, MAGENTA, MAGENTA_L, RED, RED_L, RESET, YELLOW, YELLOW_L};
#[cfg(not(feature = "bash"))]
use crate::zsh::{CYAN, GREEN, MAGENTA, MAGENTA_L, RED, RED_L, RESET, YELLOW, YELLOW_L};

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
	let mut out = String::from(&format!("{MAGENTA}("));
	if repo.is_shallow() {
		out.push_str(&format!("{GREEN}shallow {MAGENTA}"))
	}
	if repo.is_worktree() {
		out.push_str(&format!("{GREEN}worktree {MAGENTA}"))
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
		rs::Merge => out.push_str(&format!("{CYAN} merge")),
		rs::Revert => out.push_str(&format!("{CYAN} revert")),
		rs::RevertSequence => out.push_str(&format!("{CYAN} revert-sq")),
		rs::CherryPick => out.push_str(&format!("{CYAN} cherry-pick")),
		rs::CherryPickSequence => out.push_str(&format!("{CYAN} cherry-pick-sq")),
		rs::Bisect => out.push_str(&format!("{CYAN} bisect")),
		rs::Rebase => out.push_str(&format!("{CYAN} rebase")),
		rs::RebaseMerge => out.push_str(&format!("{CYAN} rebase-m")),
		rs::RebaseInteractive => out.push_str(&format!("{CYAN} rebase-i")),
		rs::ApplyMailbox => out.push_str(&format!("{CYAN} am")),
		rs::ApplyMailboxOrRebase => out.push_str(&format!("{CYAN} am-rebase")),
	}
	if let Ok(sub) = repo.submodules() {
		if !sub.is_empty() {
			out.push_str(&format!("{YELLOW} submodule-{}", sub.len()))
		}
	}
	out.push_str(&format!("{MAGENTA})"));
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
		(m, 0) => out.push_str(&format!("{GREEN} [{m}]")),
		(0, n) => out.push_str(&format!("{RED} [{n}]")),
		(m, n) => out.push_str(&format!("{GREEN} [{m}, {RED}{n}]")),
	}
	if ups > 0 {
		out.push_str(&format!("{CYAN} ~{ups}~"))
	}
	if untracked > 0 {
		out.push_str(&format!("{RED_L} -{untracked}-"))
	}
	if ignored > 0 {
		out.push_str(&format!("{MAGENTA_L} _{ignored}_"))
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
		out.push_str(&format!("{YELLOW_L} {{{count}}}"))
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
				out.push_str(&format!("{MAGENTA}({CYAN} 0 commits {MAGENTA}){RESET}",));
				return out;
			}
		}

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
