struct 음절 {
    초성: 초성,         // Initial consonant
    중성: 중성,         // Medial vowel
    종성: Option<종성>, // Final consonant (optional)
}

enum 자모 {
    초성(초성), // Initial consonant
    중성(중성), // Medial vowel
    종성(종성), // Final consonant
}

enum 초성 {
    단자음(char), // Single consonant
    복자음(char, char), // Compound consonant
}

enum 중성 {
    단모음(char), // Single vowel
    복모음(char, char), // Compound vowel
}

enum 종성 {
    단받침(char), // Single final consonant
    겹받침(char, char), // Compound final consonant
}