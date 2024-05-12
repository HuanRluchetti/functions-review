

fn main() {
    print_labeled_measurement(5, 'h');

    println!("-------------------------");

    let x: i32 = five();

    // Statemants and expressions
    // * Isso aqui e tipo uma arrow function do js
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of x is: {x} {y}");
}

// Parameters

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Functions with Return Values
// No Rust o ultimo valor declarado dentro de uma funcao e o valor retornado da mesma

fn five() -> i32 {
    5 // o retorno 
}

