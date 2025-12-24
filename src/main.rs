fn main() {
// ============================================== PARTE 1 — Classificação com match ==============================================

    let nota = 6;
    let resultado = match nota {
        0..=4 => "Reprovado",
        5..=6 => "Recuperação",
        7..=10 => "Aprovado",
        _ => "Nota invalida",
    };
    println!("resultado: {}", resultado);
// ============================================== PARTE 2 — Par ou Ímpar com if ==============================================

    let zula = 108;
    let result = if zula % 2 == 0 {
        "Par"
    } else {
        "Impar"
    };
    println!("Resultado: {}", result);
// ============================================== PARTE 3 — Contagem com for ==============================================

    for i in 0..=5 {
        println!("i: {}", i);
    };
// ============================================== PARTE 4 — Contagem regressiva com while ==============================================
    let mut contador = 3; 

    while contador >= 0 {
        println!("contador : {}", contador);
        contador -= 1;
    }

// ============================================== PARTE 5 — loop com retorno ============================================== 

    let divisor = loop {
        let x = 8;
        break x / 4;
    };
    println!("divisor: {}", divisor);
}  