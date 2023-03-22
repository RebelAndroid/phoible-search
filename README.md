# phoible-search
searching tool for PHOIBLE
Usage: phoible-search <INPUT_EXPRESSION>

Arguments:
  <INPUT_EXPRESSION>

Options:
  -h, --help  Print help
  
  An input expression consists of phonemes '|' representing or, '&' representing and, '!' representing not and '(' and ')' for grouping. For example, "p&b" searches for languages with the voiced and voiceless bilabial stops, and "θ|ð" seraches for languages with the voiced and/or voiceless dental fricatives.
  
  Right now there is no consideration of allophony, but this may change in the future.
