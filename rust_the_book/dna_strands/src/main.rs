fn main() {
    println!("{}", dna_strand("AAAA"));
    println!("{}", dna_strand("ATTGC"));
    println!("{}", dna_strand("GTAT"));
}

fn dna_strand(dna: &str) -> String {
    // Translate the DNA strand
    let mut strand = String::new();
    for x in dna.chars() {
        match x {
            'A' => strand.push('T'),
            'T' => strand.push('A'),
            'G' => strand.push('C'),
            'C' => strand.push('G'),
            _ => break,
        }
    }
    strand
}
