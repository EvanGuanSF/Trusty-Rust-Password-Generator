// Const item to hold desired character ranges.
pub const CHAR_RANGES: [(&str, &[(u32, u32)]); 2] = [
        // Known character ranges for latin keyboard ascii codes (US/UK English).
        ("special", &[(33, 47), (58, 64), (91, 96), (123, 126)]),
        ("alpha-numeric", &[(48, 57), (65, 90), (97, 122)])
    ];