# Interface para apuração de pontos
Sistema é um experimento de programa feito usando puro Rust e a crate iced.

## Objetivos
- Interface Grafica nativa.
- Leitura de arquivo AFD e levantamento.
- Alta performance.

## Estrutura do projeto
```text
src/
├── main.rs    # main do projeto
├── app/       # estado global, update e message
├── domain/    # lógica de negócio
├── infra/     # acesso a dados e integrações
├── ui/        # telas e views do iced
├── services/  # APIs externas
├── utils/     # utilidades
└── docs/      # documentação
```

## Como usar?
```Bash
git clone https://github.com/Joao-Santin/interface_rh2
cargo build
cargo run
```

