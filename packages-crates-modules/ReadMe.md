# ReadMe

`Packages`: A Cargo feature that lets you build, test, and share crates
`Crates`: A tree of modules that produces a library or executable
`Modules and use`: Let you control the organization, scope, and privacy of paths
`Paths`: A way of naming an item, such as a struct, function, or module

Voce pode usar o caminho 

`use nome_projeto::fruits::apples::apple`

Ou declarar um modulo dentro de um arquivo mod.rs e usar esse modulo dentro do main

Para declarar duas rotas nos podemos fazer

```rust
use std::io;
use std::io::Write;
```

ou 

```rust
use std::io::{self, Write};
```

Para trazer todos os items de um caminho fazemos

```rust
use std::collections::*;
```
