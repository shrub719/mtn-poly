# polyrhythm

A multipurpose command line conversion tool for metronome.

## Installation

```cargo install mtn-poly```

## Usage

- `mtn-poly compile {input}`: compile editable `.mtn` file to binary `.mtb` file
    - `-o {output}`: pass in location of output
    - `-s {offset}`: start time in ms. Used for testing
- `mtn-poly osu {input}`: convert a `.osu` file (from inside a compressed `.osz` archive) to a `.mtn` map file
    - `-o {output}`: pass in location of output
    - `-s {offset}`: start time in ms. Used for testing

