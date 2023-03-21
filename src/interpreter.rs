use crate::{parser::Expression, Language};


pub fn has_phonemes(language: &Language, expression: &Expression) -> bool{
    match expression{
        Expression::Literal(s) => has_phoneme(language, s),
        Expression::Grouping(e) => has_phonemes(language, e),
        Expression::Not(e) => !has_phonemes(language, e),
        Expression::Binary(l, o, r) => match o{
            crate::parser::BinaryOperator::AND => has_phonemes(language, l) && has_phonemes(language, r),
            crate::parser::BinaryOperator::OR => has_phonemes(language, l) || has_phonemes(language, r),
        },
    }    
}

pub fn has_phoneme(language: &Language, phoneme: &str) -> bool{
    for (p, _allophones) in &language.phonemes{
        if p == phoneme{
            return true;
        }
    }
    return false;
}