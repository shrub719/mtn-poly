use std::fs;
use std::path::PathBuf;
use std::io::Write;
use anyhow::{ Context, Result, anyhow };

fn get_field<'a>(lines: &'a Vec<&'a str>, field: &str) -> Result<&'a str> {
    let i = match lines.iter().position(|l| l.starts_with(field)) {
        Some(index) => index,
        None => return Err(anyhow!("no {} field in file", field))
    };
    Ok(
        lines[i].split(':').nth(1)
            .with_context(|| format!("line {}: {} field missing value", i, field))?
            .trim()
    )
}

fn convert_osu_x(osu_x: f32, lanes: f32) -> f32 {
    (osu_x * lanes / 512.0).floor()
    / (lanes - 1.0)
}

pub fn osu(input: PathBuf, output: PathBuf, start_offset: u32) -> Result<()> {
    let osu = fs::read_to_string(input).unwrap();
    let mut txt = fs::File::create(output).unwrap();

    let lines: Vec<&str> = osu.lines().collect();

    let lanes: f32 = get_field(&lines, "CircleSize")?.parse().with_context(|| "invalid CircleSize")?;

    let title = get_field(&lines, "TitleUnicode")?;
    let artist = get_field(&lines, "ArtistUnicode")?;
    let id = format!("osu_{}", get_field(&lines, "BeatmapID")?);

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
        
        let osu_x: f32 = parts.next()
            .with_context(|| format!("line {}: missing x coordinate", i))?.parse()
            .with_context(|| format!("line {}: invalid x coordinate", i))?;

        let _osu_y = parts.next().with_context(|| format!("line {}: missing y coordinate", i))?;

        let mut ms: u32 = parts.next()
            .with_context(|| format!("line {}: missing note time", i))?.parse()
            .with_context(|| format!("line {}: invalid note time", i))?;

        if ms < start_offset {
            i += 1;
            continue;
        } else {
            ms -= start_offset;
        }

        let type_flags: u16 = parts.next()
            .with_context(|| format!("line {}: missing note type", i))?.parse()
            .with_context(|| format!("line {}: invalid note type", i))?;

        let _ = parts.next().with_context(|| format!("line {}: missing note hitsound", i))?;

        let ms_end: u32 = parts.next()
            .with_context(|| format!("line {}: missing note properties", i))?.split(':').nth(0)
            .with_context(|| format!("line {}: missing hold note end time", i))?.parse()
            .with_context(|| format!("line {}: invalid hold note end time", i))?;

        let x = convert_osu_x(osu_x, lanes);
        if x < 0.0 || x > 1.0 { return Err(anyhow!("line {}: note position calculation error", i)) }

        match type_flags {
            1 | 5 => write!(&mut txt, "t {} {}\n", ms, x),
            128 | 132 => write!(&mut txt, "h {} {} {}\n", ms, x, ms_end),
            _ => panic!("/!\\ line {}: unsupported note type with flags {}", i, type_flags)
        }.unwrap();

        i += 1;
    }

    Ok(())
}
