# Egui CRUD App

Apenas um simples aplicativo CRUD feito em Rust, usando egui e serde.

<b style="color: red;">Atenção:</b> Esse projeto que criei é com o intuito de estudar programação através da linguagem Rust.

## Usando o aplicativo

É um pequeno sistema CRUD, onde você pode adicionar um produto, ler um produto, atualizar um produto e deletar.

Obs.: a pesquisa é feita automaticamente enquanto digita na barra de pesquisa.

![Interface Gráfica](/assets/img/gui.png "Interface Gráfica")

Após preencher os dados, é só clicar em "Add Product" e clicar em "Save Products" para salvar em um arquivo JSON.

![Usabilidade](/assets/img/gui-using.png "Usabilidade")

## Dependências

[eframe](https://crates.io/crates/eframe)

[egui_extras](https://crates.io/crates/egui_extras)

[serde](https://crates.io/crates/serde)

[serde_json](https://crates.io/crates/serde_json)

## Build / Deploy

Para realizar uma build release é só usar o Cargo.

<details>
<summary>Cargo</summary>

> `cargo build --release`

</details>