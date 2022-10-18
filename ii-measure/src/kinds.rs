use std::str::FromStr;


#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CompressionAlgorithm {
    Baseline,
    DedupHash,
    DedupBTree,

    NSDedup,
    NSRaw,

    SmazDedup,
    SmazRaw,

    FastPforDedup,

    SmazFastPforDedup,
    SmazNSDedup,
}

impl CompressionAlgorithm {
    fn lookup() -> Vec<(CompressionAlgorithm, &'static str)> {
        use CompressionAlgorithm::*;
        vec![
            (Baseline, "baseline"),
            (DedupHash, "dedup_hash"),
            (DedupBTree, "dedup_btree"),
            (NSDedup, "ns+dedup"),
            (NSRaw, "ns_raw"),
            (SmazDedup, "smaz+dedup"),
            (SmazRaw, "smaz_raw"),
            (FastPforDedup, "pfor+dedup"),
            (SmazFastPforDedup, "smaz+pfor+dedup"),
            (SmazNSDedup, "smaz+ns+dedup"),
        ]
    }

    pub fn str(self) -> &'static str {
        CompressionAlgorithm::lookup()
            .into_iter()
            .find_map(|(elem, s)| (elem == self).then(|| s))
            .unwrap()
    }
}

impl FromStr for CompressionAlgorithm {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        CompressionAlgorithm::lookup()
            .into_iter()
            .find_map(|(elem, name)| (name == s).then(|| elem))
            .ok_or_else(|| {
                let mut s = String::from("allowed: ");
                for name in Self::lookup().into_iter().map(|a| a.1) {
                    s += name;
                    s += " ";
                }
                s
            })
    }
}