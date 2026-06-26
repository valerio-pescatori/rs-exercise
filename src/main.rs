mod social_golfer;
use crate::social_golfer::valid;

fn main() {
    let s: Vec<Vec<&str>> = vec![
         vec!["ABCD", "EFGH", "IJKL", "MNOP", "QRST"], 
         vec!["AEIM", "BJOQ", "CHNT", "DGLS", "FKPR"],
         vec!["AGKO", "BIPT", "CFMS", "DHJR", "ELNQ"],
         vec!["AHLP", "BKNS", "CEOR", "DFXQ", "GJMT"],
         vec!["AFJN", "BLMR", "CGPQ", "DEKT", "HIOS"]
    ];
    // should be false

    println!("valid: {:?}", valid(s));
}
