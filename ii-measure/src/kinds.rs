use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CompressionAlgorithm {
    Baseline,
    BaselineExact,

    DedupHash,
    DedupBTree,

    NSDedup,
    NS,
    NSArena,

    SmazDedup,
    Smaz,

    FrontCodingDedup,

    FastPforExactDedup,

    SmazFastPforDedup,
    SmazNSDedup,
}

impl CompressionAlgorithm {
    fn lookup() -> Vec<(CompressionAlgorithm, &'static str)> {
        use CompressionAlgorithm::*;
        vec![
            (Baseline, "baseline"),
            (BaselineExact, "baselinex"),
            (DedupHash, "dedup_hash"),
            (DedupBTree, "dedup_btree"),
            (NSDedup, "ns+dedup"),
            (NS, "ns"),
            (NSArena, "ns_arena"),
            (Smaz, "smaz"),
            (FrontCodingDedup, "frontcoding+dedup"),
            (SmazDedup, "smaz+dedup"),
            (FastPforExactDedup, "pfor+dedup"),
            (SmazFastPforDedup, "smaz+pfor+dedup"),
            (SmazNSDedup, "smaz+ns+dedup"),
        ]
    }

    pub fn str(self) -> &'static str {
        CompressionAlgorithm::lookup()
            .into_iter()
            .find_map(|(elem, s)| (elem == self).then_some(s))
            .unwrap()
    }
}

impl FromStr for CompressionAlgorithm {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        CompressionAlgorithm::lookup()
            .into_iter()
            .find_map(|(elem, name)| (name == s).then_some(elem))
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
