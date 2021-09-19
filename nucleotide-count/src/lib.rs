use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    match nucleotide {
        'A' | 'C' | 'G' | 'T' => dna.chars().try_fold(0, |count, c| match c {
            'A' | 'C' | 'G' | 'T' => {
                if c == nucleotide {
                    Ok(count + 1)
                } else {
                    Ok(count)
                }
            }
            _ => Err(c),
        }),
        _ => Err(nucleotide),
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts: HashMap<char, usize> = vec![('A', 0), ('C', 0), ('G', 0), ('T', 0)]
        .into_iter()
        .collect();
    match dna.chars().try_for_each(|c| {
        if let Some(count) = counts.get_mut(&c) {
            *count = *count + (1 as usize);
            Ok(())
        } else {
            Err(c)
        }
    }) {
        Ok(()) => Ok(counts),
        Err(c) => Err(c),
    }
}
