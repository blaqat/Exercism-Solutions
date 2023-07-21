use unicode_segmentation::UnicodeSegmentation as uni_seg;

pub fn reverse(input: &str) -> String {
    // unimplemented!("Write a function to reverse {input}");
    uni_seg::graphemes(input, true).rev().collect()
}
