use std::path::Path;

use color_eyre::eyre::Result;
use git2::{Error, Repository, Signature};

struct LocalProvider;

impl LocalProvider {
    fn create_repository(name: &str) -> Result<Repository> {
        let repo = Repository::init(name)?;
        Ok(repo)
    }

    fn setup_main_branch(repo: &Repository) -> Result<()> {
        let head = repo.head()?;
        let head_commit = repo.find_commit(head.target().unwrap())?;

        // Create main branch
        repo.branch("main", &head_commit, false)?;

        Ok(())
    }

    fn initial_commit(repo: &Repository) -> Result<()> {
        let signature = Signature::now("John Doe", "john@example.com")?;
        let tree_id = repo.index()?.write_tree()?;
        let tree = repo.find_tree(tree_id)?;

        // Create initial commit
        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            "init commit",
            &tree,
            &[],
        )?;

        Ok(())
    }

    fn clone_repository_with_ssh(
        url: &str,
        username: &str,
        ssh_key_path: &str,
    ) -> Result<Repository> {
        let mut callbacks = git2::RemoteCallbacks::new();
        callbacks.credentials(move |url, username, _| {
            let username = username.unwrap_or("git");
            git2::Cred::ssh_key(
                username,
                None,
                std::path::Path::new(ssh_key_path),
                None,
            )
        });

        let mut fetch_options = git2::FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);

        // let repo = Repository::clone(url, Path::new("cloned_repo"), &fetch_options)?;
        let repo = Repository::clone(url, Path::new("cloned_repo"))?;

        Ok(repo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_create_repository() {
        let temp_dir = tempdir().expect("Failed to create temporary directory");
        let repo_name = temp_dir.path().to_str().unwrap();

        let result = LocalProvider::create_repository(repo_name);
        assert!(result.is_ok());

        let repo = result.unwrap();
        assert!(repo.path().exists());

        temp_dir.close().unwrap();
    }

    // #[test]
    // fn test_setup_main_branch() {
    //     let repo = Repository::init("test_repo").unwrap();
    //     let result = LocalProvider::setup_main_branch(&repo);
    //     assert!(result.is_ok());

    //     let head = repo.head().unwrap();
    //     let branch_name = head.shorthand().unwrap();
    //     assert_eq!(branch_name, "main");

    //     fs::remove_dir_all("test_repo").unwrap(); // Clean up
    // }

    // #[test]
    // fn test_initial_commit() {
    //     let repo = Repository::init("test_repo").unwrap();
    //     let result = LocalProvider::initial_commit(&repo);
    //     assert!(result.is_ok());

    //     let head = repo.head().unwrap();
    //     let commit = repo.find_commit(head.target().unwrap()).unwrap();
    //     assert_eq!(commit.message().unwrap(), "init commit");

    //     fs::remove_dir_all("test_repo").unwrap(); // Clean up
    // }

    // #[test]
    // fn test_clone_repository_with_ssh() {
    //     // Note: This test may fail if the SSH key or repository URL is invalid
    //     let repo_url = "git@github.com:user/repo.git";
    //     let ssh_username = "username";
    //     let ssh_key_path = "/path/to/ssh/key";

    //     let result = LocalProvider::clone_repository_with_ssh(repo_url, ssh_username, ssh_key_path);
    //     assert!(result.is_ok());

    //     let cloned_repo = result.unwrap();
    //     assert!(cloned_repo.path().exists());

    //     fs::remove_dir_all("cloned_repo").unwrap(); // Clean up
    // }
}
