![Rust CI](https://github.com/rodrigopereiradevelopment/passaudit/actions/workflows/main.yml/badge.svg)

# PassAudit 🦀

**PassAudit** é um auditor de senhas via linha de comando (CLI) focado em segurança e privacidade, desenvolvido inteiramente em ambiente mobile (**Termux/Android**).

A ferramenta combina análise de entropia local com verificações de vazamentos globais, garantindo que suas senhas sejam fortes e seguras sem nunca expor seus dados sensíveis.

---

## 🛡️ Diferenciais Técnicos

* **Privacidade com K-Anonymity:** A verificação de vazamentos utiliza a API *Have I Been Pwned*. Para sua segurança, apenas os 5 primeiros caracteres do hash SHA-1 da senha são enviados para a rede.
* **Algoritmo zxcvbn:** Estimativa de força baseada em entropia real, considerando padrões de teclado, nomes comuns e dicionários.
* **Segurança de Memória:** Desenvolvido em **Rust**, garantindo performance e proteção contra vulnerabilidades comuns de memória.
* **Interface Segura:** Entrada de senha oculta no terminal para evitar exposição visual (*shoulder surfing*).

## 🛠️ Tecnologias Utilizadas

* **Rust (Edição 2021)**
* **Clap:** Gestão de argumentos de linha de comando.
* **Reqwest:** Chamadas de API HTTP seguras.
* **zxcvbn-rust:** Algoritmo de análise de força de senha.
* **Colored & Inquire:** Interface de usuário no terminal (CLI).

## 🚀 Como Executar no Termux

Certifique-se de ter o Rust e o Cargo instalados:

```bash
# Clone o repositório
git clone https://github.com/rodrigopereiradevelopment/passaudit
cd passaudit

# Execute com verificação de vazamento e sugestão de senha forte
cargo run -- --leak-check --suggest


⚖️ Licença
​Este projeto está sob a licença MIT. Veja o arquivo LICENSE para detalhes.
​Desenvolvido como parte do aprendizado em Segurança Cibernética e Desenvolvimento de Sistemas na ETEC Pedro Ferreira Alves.
