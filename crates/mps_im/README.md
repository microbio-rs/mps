# mps_im

Microbe platform system - instant messaging


## slack webhook bot command

```json
{
  "token": "seu_token",
  "team_id": "T012AB3CD",
  "team_domain": "sua_equipe",
  "channel_id": "C01234567",
  "channel_name": "geral",
  "user_id": "U01234567",
  "user_name": "usuario_exemplo",
  "command": "/seu_comando",
  "text": "argumentos_do_comando",
  "response_url": "https://hooks.slack.com/commands/T012AB3CD/12345678901234567890",
  "trigger_id": "1234567890.1234567890.abcdefghijklmnopqrstuvwxyza"
}
```

- seu_token: O token de verificação que o Slack usa para autenticar a solicitação.
- T012AB3CD: O ID da equipe Slack.
- C01234567: O ID do canal onde o comando foi executado.
- U01234567: O ID do usuário que executou o comando.
- /seu_comando: O nome do seu comando Slash.
- argumentos_do_comando: Os argumentos fornecidos com o comando.
- https://hooks.slack.com/commands/T012AB3CD/12345678901234567890: A URL de resposta onde você envia a resposta do seu comando.
- 1234567890.1234567890.abcdefghijklmnopqrstuvwxyza: O ID do gatilho para a ação do comando.
