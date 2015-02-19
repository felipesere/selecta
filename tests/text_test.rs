#![feature(plugin)]
#![plugin(stainless)]
#![feature(core)]

extern crate icepick;

mod test {
    pub use icepick::text::Text;

    describe! text_test {
        it "normal_is_a_simple_string" {
            let normal = Text::Normal("test".to_string());
            assert_eq!("test", normal.printable().as_slice());
        }
    }
}
