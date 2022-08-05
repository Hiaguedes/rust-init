# Basicaaao

Extensao dos arquivos .rs

Pra rodar usar `rustc hello.rs`

## Criando problemas com cargo

quando instalamos o rust o cargo veio junto (lembra o npm com o node ne uahsuhas) e criamos um novo projeto com o cargo com

```cli
cargo new <name_project>
```

E ele vai criar um projeto com o mesmo nome (parecido com npm init) mas com um arquivo de dependencia (tipo o package.json) chamado de Cargo.toml

Para rodar o projeto dessa pasta basta fazer

```cli
cargo build
./target/debug/hello_cargo
```

Ou simplesmente 

```cli
cargo run
```

Existe como checar se o codigo ta compilavel com `cargo check`
