mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod d8;

fn main() {
    //Day 1
    println!("Day 1 - ver1 - part1, le mot de passe est: {}", d1::v1::solve_part1(d1::INPUT));
    println!("Day 1 - ver2 - part1, le mot de passe est: {}", d1::v2::solve_part1(d1::INPUT));

    println!("Day 1 - ver1 - part2, le mot de passe est: {}", d1::v1::solve_part2(d1::INPUT));
    println!("Day 1 - ver2 - part2, le mot de passe est: {}", d1::v2::solve_part2(d1::INPUT));

    //Day 2
    println!("Day 2 - ver1 - part1, la somme des invalides est: {}", d2::v1::solve_part1(d2::INPUT));
    println!("Day 2 - ver2 - part1, la somme des invalides est: {}", d2::v2::solve_part1(d2::INPUT));

    println!("Day 2 - ver1 - part2, le résultat: {}", d2::v1::solve_part2(d2::INPUT));

    //Day 3
    println!("Day 3 - ver1 - part1, le résultat: {}", d3::v1::solve_part1(d3::INPUT));
    println!("Day 3 - ver1 - part2, le résultat: {}", d3::v1::solve_part2(d3::INPUT));

    //Day 4
    println!("Day 4 - ver1 - part1, le résultat: {}", d4::v1::solve_part1(d4::INPUT));
    println!("Day 4 - ver1 - part2, le résultat: {}", d4::v1::solve_part2(d4::INPUT));

    //Day 5
    println!("Day 5 - ver1 - part1, le résultat: {}", d5::v1::solve_part1(d5::INPUT));
    println!("Day 5 - ver1 - part2, le résultat: {}", d5::v1::solve_part2(d5::INPUT));

  
    //Day 6
    println!("Day 6 - ver1 - part1, le résultat: {}", d6::v1::solve_part1(d6::INPUT));
    println!("Day 6 - ver1 - part2, le résultat: {}", d6::v1::solve_part2(d6::INPUT));

    //Day 7
    println!("Day 7 - ver1 - part1, le résultat: {}", d7::v1::solve_part1(d7::INPUT));
    println!("Day 7 - ver1 - part2, le résultat: {}", d7::v1::solve_part2(d7::INPUT));

    //Day 8
    println!("Day 8 - ver1 - part1, le résultat: {}", d8::v1::solve_part1(d8::INPUT));
}