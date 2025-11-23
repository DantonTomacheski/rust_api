# Rust Solo API

Uma API REST simples desenvolvida em Rust usando o framework Axum, demonstrando conceitos fundamentais de desenvolvimento de APIs web com Rust.

## 📋 Sobre o Projeto

Este projeto é uma API HTTP que implementa autenticação básica via endpoint de login. Desenvolvido com foco em aprendizado e demonstração de boas práticas com Rust e programação assíncrona.

## 🚀 Tecnologias Utilizadas

- **[Rust](https://www.rust-lang.org/)** - Linguagem de programação
- **[Axum](https://github.com/tokio-rs/axum)** - Framework web ergonômico e modular
- **[Tokio](https://tokio.rs/)** - Runtime assíncrono para Rust
- **[Serde](https://serde.rs/)** - Serialização e deserialização de dados
- **[Tracing](https://github.com/tokio-rs/tracing)** - Logging e instrumentação

## 📦 Dependências

```toml
axum = "0.8.7"              # Framework web
tokio = "1.48.0"            # Runtime assíncrono
serde = "1.0.228"           # Serialização/Deserialização
serde_json = "1.0"          # Suporte JSON
tracing = "0.1.41"          # Logging
tracing-subscriber = "0.3.20" # Subscriber para tracing
tower-http = "0.6.6"        # Middleware HTTP
tower-cookies = "0.11.0"    # Gerenciamento de Cookies
uuid = "1.18.1"             # Geração de UUIDs
```

### Dev Dependencies

```toml
httpc-test = "0.1.10"       # Testes HTTP
anyhow = "1.0.100"          # Tratamento de erros
```

## 🏗️ Estrutura do Projeto

```
rust-solo/
├── src/
│   ├── main.rs              # Ponto de entrada da aplicação
│   ├── error.rs             # Definições de erros (versão 1)
│   ├── error2.rs            # Definições de erros (versão 2)
│   ├── web/                 # Módulo web (versão 1)
│   │   ├── mod.rs
│   │   └── routes_login.rs
│   └── web2/                # Módulo web (versão 2)
│       ├── mod.rs
│       └── routes_login2.rs
├── tests/                   # Testes de integração
│   └── quick_dev.rs         # Testes rápidos de desenvolvimento
├── Cargo.toml               # Configuração do projeto
└── README.md                # Este arquivo
```

## 🔧 Instalação

### Pré-requisitos

- Rust 1.70 ou superior
- Cargo (geralmente instalado junto com Rust)

### Instalar Rust

Se você ainda não tem Rust instalado, visite [rustup.rs](https://rustup.rs/) ou execute:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Clonar e Executar

```bash
# Clone o repositório
git clone <url-do-repositorio>
cd rust-solo

# Compilar o projeto
cargo build

# Executar em modo de desenvolvimento
cargo run

# Executar em modo release (otimizado)
cargo run --release
```

## 🎯 Uso

### Iniciar o Servidor

```bash
cargo run
```

O servidor iniciará em `http://127.0.0.1:8080`

### Endpoint Disponível

#### POST `/api/login`

Endpoint de autenticação básica.

**Request:**

```json
{
  "name": "Danton",
  "pwd": "213"
}
```

**Response (Sucesso - 200):**

```json
{
  "result": {
    "success": true
  }
}
```

_Nota: A resposta também define um cookie `user-1.exp.sign`._

**Response (Erro - 401):**

```
You dont have permission
```

#### GET `/hello/{name}`

Endpoint simples de saudação.

**Response (200):**

```html
Name is: {name}
```

### Exemplo com cURL

```bash
curl -X POST http://127.0.0.1:8080/api/login \
  -H "Content-Type: application/json" \
  -d '{"name":"Danton","pwd":"213"}'
```

### Exemplo com PowerShell

```powershell
Invoke-RestMethod -Uri http://127.0.0.1:8080/api/login `
  -Method Post `
  -ContentType "application/json" `
  -Body '{"name":"Danton","pwd":"213"}'
```

## 📝 Funcionalidades

- ✅ Servidor HTTP assíncrono
- ✅ Endpoint de autenticação
- ✅ Gerenciamento de Cookies
- ✅ Middleware Customizado
- ✅ Logging com tracing
- ✅ Tratamento de erros customizado
- ✅ Serialização/Deserialização JSON

## 🔍 Detalhes Técnicos

### Sistema de Erros

O projeto implementa um sistema de erros customizado que integra com o Axum:

```rust
pub enum Error {
    LoginFail,
}
```

Cada erro é convertido automaticamente em uma resposta HTTP apropriada através da trait `IntoResponse`.

### Logging

O projeto utiliza o `tracing` para logging estruturado. Os logs são configurados no nível `INFO` e incluem informações sobre:

- Endereço do servidor
- Payloads de requisições
- Eventos do sistema

## 🛠️ Desenvolvimento

### Executar Testes

```bash
# Executar todos os testes
cargo test

# Executar teste rápido de desenvolvimento
cargo test --test quick_dev -- --nocapture
```

### Verificar Código

```bash
# Verificar erros de compilação
cargo check

# Verificar formatação
cargo fmt --check

# Executar linter
cargo clippy
```

### Build de Produção

```bash
cargo build --release
```

O binário compilado estará em `target/release/rust-solo`

## 📚 Aprendizados

Este projeto demonstra:

- Configuração básica de um servidor Axum
- Roteamento HTTP
- Handlers assíncronos
- Deserialização de JSON com Serde
- Sistema de erros customizado
- Logging e observabilidade com Tracing
- Boas práticas de estruturação de projetos Rust

## 🤝 Contribuindo

Contribuições são bem-vindas! Sinta-se à vontade para:

1. Fork o projeto
2. Criar uma branch para sua feature (`git checkout -b feature/MinhaFeature`)
3. Commit suas mudanças (`git commit -m 'Adiciona MinhaFeature'`)
4. Push para a branch (`git push origin feature/MinhaFeature`)
5. Abrir um Pull Request

## 📄 Licença

Este projeto está sob a licença MIT. Veja o arquivo LICENSE para mais detalhes.

## 👤 Autor

**Danton Tomacheski**

---

⭐ Se este projeto foi útil para você, considere dar uma estrela!
