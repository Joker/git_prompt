#![deny(warnings)]

use git2::{
	DescribeOptions, ErrorCode::UnbornBranch, Repository, RepositoryState as rs, Status as st,
	StatusOptions,
};

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
	let head = unwrap_or_return!(repo.head());
	let oid = match head.target() {
		Some(id) => id,
		None => return String::new(),
	};
	let obj = unwrap_or_return!(repo.find_object(oid, None));

	let des = unwrap_or_return!(obj.describe(
		DescribeOptions::new()
			.describe_all()
			.show_commit_oid_as_fallback(true),
	));
	des.format(None).unwrap_or(" head ".to_string())
}

fn print_branch(repo: &Repository) -> String {
	let mut out = String::from("%{\x1b[0;35m%}(");
	match repo.head() {
		Ok(head) => match head.shorthand() {
			Some("HEAD") => out.push_str(&print_describe(repo)),
			Some(name) => out.push_str(name),
			None => (),
		},
		Err(e) => match e.code() {
			UnbornBranch => out.push_str(" %{\x1b[0;36m%}no commits "),
			_ => (),
		},
	};
	match repo.state() {
		rs::Clean => (),
		rs::Merge => out.push_str("%{\x1b[0;36m%} merge"),
		rs::Revert => out.push_str("%{\x1b[0;36m%} revert"),
		rs::RevertSequence => out.push_str("%{\x1b[0;36m%} revert-s"),
		rs::CherryPick => out.push_str("%{\x1b[0;36m%} cherry-pick"),
		rs::CherryPickSequence => out.push_str("%{\x1b[0;36m%} cherry-pick-s"),
		rs::Bisect => out.push_str("%{\x1b[0;36m%} bisect"),
		rs::Rebase => out.push_str("%{\x1b[0;36m%} rebase"),
		rs::RebaseInteractive => out.push_str("%{\x1b[0;36m%} rebase-i"),
		rs::RebaseMerge => out.push_str("%{\x1b[0;36m%} rebase-m"),
		rs::ApplyMailbox => out.push_str("%{\x1b[0;36m%} am"),
		rs::ApplyMailboxOrRebase => out.push_str("%{\x1b[0;36m%} am-rebase"),
	};
	let sub = match repo.submodules() {
		Ok(vec) => vec.len(),
		Err(_) => 0,
	};
	if sub > 0 {
		out.push_str(&format!("%{{\x1b[0;33m%}} sub-{}", sub))
	}
	out.push_str("%{\x1b[0;35m%})");
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
		(m, 0) => out.push_str(&format!("%{{\x1b[0;32m%}} [{}]", m)),
		(0, n) => out.push_str(&format!("%{{\x1b[0;31m%}} [{}]", n)),
		(m, n) => out.push_str(&format!("%{{\x1b[0;32m%}} [{}, \x1b[0;31m{}]", m, n)),
	}
	if ups > 0 {
		out.push_str(&format!("%{{\x1b[0;36m%}} ~{}~", ups))
	}
	if untracked > 0 {
		out.push_str(&format!("%{{\x1b[1;31m%}} -{}-", untracked))
	}
	if ignored > 0 {
		out.push_str(&format!("%{{\x1b[1;35m%}} _{}_", ignored))
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
		out.push_str(&format!("%{{\x1b[33;1m%}} {{{}}}", count))
	}
	out
}

pub fn prompt() -> String {
	let mut out = String::new();
	match &mut Repository::discover(".") {
		Ok(repo) => {
			{
				if repo.is_bare() {
					out.push_str("( bare )");
					return out;
				}

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
		_ => (),
	}
	out.push_str("%{\x1b[0m%}");
	out
}
