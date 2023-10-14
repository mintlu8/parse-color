#[allow(non_upper_case_globals)]
mod colors {
    pub static slate: [[u8; 4]; 11] = [
        0xf8fafcffu32.to_be_bytes(),
        0xf1f5f9ffu32.to_be_bytes(),
        0xe2e8f0ffu32.to_be_bytes(),
        0xcbd5e1ffu32.to_be_bytes(),
        0x94a3b8ffu32.to_be_bytes(),
        0x64748bffu32.to_be_bytes(),
        0x475569ffu32.to_be_bytes(),
        0x334155ffu32.to_be_bytes(),
        0x1e293bffu32.to_be_bytes(),
        0x0f172affu32.to_be_bytes(),
        0x020617ffu32.to_be_bytes(),
    ];
    pub static grey: [[u8; 4]; 11] = [
        0xf9fafbffu32.to_be_bytes(),
        0xf3f4f6ffu32.to_be_bytes(),
        0xe5e7ebffu32.to_be_bytes(),
        0xd1d5dbffu32.to_be_bytes(),
        0x9ca3afffu32.to_be_bytes(),
        0x6b7280ffu32.to_be_bytes(),
        0x4b5563ffu32.to_be_bytes(),
        0x374151ffu32.to_be_bytes(),
        0x1f2937ffu32.to_be_bytes(),
        0x111827ffu32.to_be_bytes(),
        0x030712ffu32.to_be_bytes(),
    ];
    pub static zinc: [[u8; 4]; 11] = [
        0xfafafaffu32.to_be_bytes(),
        0xf4f4f5ffu32.to_be_bytes(),
        0xe4e4e7ffu32.to_be_bytes(),
        0xd4d4d8ffu32.to_be_bytes(),
        0xa1a1aaffu32.to_be_bytes(),
        0x71717affu32.to_be_bytes(),
        0x52525bffu32.to_be_bytes(),
        0x3f3f46ffu32.to_be_bytes(),
        0x27272affu32.to_be_bytes(),
        0x18181bffu32.to_be_bytes(),
        0x09090bffu32.to_be_bytes(),
    ];
    pub static neutral: [[u8; 4]; 11] = [
        0xfafafaffu32.to_be_bytes(),
        0xf5f5f5ffu32.to_be_bytes(),
        0xe5e5e5ffu32.to_be_bytes(),
        0xd4d4d4ffu32.to_be_bytes(),
        0xa3a3a3ffu32.to_be_bytes(),
        0x737373ffu32.to_be_bytes(),
        0x525252ffu32.to_be_bytes(),
        0x404040ffu32.to_be_bytes(),
        0x262626ffu32.to_be_bytes(),
        0x171717ffu32.to_be_bytes(),
        0x0a0a0affu32.to_be_bytes(),
    ];
    pub static stone: [[u8; 4]; 11] = [
        0xfafaf9ffu32.to_be_bytes(),
        0xf5f5f4ffu32.to_be_bytes(),
        0xe7e5e4ffu32.to_be_bytes(),
        0xd6d3d1ffu32.to_be_bytes(),
        0xa8a29effu32.to_be_bytes(),
        0x78716cffu32.to_be_bytes(),
        0x57534effu32.to_be_bytes(),
        0x44403cffu32.to_be_bytes(),
        0x292524ffu32.to_be_bytes(),
        0x1c1917ffu32.to_be_bytes(),
        0x0c0a09ffu32.to_be_bytes(),
    ];
    pub static red: [[u8; 4]; 11] = [
        0xfef2f2ffu32.to_be_bytes(),
        0xfee2e2ffu32.to_be_bytes(),
        0xfecacaffu32.to_be_bytes(),
        0xfca5a5ffu32.to_be_bytes(),
        0xf87171ffu32.to_be_bytes(),
        0xef4444ffu32.to_be_bytes(),
        0xdc2626ffu32.to_be_bytes(),
        0xb91c1cffu32.to_be_bytes(),
        0x991b1bffu32.to_be_bytes(),
        0x7f1d1dffu32.to_be_bytes(),
        0x450a0affu32.to_be_bytes(),
    ];
    pub static orange: [[u8; 4]; 11] = [
        0xfff7edffu32.to_be_bytes(),
        0xffedd5ffu32.to_be_bytes(),
        0xfed7aaffu32.to_be_bytes(),
        0xfdba74ffu32.to_be_bytes(),
        0xfb923cffu32.to_be_bytes(),
        0xf97316ffu32.to_be_bytes(),
        0xea580cffu32.to_be_bytes(),
        0xc2410cffu32.to_be_bytes(),
        0x9a3412ffu32.to_be_bytes(),
        0x7c2d12ffu32.to_be_bytes(),
        0x431407ffu32.to_be_bytes(),
    ];
    pub static amber: [[u8; 4]; 11] = [
        0xfffbebffu32.to_be_bytes(),
        0xfef3c7ffu32.to_be_bytes(),
        0xfde68affu32.to_be_bytes(),
        0xfcd34dffu32.to_be_bytes(),
        0xfbbf24ffu32.to_be_bytes(),
        0xf59e0bffu32.to_be_bytes(),
        0xd97706ffu32.to_be_bytes(),
        0xb45309ffu32.to_be_bytes(),
        0x92400effu32.to_be_bytes(),
        0x78350fffu32.to_be_bytes(),
        0x451a03ffu32.to_be_bytes(),
    ];
    pub static yellow: [[u8; 4]; 11] = [
        0xfefce8ffu32.to_be_bytes(),
        0xfef9c3ffu32.to_be_bytes(),
        0xfef08affu32.to_be_bytes(),
        0xfde047ffu32.to_be_bytes(),
        0xfacc15ffu32.to_be_bytes(),
        0xeab308ffu32.to_be_bytes(),
        0xca8a04ffu32.to_be_bytes(),
        0xa16207ffu32.to_be_bytes(),
        0x854d0effu32.to_be_bytes(),
        0x713f12ffu32.to_be_bytes(),
        0x422006ffu32.to_be_bytes(),
    ];
    pub static lime: [[u8; 4]; 11] = [
        0xf7fee7ffu32.to_be_bytes(),
        0xecfccbffu32.to_be_bytes(),
        0xd9f99dffu32.to_be_bytes(),
        0xbef264ffu32.to_be_bytes(),
        0xa3e635ffu32.to_be_bytes(),
        0x84cc16ffu32.to_be_bytes(),
        0x65a30dffu32.to_be_bytes(),
        0x4d7c0fffu32.to_be_bytes(),
        0x3f6212ffu32.to_be_bytes(),
        0x365314ffu32.to_be_bytes(),
        0x1a2e05ffu32.to_be_bytes(),
    ];
    pub static green: [[u8; 4]; 11] = [
        0xf0fdf4ffu32.to_be_bytes(),
        0xdcfce7ffu32.to_be_bytes(),
        0xbbf7d0ffu32.to_be_bytes(),
        0x86efacffu32.to_be_bytes(),
        0x4ade80ffu32.to_be_bytes(),
        0x22c55effu32.to_be_bytes(),
        0x16a34affu32.to_be_bytes(),
        0x15803dffu32.to_be_bytes(),
        0x166534ffu32.to_be_bytes(),
        0x14532dffu32.to_be_bytes(),
        0x052e16ffu32.to_be_bytes(),
    ];
    pub static emerald: [[u8; 4]; 11] = [
        0xecfdf5ffu32.to_be_bytes(),
        0xd1fae5ffu32.to_be_bytes(),
        0xa7f3d0ffu32.to_be_bytes(),
        0x6ee7b7ffu32.to_be_bytes(),
        0x34d399ffu32.to_be_bytes(),
        0x10b981ffu32.to_be_bytes(),
        0x059669ffu32.to_be_bytes(),
        0x047857ffu32.to_be_bytes(),
        0x065f46ffu32.to_be_bytes(),
        0x064e3bffu32.to_be_bytes(),
        0x022c22ffu32.to_be_bytes(),
    ];
    pub static teal: [[u8; 4]; 11] = [
        0xf0fdfaffu32.to_be_bytes(),
        0xccfbf1ffu32.to_be_bytes(),
        0x99f6e4ffu32.to_be_bytes(),
        0x5eead4ffu32.to_be_bytes(),
        0x2dd4bfffu32.to_be_bytes(),
        0x14b8a6ffu32.to_be_bytes(),
        0x0d9488ffu32.to_be_bytes(),
        0x0f766effu32.to_be_bytes(),
        0x115e59ffu32.to_be_bytes(),
        0x134e4affu32.to_be_bytes(),
        0x042f2effu32.to_be_bytes(),
    ];
    pub static cyan: [[u8; 4]; 11] = [
        0xecfeffffu32.to_be_bytes(),
        0xcffafeffu32.to_be_bytes(),
        0xa5f3fcffu32.to_be_bytes(),
        0x67e8f9ffu32.to_be_bytes(),
        0x22d3eeffu32.to_be_bytes(),
        0x06b6d4ffu32.to_be_bytes(),
        0x0891b2ffu32.to_be_bytes(),
        0x0e7490ffu32.to_be_bytes(),
        0x155e75ffu32.to_be_bytes(),
        0x164e63ffu32.to_be_bytes(),
        0x083344ffu32.to_be_bytes(),
    ];
    pub static sky: [[u8; 4]; 11] = [
        0xf0f9ffffu32.to_be_bytes(),
        0xe0f2feffu32.to_be_bytes(),
        0xbae6fdffu32.to_be_bytes(),
        0x7dd3fcffu32.to_be_bytes(),
        0x38bdf8ffu32.to_be_bytes(),
        0x0ea5e9ffu32.to_be_bytes(),
        0x0284c7ffu32.to_be_bytes(),
        0x0369a1ffu32.to_be_bytes(),
        0x075985ffu32.to_be_bytes(),
        0x0c4a6effu32.to_be_bytes(),
        0x082f49ffu32.to_be_bytes(),
    ];
    pub static blue: [[u8; 4]; 11] = [
        0xeff6ffffu32.to_be_bytes(),
        0xdbeafeffu32.to_be_bytes(),
        0xbfdbfeffu32.to_be_bytes(),
        0x93c5fdffu32.to_be_bytes(),
        0x60a5faffu32.to_be_bytes(),
        0x3b82f6ffu32.to_be_bytes(),
        0x2563ebffu32.to_be_bytes(),
        0x1d4ed8ffu32.to_be_bytes(),
        0x1e40afffu32.to_be_bytes(),
        0x1e3a8affu32.to_be_bytes(),
        0x172554ffu32.to_be_bytes(),
    ];
    pub static indigo: [[u8; 4]; 11] = [
        0xeef2ffffu32.to_be_bytes(),
        0xe0e7ffffu32.to_be_bytes(),
        0xc7d2feffu32.to_be_bytes(),
        0xa5b4fcffu32.to_be_bytes(),
        0x818cf8ffu32.to_be_bytes(),
        0x6366f1ffu32.to_be_bytes(),
        0x4f46e5ffu32.to_be_bytes(),
        0x4338caffu32.to_be_bytes(),
        0x3730a3ffu32.to_be_bytes(),
        0x312e81ffu32.to_be_bytes(),
        0x1e1b4bffu32.to_be_bytes(),
    ];
    pub static violet: [[u8; 4]; 11] = [
        0xf5f3ffffu32.to_be_bytes(),
        0xede9feffu32.to_be_bytes(),
        0xddd6feffu32.to_be_bytes(),
        0xc4b5fdffu32.to_be_bytes(),
        0xa78bfaffu32.to_be_bytes(),
        0x8b5cf6ffu32.to_be_bytes(),
        0x7c3aedffu32.to_be_bytes(),
        0x6d28d9ffu32.to_be_bytes(),
        0x5b21b6ffu32.to_be_bytes(),
        0x4c1d95ffu32.to_be_bytes(),
        0x2e1065ffu32.to_be_bytes(),
    ];
    pub static purple: [[u8; 4]; 11] = [
        0xfaf5ffffu32.to_be_bytes(),
        0xf3e8ffffu32.to_be_bytes(),
        0xe9d5ffffu32.to_be_bytes(),
        0xd8b4feffu32.to_be_bytes(),
        0xc084fcffu32.to_be_bytes(),
        0xa855f7ffu32.to_be_bytes(),
        0x9333eaffu32.to_be_bytes(),
        0x7e22ceffu32.to_be_bytes(),
        0x6b21a8ffu32.to_be_bytes(),
        0x581c87ffu32.to_be_bytes(),
        0x3b0764ffu32.to_be_bytes(),
    ];
    pub static fuchsia: [[u8; 4]; 11] = [
        0xfdf4ffffu32.to_be_bytes(),
        0xfae8ffffu32.to_be_bytes(),
        0xf5d0feffu32.to_be_bytes(),
        0xf0abfcffu32.to_be_bytes(),
        0xe879f9ffu32.to_be_bytes(),
        0xd946efffu32.to_be_bytes(),
        0xc026d3ffu32.to_be_bytes(),
        0xa21cafffu32.to_be_bytes(),
        0x86198fffu32.to_be_bytes(),
        0x701a75ffu32.to_be_bytes(),
        0x4a044effu32.to_be_bytes(),
    ];
    pub static pink: [[u8; 4]; 11] = [
        0xfdf2f8ffu32.to_be_bytes(),
        0xfce7f3ffu32.to_be_bytes(),
        0xfbcfe8ffu32.to_be_bytes(),
        0xf9a8d4ffu32.to_be_bytes(),
        0xf472b6ffu32.to_be_bytes(),
        0xec4899ffu32.to_be_bytes(),
        0xdb2777ffu32.to_be_bytes(),
        0xbe185dffu32.to_be_bytes(),
        0x9d174dffu32.to_be_bytes(),
        0x831843ffu32.to_be_bytes(),
        0x500724ffu32.to_be_bytes(),
    ];
    pub static rose: [[u8; 4]; 11] = [
        0xfff1f2ffu32.to_be_bytes(),
        0xffe4e6ffu32.to_be_bytes(),
        0xfecdd3ffu32.to_be_bytes(),
        0xfda4afffu32.to_be_bytes(),
        0xfb7185ffu32.to_be_bytes(),
        0xf43f5effu32.to_be_bytes(),
        0xe11d48ffu32.to_be_bytes(),
        0xbe123cffu32.to_be_bytes(),
        0x9f1239ffu32.to_be_bytes(),
        0x881337ffu32.to_be_bytes(),
        0x4c0519ffu32.to_be_bytes(),
    ];
}

/// Parse a TailwindCSS color name into a `[u8;4]` in the order of `RGBA`.
pub fn parse_tailwind(name: &str, index: usize) -> Option<[u8; 4]>{
    let s: String = name.chars().filter_map(|c|
        match c {
            'A'..='Z' => Some(c.to_ascii_lowercase()),
            'a'..='z' => Some(c),
            _ => None,
        }
    ).collect();
    parse_tailwind_flat_lower(&s, index)
}

/// Parse a TailwindCSS color name into a `[u8;4]` in the order of `RGBA`.
///
/// This is a non-allocating version of [`parse_tailwind`], 
/// will fail if the string is not `flatlowercase`.
pub fn parse_tailwind_flat_lower(name: &str, index: usize) -> Option<[u8; 4]>{
    match name {
        "none" => return Some([0, 0, 0, 0]),
        "transparent" => return Some([0, 0, 0, 0]),
        "black" => return Some([0, 0, 0, 255]),
        "white" => return Some([255, 255, 255, 255]),
        _ => (),
    };
    let index = match index {
        50 => 0,
        100 => 1,
        200 => 2,
        300 => 3,
        400 => 4,
        500 => 5,
        600 => 6,
        700 => 7,
        800 => 8,
        900 => 9,
        950 => 10,
        _ => return None
    };
    let item = match name {
        "slate" => colors::slate[index],
        "grey" => colors::grey[index],
        "zinc" => colors::zinc[index],
        "neutral" => colors::neutral[index],
        "stone" => colors::stone[index],
        "red" => colors::red[index],
        "orange" => colors::orange[index],
        "amber" => colors::amber[index],
        "yellow" => colors::yellow[index],
        "lime" => colors::lime[index],
        "green" => colors::green[index],
        "emerald" => colors::emerald[index],
        "teal" => colors::teal[index],
        "cyan" => colors::cyan[index],
        "sky" => colors::sky[index],
        "blue" => colors::blue[index],
        "indigo" => colors::indigo[index],
        "purple" => colors::purple[index],
        "fuchsia" => colors::fuchsia[index],
        "pink" => colors::pink[index],
        "rose" => colors::rose[index],
        _ => return None,
    };
    return Some(item);
}
