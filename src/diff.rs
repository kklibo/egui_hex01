//duplicated from yew_hex02/arb_comp04
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HexCell {
    Same { value: u8, source_id: usize },
    Diff { value: u8, source_id: usize },
    Blank,
}

pub fn get_diffs(
    a: &[u8],
    b: &[u8],
    range: std::ops::Range<usize>,
) -> (Vec<HexCell>, Vec<HexCell>) {
    let mut a = a.iter();
    let mut b = b.iter();
    let mut a_diff = Vec::new();
    let mut b_diff = Vec::new();

    let mut i = 0;
    loop {
        if i >= range.end {
            break;
        }

        let a_next = a.next();
        let b_next = b.next();

        if i < range.start {
            i += 1;
            continue;
        }

        i += 1;

        match (a_next, b_next) {
            (Some(a), Some(b)) => {
                if a == b {
                    a_diff.push(HexCell::Same {
                        value: *a,
                        source_id: 0,
                    });
                    b_diff.push(HexCell::Same {
                        value: *b,
                        source_id: 0,
                    });
                } else {
                    a_diff.push(HexCell::Diff {
                        value: *a,
                        source_id: 0,
                    });
                    b_diff.push(HexCell::Diff {
                        value: *b,
                        source_id: 0,
                    });
                }
            }
            (Some(_), None) => {
                a_diff.push(HexCell::Blank);
            }
            (None, Some(_)) => {
                b_diff.push(HexCell::Blank);
            }
            (None, None) => {
                a_diff.push(HexCell::Blank);
                b_diff.push(HexCell::Blank);
            }
        }
    }
    (a_diff, b_diff)
}
