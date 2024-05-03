extern crate rand;

use rand::Rng;

fn generate_random_bits(length: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..length).map(|_| rng.gen_range(0..2)).collect()
}

fn generate_random_bases(length: usize) -> Vec<char> {
    let mut rng = rand::thread_rng();
    (0..length).map(|_| {
        match rng.gen_range(0..2) {
            0 => 'H',
            _ => 'V',
        }
    }).collect()
}

fn encode_bits(bits: &Vec<u8>, bases: &Vec<char>) -> Vec<String> {
    bits.iter().zip(bases.iter()).map(|(&bit, &basis)| {
        match basis {
            'H' => if bit == 0 { "|H⟩".to_string() } else { "|V⟩".to_string() },
            'V' => if bit == 0 { "|D⟩".to_string() } else { "|A⟩".to_string() },
            _ => panic!("Invalid basis"),
        }
    }).collect()
}// Измерение квантовых состояний с использованием линейных поляризаторов

fn measure_states(states: Vec<String>, bases: Vec<char>) -> Vec<u8> {
    states.iter().zip(bases.iter()).map(|(state, &basis)| {
        match basis {
            'H' => if state == "|H⟩" { 0 } else { 1 },
            'V' => if state == "|D⟩" { 0 } else { 1 },
            _ => panic!("Invalid basis"),
        }
    }).collect()
}

fn main() {
    let length = 10;
    let bits = generate_random_bits(length);
    let bases = generate_random_bases(length);
    let states = encode_bits(&bits, &bases);
    let measured_bits = measure_states(states.clone(), bases.clone());

    println!("Переданные биты: {:?}", bits);
    println!("Использованные базисы: {:?}", bases);
    println!("Квантовые состояния: {:?}", states);
    println!("Полученные состояния после канала: {:?}", states);
    println!("Принимающая сторона: {:?}", measured_bits);
}
