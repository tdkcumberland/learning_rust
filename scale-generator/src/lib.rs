#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Error;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Scale<'a> {
    scale: Vec<&'a str>
}

static SHARP:&[&str; 12] = &["A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#"];
static FLAT:&[&str; 12] = &["A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab"];

impl Scale<'_> {
    
    pub fn new<'a>(tonic: &'a str, intervals: &'a str) -> Result<Scale<'a>, Error> {
        let mut out:Vec<&str> = Vec::new();
        let mut initial_index = Scale::get_tonic_index(tonic, Scale::chrome_selection(tonic));
        out.push(Scale::chrome_selection(tonic)[initial_index]);
        for step in intervals.chars() {
            match step {
                'M' => initial_index += 2,
                'm' => initial_index += 1,
                'A' => initial_index += 3,
                _ => panic!("interval not recognized!")
            };
            out.push(Scale::chrome_selection(tonic)[initial_index%12])
        }
        Ok(Scale {scale: out.to_vec()})
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        Ok(Scale {scale: Scale::generate_chromatic(&tonic, Scale::chrome_selection(tonic))})
    }

    fn generate_chromatic<'a>(tonic: &'a str, sf:&'a[&str; 12]) -> Vec<&'a str> {
        let tonic_index = Scale::get_tonic_index(tonic, sf);
        let mut out:Vec<&str> = [&sf[tonic_index..sf.len()], &sf[0..tonic_index]].concat();
        out.push(tonic);
        out
    }

    fn get_tonic_index(tonic: &str, sf:&[&str; 12]) -> usize {
        sf.iter().position(|element| element.to_uppercase() == tonic.to_uppercase()).unwrap()
    }

    fn chrome_selection(tonic: &str) -> &[&str; 12]{
        match tonic {
            "C" | "a" | "G" | "D" | "A" | "E" | "B" | "F#" | "e" | "b" | "f#" | "c#" | "g#" | "d#" => &SHARP,
            "F" | "Bb" | "Eb" | "Ab" | "Db" | "Gb" | "d" | "g" | "c" | "f" | "bb" | "eb" => &FLAT,
            _ => panic!("not recognizable tonic")
        }
    }

    pub fn enumerate(&self) -> Vec<&str> {
        self.scale.to_vec()
    }
}
