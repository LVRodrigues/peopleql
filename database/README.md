# PostgreSQL

Banco de dados PostgreSQL para o aplicativo **Guess Who?** Aqui é descrito a criação de
um banco de dados para desenvolvimento e como acessá-lo.

> Foi colocado em um novo repositório para que possa ser compartilhado em outros projetos.

## Docker

O arquivo **Dockerfile** configura um banco de dados para uso no sistema **Guess Who?**.

Para criar a imagem Docker para o projeto, execute:

```bash
docker build -f Dockerfile -t guess-who/database .
```

Para executar a imagem com uma base de dados volátil:

```bash
docker run --rm -p 5432:5432/tcp guess-who/database
```

Para executar a imagem com uma base de dados persistente:

```bash
docker run --name guess-who-database -p 5432:5432 guess-who/database
docker start guess-who-database
docker stop guess-who-database
```

## Acessar o Banco de Dados

São criados dois usuários:

1. **guess-admin**: usuário com direitos de DBA.
2. **guess-user**: usuário para operação do sistema.

Nos dois casos, a senha é **guess-pass**.

O banco de dados tem o nome **guess-who** e é acessível pelo endereço **localhost**, na porta **5432**.

> Mesmo durante o devesvolvimento, recomenda-se o uso do usuário **guess-user**.

## Reutilizar em outros projetos (repositórios)

Se algum outro projeto desejar utilizar este banco de dados, no repositório do novo projeto, utilize os comandos:

```bash
> mkdir database
> git remote add -f database https://github.com/LVRodrigues/guess-who-database.git
> git pull -s subtree database main
```
