[Ver Código Final](https://github.com/jotavetech/tech4all/tree/main/binary_decimal)

## Como Funciona a Conversão de Binários Para Decimais?

Para converter um número na base 2 (binário) para um número na base 10 (decimal), multiplicamos cada digito do número binário pela potência de 2 relativa à posição que ele está ocupando:

Exemplo, no número binário 1010, pegamos as posições de trás pra frente começando do 0:

- 0: Posição 0
- 1: Posição 1
- 0: Posição 2
- 1: Posição 3

Agora o calculo ficaria assim: 0 _ 2⁰ + 1 _ 2¹ + 0 _ 2² + 1 _ 2³ = 10 no sistema decimal.

## Mão na Massa

Pronto, sabendo como funciona a conversão dos sistemas numéricos vamos criar o nosso programa usando Rust. Se você preferir, sinta-se a vontade para fazer na sua linguagem de programação favorita.

Se quiser instalar o Rust, entre aqui: https://rust-book.cs.brown.edu/.

### Criando Projeto

Em nosso terminal, vamos criar o projeto usando o comando `cargo new binary_decimal`, isso irá criar um novo programa em rust usando o [gerenciador de pacotes do Rust](https://github.com/jotavetech/the-rust-book-ptbr/blob/main/02%20-%20Come%C3%A7ando/02%20-%20Hello%2C%20Cargo!.md), que nos facilita na hora de criar programas ou fazer a sua build.

Para rodar o programa, usamos `cargo run` dentro da pasta criada.

Após isso vamos acessar a nova pasta _binary_decimal_ que ele criou e vamos começar a escrever o nosso código dentro do arquivo _main.rs_ na pasta _src_.

Dentro do arquivo _main.rs_ você vai ver o seguinte código:

```rust
fn main() {
	println!("Hello, world!");
}
```

O programa em Rust sempre vai procurar a função principal `main` para saber onde começa o seu código, tudo que fizermos vai começar dentro dela. Agora, no escopo da função `main` temos o código `println!("Hello, world!");`, um macro do Rust(não vamos nos aprofundar na diferença entre macros e funções.) que vai imprimir _Hello, world!_ na tela assim que você executar o programa.

### Escrevendo as Primeiras Linhas

Primeiro, no topo do nosso arquivo _main.rs_ vamos importar um módulo da biblioteca padrão do Rust, que vai nos permitir ler entradas dos usuários:

```rust
use std::io;

fn main() {
	//.....
}
```

Agora, vamos escrever as nossas primeiras linhas do código:

```rust
use std::io;

fn main() {
    println!("Olá, escreva um número binário: ");

    let mut binary_number = String::new();
    let mut converted_numbers: Vec<u32> = Vec::new();

    io::stdin().read_line(&mut binary_number).unwrap();
}
```

Primeiro, chamamos o macro `println!` para imprimir _"Olá, escreva um número binário:"_ no terminal, e em seguida com `let mut binary_number = String::new();` criamos uma variável que poderá armazenar strings (textos) dinâmicas que podem crescer e diminuir de tamanho. Também usamos `let mut converted_numbers: Vec<i32> = Vec::new();` para criar um vetor de tamanho dinâmico que armazena inteiros. Note que colocamos a palavra-chave `mut` antes do nome da variável, isso diz ao Rust que ela pode e vai ser modificada mais pra frente no nosso código, já que variáveis são imutáveis por padrão em Rust.

Na linha `io::stdin().read_line(&mut binary_number).unwrap();` usamos método `read_line` dentro do módulo `io` para ler o que o usuário digitar no terminal, depois `(&mut binary_number)` diz ao Rust para pegar o valor digitado no terminal e armazenar esse valor no endereço que memória que a variável `binary_number` aponta, se o usuário digitar 10 então a variável `binary_number` estará apontando para um endereço na memória com o valor 10.

### Convertendo e Armazenando Valores Binários para Decimais

Vamos adicionar mais algumas linhas:

```rust
use std::io;

fn main() {
    println!("Olá, escreva um número binário: ");

    let mut binary_number = String::new();
    let mut converted_numbers: Vec<u32> = Vec::new();

    io::stdin().read_line(&mut binary_number).unwrap();

    let mut pos = 0;

    for binary in binary_number.trim().chars().rev() {
	if binary == '1' {
            let result = 1 * 2_u32.pow(pos);
            converted_numbers.push(result);
        }
        pos += 1;
    }
}
```

Como aprendemos no começo, cada digito em um número binário possuí uma posição, que começa em 0 da esquerda para a direita, então criamos a variável `pos` que será iniciada em 0 e servirá como controle da posição em que estivermos olhando para fazer a multiplicação.

Após isso criamos um loop que passará por todos os caracteres armazenados na variável `binary_number`, como precisamos cuidar disso de trás pra frente usamos o método `.rev()` para inverter a string, também usamos o método `.chars()` para dizer que queremos lidar com os _caracteres_ da string, e o método `.trim()` para remover os espaços em branco da string, que são adicionados quando clicamos enter para enviar, já que é um caractere válido de escape mas não queremos ele pois causaria mais problemas.

A cada caractere que passarmos no loop vamos verificar se ele é do valor `1`, usando `if binary == '1'` e, caso seja, então vamos fazer o nosso calculo multiplicando 1 _ 2 elevado a posição em que o número está, que é controlada pela variável `pos`, fazemos isso armazenando o resultado em uma variável `result` fazendo a elevação com o método `pow()` com ```let result = 1 _ 2_u32.pow(pos);`. Precisamos guardar os valores em algum lugar, já que no final de tudo precisamos somar todos os resultados para ter o nosso valor em decimal, então na linha `converted_numbers.push(result);`armazenamos o resultado da operação no nosso vetor`converted_numbers`, e, logo após, incrementamos a variável `pos`com`pos += 1;``` para termos o valor da próxima posição, que aumenta de 1 em 1.

Exemplo:

Se o usuário digitar 1011, vamos fazer o loop de trás para frente. Na primeira iteração, o valor será 1 e a posição será 0. Assim, será multiplicado 1 por 2 elevado a 0, resultando em 1, que será armazenado. Em seguida, incrementamos o valor da posição em 1 e avançamos para a próxima iteração, onde o valor 1 será novamente 1 e a posição será 1, resultando em 1 multiplicado por 2 elevado a 1. Novamente, incrementamos a posição, chegando à terceira iteração, onde o valor é 0 e a posição é 2. Nessa etapa, teremos 0 multiplicado por 2 elevado a 2. Esse processo continua até percorrer todos os dígitos fornecidos pelo usuário, calculando o valor decimal correspondente à representação binária.

### Somando os Valores Armazenados e Imprimindo na Tela

Escrevendo o resto do nosso código, a versão final ficará assim:

```rust
use std::io;

fn main() {
    println!("Olá, escreva um número binário: ");

    let mut binary_number = String::new();
    let mut converted_numbers: Vec<u32> = Vec::new();

    io::stdin().read_line(&mut binary_number).unwrap();

    let mut pos = 0;

    for binary in binary_number.trim().chars().rev() {
	if binary == '1' {
            let result = 1 * 2_u32.pow(pos);
            converted_numbers.push(result);
	}
	pos += 1;
    }

    let decimal_number = convert_to_decimal(&mut converted_numbers);

    println!(
	"O número binário {} em decimal é {}",
	binary_number, decimal_number
    );
}

fn convert_to_decimal(numbers: &mut Vec<u32>) -> u32 {
    let mut result = 0;

    for number in numbers {
        result += number.clone();
    }

    result
}
```

Primeiro, criamos fora da função `main`, uma nova função chamada `convert_to_decimal` que receberá um parâmetro chamado `numbers` que recebe uma referência a um vetor de inteiros.

Em `(numbers: &mut Vec<u32>)` dizemos o tipo do parâmetro e na parte `-> u32` dizemos o que a função deverá retornar, no nosso caso, um inteiro.

Dentro da nova função criamos uma variável `result` que ficará responsável com o nosso calculo de todos os valores armazenados dentro do vetor que recebemos, para fazer isso iteramos sobre todos os números dentro de `numbers` com um loop, somando o valor da iteração atual com o valor de `result`, assim: `result += number.clone();`. O método `.clone()` faz com que o número seja copiado para result e não passe a sua referencia para o lugar em que está armazenado no valor, evitando bugs, e alterações indesejadas.

Por fim, criamos a variável `decimal_number` que recebe o resultado da função `convert_to_decimal` que recebe como argumento a referência do vetor que queremos iterar. `let decimal_number = convert_to_decimal(&mut converted_numbers);`.

Agora `decimal_number` tem o resultado do número convertido para decimal e imprimimos isso na tela com o macro `println!`:

```rust
println!(
		"O número binário {} em decimal é {}",
		binary_number, decimal_number
	);
```

[Código Final](https://github.com/jotavetech/tech4all/tree/main/binary_decimal) Aqui está o código final do artigo de hoje!

## FIM!

Esse é o primeiro artigo que eu faço, espero que tenham gostado e aprendido algo novo, estou sempre aberto para sugestões e dicas em que posso melhorar <3

Se curtiu, tenho alguns links que possam interessar:

[The Rust Book PT-BR](https://github.com/jotavetech/the-rust-book-ptbr), essa é a versão traduzida do livro oficial de Rust, tento simplificar e traduzir pra poder aumentar um pouco do público que quer aprender essa linguagem.

[tech4all](https://github.com/jotavetech/tech4all) Meu repositório onde pretendo guardar os códigos que escrevo nos artigos, como esse. Lá você pode encontrar esse conversor também!

[Código Final](https://github.com/jotavetech/tech4all/tree/main/binary_decimal) Aqui está o código final do artigo de hoje!

[Binário para Decimal](https://marco.uminho.pt/~joao/Computacao2/node6.html) Aqui foi onde peguei a explicação sobre como converter binários para decimal.

VALEU ❤️
