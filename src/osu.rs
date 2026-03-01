use std::fs;
use std::path::PathBuf;
use std::io::Write;

fn get_field<'a>(lines: &'a Vec<&'a str>, field: &str) -> &'a str {
    let i = match lines.iter().position(|l| l.starts_with(field)) {
        Some(index) => index,
        None => panic!("no {} in file", field)
    };
    lines[i].split(':').nth(1).expect("missing field").trim()
}

fn convert_osu_x(osu_x: f32, lanes: f32) -> f32 {
    (osu_x * lanes / 512.0).floor()
    / (lanes - 1.0)
}

pub fn osu(input: PathBuf, output: PathBuf) {
    let osu = fs::read_to_string(input).unwrap();
    let mut txt = fs::File::create(output).unwrap();

    let lines: Vec<&str> = osu.lines().collect();

    let lanes: f32 = get_field(&lines, "CircleSize").parse().expect("invalid CircleSize");

    let title = get_field(&lines, "TitleUnicode");
    let artist = get_field(&lines, "ArtistUnicode");
    let id = format!("osu_{}", get_field(&lines, "BeatmapID"));

    write!(
        &mut txt, "// title: {}\n// artist: {}\n// id: {}\n// bpm: ms\n",
        title, artist, id
    ).unwrap();

    let mut i = match lines.iter().position(|l| l == &"[HitObjects]") {
        Some(index) => index + 1,
        None => panic!("no HitObjects section in file")
    };

    while i < lines.len() {
        let line = lines[i].trim();
        let mut parts = line.split(',');
        
        let osu_x: f32 = parts.next().expect("no note x").parse().expect("invalid note x");
        let _osu_y = parts.next().expect("no note y");
        let ms: u32 = parts.next().expect("no note time").parse().expect("invalid note time");
        let type_flags: u16 = parts.next().expect("no type flags").parse().expect("invalid type flags");
        let _ = parts.next().expect("not enough note information");
        let ms_end: u32 = parts.next().expect("no note properties").split(':').nth(0).expect("no hold note end time").parse().expect("invalid hold note end time");

        let x = convert_osu_x(osu_x, lanes);
        assert!(0.0 <= x && x <= 1.0, "note x out of range");

        match type_flags {
            1 | 5 => write!(&mut txt, "t {} {}\n", ms, x),
            128 | 132 => write!(&mut txt, "h {} {} {}\n", ms, x, ms_end),
            _ => panic!("/!\\ line {}: unsupported note type with flags {}", i, type_flags)
        }.unwrap();

        i += 1;
    }
}
