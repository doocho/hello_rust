use crate::tx::Tx;

type Hash = String;

fn combine_hash(left: &Hash, right: &Hash) -> Hash {
    format!("{left}|{right}")
}

fn merkle_level(current: &[Hash]) -> Vec<Hash> {
    let mut result = Vec::new();
    for i in (0..current.len()).step_by(2) {
        let left = &current[i];
        let right = if i + 1 < current.len() {
            &current[i + 1]
        } else {
            left
        };
        let hash = combine_hash(left, right);
        result.push(hash);
    }
    result
}

fn merkle_root(leaves: &[Hash]) -> Option<Hash> {
    if leaves.is_empty() {
        return None;
    }
    let mut current = leaves.to_vec();
    while current.len() > 1 {
        current = merkle_level(&current);
    }
    Some(current[0].clone())
}

pub struct Block {
    pub tx_hashes: Vec<Hash>,
    pub merkle_root: Hash,
}

impl Block {
    pub fn merkle_root(&self) -> Option<Hash> {
        merkle_root(&self.tx_hashes)
    }

    pub fn is_valid(&self) -> bool {
        let merkle_root_calculated = self.merkle_root();
        if merkle_root_calculated.is_none() {
            return false;
        } else {
            let merkle_root_calculated = merkle_root_calculated.unwrap();
            self.merkle_root == merkle_root_calculated
        }
    }
}

trait Hashable {
    fn hash(&self) -> Hash;
}

fn merkle_root_of<T: Hashable>(items: &[T]) -> Option<Hash> {
    if items.is_empty() {
        return None;
    }
    let hashes: Vec<Hash> = items.iter().map(|item| item.hash()).collect();
    merkle_root(&hashes)
}

impl Hashable for Tx {
    fn hash(&self) -> Hash {
        format!("{}-{}-{}", self.from, self.to, self.amount)
    }
}
