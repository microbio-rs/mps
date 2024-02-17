# mps_scm

Microbe platform system - source system control

## Criando um Token de Acesso Pessoal no GitHub

1. Acesse o GitHub e faça login na sua conta.
2. No canto superior direito, clique na sua foto de perfil e selecione "Settings" (Configurações).
3. No menu lateral esquerdo, selecione "Developer settings" (Configurações de desenvolvedor).
4. No submenu que aparece, selecione "Personal access tokens" (Tokens de acesso pessoal).
5. Clique no botão "Generate new token" (Gerar novo token de acesso pessoal).
6. Você será solicitado a fornecer o nome do token e selecionar as permissões que deseja conceder a ele. As permissões controlam o acesso que o token terá às suas contas e repositórios do GitHub.
7. Selecione as permissões necessárias com base nas operações que o seu aplicativo precisa realizar. É recomendável conceder apenas as permissões necessárias para minimizar os riscos de segurança.
8. Depois de selecionar as permissões, clique no botão "Generate token" (Gerar token) na parte inferior da página.
9. Após a geração, o GitHub exibirá o token de acesso pessoal. Atenção: Copie o token imediatamente, pois ele só será exibido uma vez por razões de segurança. Se você perder o token, precisará gerar um novo.
10. Guarde o token em um local seguro. Nunca compartilhe seu token de acesso pessoal e evite incluí-lo em código fonte ou repositórios públicos.
11. Depois de copiar o token, clique no botão "Done" (Concluído) para finalizar o processo.

## create repo

```console
grpcurl -plaintext -import-path ./crates/mps_scm/proto -proto scm.proto -d '{"name": "mps-test-repo"}' '[::1]:50051' scm.Scm/CreateRepo
```

## To Do

* [ ] github settings default branch
* [ ] github settings features remove wiki, issues, projects, preserve this repository
* [ ] github settings pr: allow/disable (merge,squas,rebase) commits
* [ ] github settings pr: always suggest updating
* [ ] github settings pr: allow/auto merge
* [ ] github settings pr: automatic delete branch
* [ ] github settings branch: branch protection
* [ ] github settings webhook: add
* [ ] github settings ruleset (branch,tag)
