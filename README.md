# 🦀 Mini Projeto em Rust — Controle de Fluxo

Este mini projeto em **Rust** demonstra, de forma prática e organizada, os principais conceitos de **controle de fluxo** da linguagem, incluindo `match`, `if/else` e os loops `for`, `while` e `loop`.

O código está dividido em partes independentes, facilitando o entendimento de cada estrutura.

---

## 📌 Objetivos do Projeto

* Utilizar `match` para classificação de valores
* Aplicar `if` como expressão condicional
* Trabalhar com operadores aritméticos
* Criar loops com `for`, `while` e `loop`
* Entender como loops podem retornar valores

---

## 📄 Código Fonte

```rust
fn main() {
    // ================= PARTE 1 — Classificação com match =================
    let nota = 6;
    let resultado = match nota {
        0..=4 => "Reprovado",
        5..=6 => "Recuperação",
        7..=10 => "Aprovado",
        _ => "Nota invalida",
    };
    println!("resultado: {}", resultado);

    // ================= PARTE 2 — Par ou Ímpar com if =================
    let zula = 108;
    let result = if zula % 2 == 0 {
        "Par"
    } else {
        "Impar"
    };
    println!("Resultado: {}", result);

    // ================= PARTE 3 — Contagem com for =================
    for i in 0..=5 {
        println!("i: {}", i);
    };

    // ================= PARTE 4 — Contagem regressiva com while =================
    let mut contador = 3;
    while contador >= 0 {
        println!("contador : {}", contador);
        contador -= 1;
    }

    // ================= PARTE 5 — loop com retorno =================
    let divisor = loop {
        let x = 8;
        break x / 4;
    };
    println!("divisor: {}", divisor);
}
```

---

## 🧠 Explicação por Partes

### 🔹 Parte 1 — Classificação com `match`

* Usa intervalos (`0..=4`, `7..=10`)
* Classifica a nota do aluno
* Demonstra o `match` como expressão

---

### 🔹 Parte 2 — Par ou Ímpar com `if`

* Utiliza o operador módulo (`%`)
* O `if` retorna um valor para uma variável

---

### 🔹 Parte 3 — Contagem com `for`

* Loop baseado em intervalo
* Executa de 0 até 5 (inclusive)

---

### 🔹 Parte 4 — Contagem regressiva com `while`

* Loop baseado em condição
* Utiliza variável mutável
* Controla manualmente o contador

---

### 🔹 Parte 5 — `loop` com retorno

* Demonstra o `loop` infinito
* Usa `break` para encerrar o loop
* Retorna um valor ao sair do loop

---

## ▶️ Como Executar

1. Verifique se o Rust está instalado:

   ```bash
   rustc --version
   ```

2. Crie um novo projeto:

   ```bash
   cargo new controle_fluxo_completo
   ```

3. Substitua o conteúdo do arquivo `src/main.rs` pelo código acima

4. Execute:

   ```bash
   cargo run
   ```

---

## 🖨️ Saída Esperada (Exemplo)

```
resultado: Recuperação
Resultado: Par
i: 0
i: 1
i: 2
i: 3
i: 4
i: 5
contador : 3
contador : 2
contador : 1
contador : 0
divisor: 2
```

---

## 🚀 Próximos Passos

* Criar funções para cada parte do código
* Utilizar `match` com `enum`
* Combinar loops com condicionais
* Explorar `while let`

---

## 📚 Conclusão

Este mini projeto consolida os principais conceitos de **controle de fluxo em Rust**, mostrando como a linguagem permite escrever código claro, seguro e expressivo.

Excelente exercício para reforçar a base da linguagem!
