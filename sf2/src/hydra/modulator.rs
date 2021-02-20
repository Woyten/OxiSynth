use crate::Reader;
use riff::Chunk;

#[derive(Debug)]
pub struct SFModulator {
    src: u16,
    dest: u16,
    amount: i16,
    amt_src: u16,
    transform: u16,
}

impl SFModulator {
    pub fn read(reader: &mut Reader) -> Self {
        let src: u16 = reader.read_u16();
        let dest: u16 = reader.read_u16();
        let amount: i16 = reader.read_i16();
        let amt_src: u16 = reader.read_u16();
        let transform: u16 = reader.read_u16();

        Self {
            src,
            dest,
            amount,
            amt_src,
            transform,
        }
    }

    pub fn read_all(pmod: &Chunk, file: &mut std::fs::File) -> Vec<Self> {
        assert_eq!(pmod.id().as_str(), "pmod");

        let size = pmod.len();
        if size % 10 != 0 || size == 0 {
            panic!("Preset modulator chunk size mismatch");
        }

        let amount = size / 10;

        let data = pmod.read_contents(file).unwrap();
        let mut reader = Reader::new(data);

        (0..amount).map(|_| Self::read(&mut reader)).collect()
    }
}
