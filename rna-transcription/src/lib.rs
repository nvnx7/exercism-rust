#[derive(Debug, PartialEq)]
pub struct Dna(String);

#[derive(Debug, PartialEq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        match dna
            .chars()
            .enumerate()
            .try_fold(String::new(), |acc, (idx, c)| match c {
                'A' | 'C' | 'G' | 'T' => Ok(format!("{}{}", acc, c)),
                _ => Err(idx),
            }) {
            Ok(nucleotides) => Ok(Self(nucleotides)),
            Err(idx) => Err(idx),
        }
    }

    pub fn into_rna(self) -> Rna {
        Rna(self
            .0
            .chars()
            .map(|c| match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => panic!("Unexpected dna nucleotide received!"),
            })
            .collect::<String>())
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        match rna
            .chars()
            .enumerate()
            .try_fold(String::new(), |acc, (idx, c)| match c {
                'A' | 'C' | 'G' | 'U' => Ok(format!("{}{}", acc, c)),
                _ => Err(idx),
            }) {
            Ok(nucleotides) => Ok(Self(nucleotides)),
            Err(idx) => Err(idx),
        }
    }
}
