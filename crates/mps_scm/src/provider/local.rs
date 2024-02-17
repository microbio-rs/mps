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
use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use git2::{
    build::RepoBuilder, Cred, FetchOptions, IndexAddOption, PushOptions,
    RemoteCallbacks, Repository, Signature,
};
use tracing::{debug, info};

#[derive(thiserror::Error, Debug)]
pub(crate) enum LocalError {
    #[error("Lib git2 errror: {0}")]
    Git2Error(#[from] git2::Error),
}

#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct LocalConfig {
    pub(crate) base_git_path: PathBuf,
    pub(crate) git_user: String,
    pub(crate) owner: String,
    pub(crate) ssh_prv_key: PathBuf,
    pub(crate) sample_repo: String,
}

impl LocalConfig {
    fn repo_path(&self) -> PathBuf {
        self.base_git_path.join(&self.owner)
    }
}

pub(crate) struct LocalProvider {
    config: LocalConfig,
}

impl LocalProvider {
    pub fn new(config: LocalConfig) -> Self {
        Self { config }
    }
}

impl LocalProvider {
    pub(crate) fn clone_sample(
        &self,
        name: &str,
    ) -> Result<PathBuf, LocalError> {
        self.clone(
            &self.config.sample_repo,
            self.config.repo_path().join(name).as_path(),
        )
    }

    pub(crate) fn clone<P: AsRef<Path> + Copy>(
        &self,
        url: &str,
        to: P,
    ) -> Result<PathBuf, LocalError> {
        // TODO: check if folder (to) exist if so just pull
        debug!("cloning repo from {} to {}", url, to.as_ref().display());

        let mut builder = RepoBuilder::new();
        let mut fetch_options = FetchOptions::new();
        let mut remote_callbacks = RemoteCallbacks::new();

        // mount credentials based on ssh
        remote_callbacks.credentials(|_url, _username, _allowed| {
            let credentials = git2::Cred::ssh_key(
                &self.config.git_user,
                None,
                self.config.ssh_prv_key.as_path(),
                None,
            );
            credentials
        });
        fetch_options.remote_callbacks(remote_callbacks);
        builder.fetch_options(fetch_options);
        let repo = builder.clone(url, to.as_ref())?;

        // TODO: check if folder exist because has clone no has folder
        debug!("{} cloned in to {}", url, to.as_ref().display());

        Ok(repo.workdir().unwrap().to_path_buf())
    }

    pub(crate) fn icp(
        &self,
        repo_path: &str,
        repo_url: &str,
        user: &str,
        ssh_priv_key_path: &Path,
    ) -> Result<(), LocalError> {
        // Inicializa um novo repositório Git ou abre um existente
        let repo = Repository::init(repo_path)?;
        info!("Repositório Git inicializado com sucesso em {}", repo_path);

        // Adiciona todos os arquivos ao staging
        let mut index = repo.index()?;
        index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
        let oid = index.write_tree()?;
        let tree = repo.find_tree(oid)?;

        // Realiza o commit inicial
        let signature = Signature::now("mps", "mps@mps.com")?;
        let commit_id = repo.commit(
            Some("HEAD"), // Atualiza a cabeça (HEAD)
            &signature,
            &signature,
            "Initial commit",
            &tree,
            &[],
        )?;

        info!("Commit inicial realizado com sucesso. ID: {}", commit_id);

        // Configura as credenciais para o envio ao repositório remoto
        let mut remote_callbacks = RemoteCallbacks::new();
        remote_callbacks.credentials(|_url, _username, _allowed| {
            let credentials =
                git2::Cred::ssh_key(user, None, ssh_priv_key_path, None);
            credentials
        });

        // Adiciona um novo remoto
        let mut remote = repo.remote("origin", repo_url)?;

        // Cria opções de envio
        let mut push_options = PushOptions::new();
        push_options.remote_callbacks(remote_callbacks);

        // Realiza o push para o repositório remoto
        remote.push(
            &["refs/heads/main:refs/heads/main"],
            Some(&mut push_options),
        )?;

        info!("Push realizado com sucesso!");
        Ok(())
    }

    pub(crate) fn create_repository<P: AsRef<Path> + Display + Copy>(
        path: P,
    ) -> Result<Repository, LocalError> {
        debug!("creating repo from on {}", path);
        let repo = Repository::init(path)?;
        debug!("repo created on {}", path);
        Ok(repo)
    }

    fn pull() {
        // Caminho do repositório local
        let repo_path = Path::new("/caminho/do/seu/repo");

        // Abrir o repositório
        let repo = match Repository::open(repo_path) {
            Ok(repo) => repo,
            Err(e) => panic!("Falha ao abrir o repositório: {}", e),
        };

        // Configurar callbacks remotos para autenticação SSH
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, username, _allowed| {
            let username = username.expect("Username not provided");
            Cred::ssh_key_from_agent(username)
        });

        // Configurar opções de fetch com callbacks remotos
        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);

        // Encontrar o controle remoto "origin"
        let mut remote = match repo.find_remote("origin") {
            Ok(remote) => remote,
            Err(e) => {
                panic!("Falha ao encontrar o controle remoto 'origin': {}", e)
            }
        };

        // Realizar o pull
        match remote.fetch(&["master"], Some(&mut fetch_options), None) {
            Ok(_) => println!("Pull realizado com sucesso!"),
            Err(e) => eprintln!("Erro ao realizar o pull: {}", e),
        }
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
