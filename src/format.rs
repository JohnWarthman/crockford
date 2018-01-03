use encoding;

#[derive(Copy, Clone, Debug)]
pub enum Case {
    Upper,
    Lower,
}

impl Case {
    fn is_uppercase(&self) -> bool {
        match *self {
            Case::Upper => true,
            Case::Lower => false,
        }
    }
}

#[derive(Debug)]
/// An encoder with formatting options.
///
/// Values encoded using an instance of `Encoder` will be formatted with respect to the options
/// provided, e.g. capitalization, grouping of digits, and so forth.
///
/// Note: all fields of `Encoder` are public. This is to allow for the use of an instance of
/// `Encoder` as constant or static.
pub struct Encoder {
    pub case: Case,
}

impl Encoder {
    pub fn new() -> Self {
        Self { case: Case::Lower }
    }

    pub fn with_case(case: Case) -> Self {
        Self { case }
    }

    pub fn encode(&self, n: u64) -> Formatter {
        let mut f = Formatter::new(self);
        encoding::encode_into(n, &mut f);
        f
    }
}

impl Default for Encoder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Formatter<'e> {
    encoder: &'e Encoder,
    len: usize,
    data: [u8; 13],
}

impl<'e> Formatter<'e> {
    pub fn new(encoder: &'e Encoder) -> Self {
        Formatter {
            encoder,
            len: 0,
            data: [0; 13],
        }
    }

    pub fn render(&self) -> String {
        let mut s = String::with_capacity(self.len);
        for idx in 0..self.len {
            s.push(self.data[idx] as char);
        }
        s
    }

    pub fn render_into<W: encoding::Write>(&self, w: &mut W) {
        for idx in 0..self.len {
            w.write(self.data[idx]);
        }
    }
}

impl<'e> encoding::Write for Formatter<'e> {
    fn write(&mut self, mut u: u8) {
        // FIXME: I believe this kind of transformation should be performed if and when the
        // formatter is realized rather than at write time. When we're writing, we should only
        // be writing.
        if !self.encoder.case.is_uppercase() {
            u = u.to_ascii_lowercase();
        }

        // I'm not going to do an explicit bounds check here because #encode_into won't attempt to
        // write more than 13 bytes here. If you employ the #Write trait and then do the #left
        // thing with it, that's your problem. Anyway, this isn't memory unsafe because indexed
        // access is implicitly checked, and you'll just get a panic if you try any dumbfuckery.
        self.data[self.len] = u;
        self.len += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowercase_encoder_works() {
        let encoder = Encoder::new();
        let result = encoder.encode(5111);

        let mut s = String::new();
        result.render_into(&mut s);

        assert_eq!("4zq", &*s);
        assert_eq!("4zq", &*result.render());
    }

    #[test]
    fn uppercase_encoder_works() {
        let encoder = Encoder::with_case(Case::Upper);
        let result = encoder.encode(5111);

        let mut s = String::new();
        result.render_into(&mut s);

        assert_eq!("4ZQ", &*s);
        assert_eq!("4ZQ", &*result.render());
    }
}

#[cfg(test)]
mod benchmarks {
    use super::*;
    use test::{self, Bencher};

    #[bench]
    fn encode_5111(b: &mut Bencher) {
        let encoder = Encoder::with_case(Case::Lower);
        b.iter(|| test::black_box(encoder.encode(5111)));
    }

    #[bench]
    fn encode_18446744073709551615(b: &mut Bencher) {
        let encoder = Encoder::with_case(Case::Lower);
        b.iter(|| test::black_box(encoder.encode(18446744073709551615)));
    }

    #[bench]
    fn encode_5111_with_render(b: &mut Bencher) {
        let encoder = Encoder::with_case(Case::Lower);
        b.iter(|| test::black_box(encoder.encode(5111).render()));
    }

    #[bench]
    fn encode_18446744073709551615_with_render(b: &mut Bencher) {
        let encoder = Encoder::with_case(Case::Lower);
        b.iter(|| test::black_box(encoder.encode(18446744073709551615).render()));
    }

    #[bench]
    fn encode_5111_with_render_into(b: &mut Bencher) {
        let encoder = Encoder::with_case(Case::Lower);
        let mut s = String::with_capacity(13);

        b.iter(|| {
            s.clear();
            test::black_box(encoder.encode(5111).render_into(&mut s));
        });
    }

    #[bench]
    fn encode_18446744073709551615_with_render_into(b: &mut Bencher) {
        let encoder = Encoder::with_case(Case::Lower);
        let mut s = String::with_capacity(13);

        b.iter(|| {
            s.clear();
            test::black_box(encoder.encode(18446744073709551615).render_into(&mut s));
        });
    }
}
