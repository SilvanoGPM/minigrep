<h1 align="center">Minigrep</h1>

<p align="center">Ferramenta de linha de comando em Rust</p>

## :rocket: Funcionalidades

- Pesquisa eficiente de um padrão de busca em arquivos de texto.
- Suporta flags e variáveis de ambiente para ignorar maiúsculas e minúsculas.
- Exibe os resultados da pesquisa destacando as correspondências encontradas.

## ⚙️ Requisitos

- Rust 1.63.0 ou superior.
- Cargo 1.63.0 ou superior.

## :wrench: Instalação

1. Clone este repositório para o seu sistema local:

```
git clone https://github.com/SilvanoGPM/minigrep.git
```

2. Acesse o diretório do projeto:

```
cd minigrep
```

3. Compile o projeto usando o cargo:

```
cargo build --release
```

Após a compilação bem-sucedida, você encontrará o executável minigrep na pasta target/release. Adicione-o ao seu PATH ou mova-o para um diretório de sua preferência.

## 🤖 Uso

```
minigrep <texto> <arquivo>
```

* `<texto>`: O texto de busca que você deseja encontrar nos arquivos. 

* `<arquivo>`: O arquivo no qual você deseja realizar a busca.

Exemplo:

```
minigrep nobody example.txt
```

Incluir maiúsculas e minúsculas:

```
minigrep nobody example.txt --i
```

Caso em todas as pesquisas deseje incluir maiúsculas e minúsculas, você pode utilizar uma variável de ambiente.

Linux:
```
IGNORE_CASE=1 minigrep nobody example.txt
```

PowerShell:
```
$Env:IGNORE_CASE=1
minigrep nobody example.txt
```

Para remover a variável de ambiente no PowerShell utilize:

```
Remove-Item Env:IGNORE_CASE
```

## :heart: Agradecimentos

- [The Rust Programming Language](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)

