use std::io;

fn main() {
    let mut dna=String::new();

    println!("Enter DNA STRAND: ");
    io::stdin().read_line(&mut dna).expect("enter dna strand");

    let mut compdna=String::new();

    for c in dna.chars(){
        if c=='A'{
            compdna.push('T');
        }
        else if c=='T'{
            compdna.push('A');
        }
        else if c=='C'{
            compdna.push('G');
        }
        else if c=='G'{
            compdna.push('C');
        }
    }
    println!("{}",compdna);
}
