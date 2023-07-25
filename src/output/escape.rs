use ansi_term::{ANSIString, Style};

use super::file_name::QuoteStyle;

pub fn push_bit(bits: &mut Vec<ANSIString<'_>>, str: String, good: &Style, bad: &Style, is_good: bool) {
    bits.push(
        if is_good {
            good.paint(str)
        } else {
            bad.paint(str.escape_default().to_string())
        }
    );
}

pub fn escape(string: String, bits: &mut Vec<ANSIString<'_>>, good: Style, bad: Style, quote_style: QuoteStyle) {
    let needs_quotes = string.contains(' ') || string.contains('\'');
    let quote_bit = good.paint(if string.contains('\'') { "\"" } else { "\'" });

    let mut string_clone = string.clone();
    let mut last_char_good = true;

    let mut index = 0;
    for c in string.clone().chars() {
        let this_char_good = c >= ' ' && c != '\x7f';
        
        if this_char_good != last_char_good {
            if index > 0 {
                let temp_str = string_clone.split_off(index);
                push_bit(bits, string_clone, &good, &bad, last_char_good);
                string_clone = temp_str;
                index = 0;
            }
            last_char_good = this_char_good;
        }

        index += 1;
    }
    push_bit(bits, string_clone, &good, &bad, last_char_good);

    if quote_style != QuoteStyle::NoQuotes && needs_quotes {
        bits.insert(0, quote_bit.clone());
        bits.push(quote_bit);
    }
}
