#![allow(dead_code)]
// Copyright (c) 2023 Murilo Ijanc' <mbsd@m0x.ru>
//
// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
use color_eyre::eyre::Result;
use git2::{Repository, Signature};

use tracing::debug;

pub struct LocalProvider;

impl LocalProvider {
    pub fn create_repository(name: &str) -> Result<Repository> {
        let repo = Repository::init(name)?;
        Ok(repo)
    }

    // pub fn setup_main_branch(repo: &Repository) -> Result<()> {
    //     let head = repo.head()?;
    //     let head_commit = head.peel_to_commit()?;
    //     // let head_commit = repo.find_commit(head.target().unwrap())?;

    //     // Create main branch
    //     // repo.branch("main", &head_commit, false)?;
    //     let branch_ref = repo.branch("main", &head_commit, false)?;

    //     // Move a HEAD para a nova branch
    //     repo.set_head(branch_ref.name().unwrap().unwrap())?;

    //     Ok(())
    // }

    pub fn initial_commit(repo: &Repository) -> Result<()> {
        let signature = Signature::now("Mps", "mps@mps.com")?;
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

    pub fn clone(url: &str, to: &str) -> Result<Repository> {
        debug!("cloning repo from {} to {}", url, to);
        let repo = Repository::clone(url, to)?;
        debug!("repo cloned {} to {}", url, to);
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
    //     let temp_dir = tempdir().expect("Failed to create temporary directory");
    //     let repo_name = temp_dir.path().to_str().unwrap();
    //     let repo = Repository::init(repo_name).unwrap();
    //     let result = LocalProvider::setup_main_branch(&repo);
    //     result.unwrap();
    //     // assert!(result.is_ok());

    //     let head = repo.head().unwrap();
    //     let branch_name = head.shorthand().unwrap();
    //     assert_eq!(branch_name, "main");

    //     temp_dir.close().unwrap();
    // }

    #[test]
    fn test_initial_commit() {
        let temp_dir = tempdir().expect("Failed to create temporary directory");
        let repo_name = temp_dir.path().to_str().unwrap();
        let repo = Repository::init(repo_name).unwrap();
        let result = LocalProvider::initial_commit(&repo);
        assert!(result.is_ok());

        let head = repo.head().unwrap();
        let commit = repo.find_commit(head.target().unwrap()).unwrap();
        assert_eq!(commit.message().unwrap(), "init commit");

        temp_dir.close().unwrap();
    }

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
