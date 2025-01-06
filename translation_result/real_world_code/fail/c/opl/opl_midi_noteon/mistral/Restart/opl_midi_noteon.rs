

const OCTAVETABLE: [u8; 21] = [
b'C', b'D', b'E', b'F', b'G', b'A', b'B', // removed 'C#', 'D#', 'F#', 'G#', 'A#'
b'C', b'D', b'E', b'F', b'G', b'A', b'B', // repeated for the second octave
b'C', b'D', b'E', b'F', b'G', b'A', b'B',
];

fn transpose(octave: i32, note: u8) -> u8 {
let table_index = (note as i32 - b'A' as i32 + octave * 12) % 21 as i32;
OCTAVETABLE[(table_index as usize + 21) % 21]
}

