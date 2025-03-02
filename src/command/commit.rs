use crate::conventional::commit::Commit;
use crate::CocoGitto;
use anyhow::Result;
use conventional_commit_parser::commit::{CommitType, ConventionalCommit};
use conventional_commit_parser::parse_footers;
use log::info;

impl CocoGitto {
    #[allow(clippy::too_many_arguments)]
    pub fn conventional_commit(
        &self,
        commit_type: &str,
        scope: Option<String>,
        summary: String,
        body: Option<String>,
        footer: Option<String>,
        is_breaking_change: bool,
        sign: bool,
    ) -> Result<()> {
        // Ensure commit type is known
        let commit_type = CommitType::from(commit_type);

        // Ensure footers are correctly formatted
        let footers = match footer {
            Some(footers) => parse_footers(&footers)?,
            None => Vec::with_capacity(0),
        };

        let conventional_message = ConventionalCommit {
            commit_type,
            scope,
            body,
            footers,
            summary,
            is_breaking_change,
        }
        .to_string();

        // Validate the message
        conventional_commit_parser::parse(&conventional_message)?;

        // Git commit
        let sign = sign || self.repository.gpg_sign();
        let oid = self.repository.commit(&conventional_message, sign)?;

        // Pretty print a conventional commit summary
        let commit = self.repository.0.find_commit(oid)?;
        let commit = Commit::from_git_commit(&commit)?;
        info!("{}", commit);

        Ok(())
    }
}
