use std::io;

fn main() {
    println!("Enter wire colors separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let wires: Vec<String> = input.trim().split_whitespace().map(String::from).collect();
    
    println!("Enter the last digit of the serial number:");
    let mut serial_input = String::new();
    io::stdin().read_line(&mut serial_input).expect("Failed to read line");
    let serial_last_digit: u32 = serial_input.trim().parse().expect("Please enter a valid number");

    let wire_to_cut = decide_wire_to_cut(&wires, serial_last_digit);
    println!("Cut wire number: {}", wire_to_cut);
}

fn decide_wire_to_cut(wires: &[String], serial_last_digit: u32) -> usize {
    match wires.len() {
        3 => cut_three_wires(wires),
        4 => cut_four_wires(wires, serial_last_digit % 2 != 0),
        5 => cut_five_wires(wires, serial_last_digit % 2 != 0),
        6 => cut_six_wires(wires, serial_last_digit % 2 != 0),
        _ => panic!("Unsupported number of wires"),
    }
}

fn cut_three_wires(wires: &[String]) -> usize {
    if !wires.iter().any(|w| w == "r") {
        2
    } else if wires.last().unwrap() == "w" {
        3
    } else if wires.iter().filter(|&w| w == "bl").count() > 1 {
        wires.iter().rposition(|w| w == "bl").unwrap() + 1
    } else {
        3
    }
}

fn cut_four_wires(wires: &[String], serial_odd: bool) -> usize {
    let r_wires = wires.iter().filter(|&w| w == "r").count();
    if r_wires > 1 && serial_odd {
        wires.iter().rposition(|w| w == "r").unwrap() + 1
    } else if wires.last().unwrap() == "y" && r_wires == 0 {
        1
    } else if wires.iter().filter(|&w| w == "bl").count() == 1 {
        1
    } else if wires.iter().filter(|&w| w == "y").count() > 1 {
        4
    } else {
        2
    }
}

fn cut_five_wires(wires: &[String], serial_odd: bool) -> usize {
    if wires.last().unwrap() == "bk" && serial_odd {
        4
    } else if wires.iter().filter(|&w| w == "r").count() == 1 && wires.iter().filter(|&w| w == "y").count() > 1 {
        1
    } else if !wires.iter().any(|w| w == "bk") {
        2
    } else {
        1
    }
}

fn cut_six_wires(wires: &[String], serial_odd: bool) -> usize {
    if !wires.iter().any(|w| w == "y") && serial_odd {
        3
    } else if wires.iter().filter(|&w| w == "y").count() == 1 && wires.iter().filter(|&w| w == "white").count() > 1 {
        4
    } else if !wires.iter().any(|w| w == "r") {
        6
    } else {
        4
    }
}