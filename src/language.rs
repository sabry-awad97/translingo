use std::collections::HashMap;

// Define a mapping of supported languages
// The key is the ISO 639-1 code, and the value is the language name
lazy_static::lazy_static! {
    pub static ref LANGS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("auto", "Automatic");
        m.insert("af", "Afrikaans");
        m.insert("sq", "Albanian");
        m.insert("am", "Amharic");
        m.insert("ar", "Arabic");
        m.insert("hy", "Armenian");
        m.insert("az", "Azerbaijani");
        m.insert("eu", "Basque");
        m.insert("be", "Belarusian");
        m.insert("bn", "Bengali");
        m.insert("bs", "Bosnian");
        m.insert("bg", "Bulgarian");
        m.insert("ca", "Catalan");
        m.insert("ceb", "Cebuano");
        m.insert("ny", "Chichewa");
        m.insert("zh-cn", "Chinese Simplified");
        m.insert("zh-tw", "Chinese Traditional");
        m.insert("co", "Corsican");
        m.insert("hr", "Croatian");
        m.insert("cs", "Czech");
        m.insert("da", "Danish");
        m.insert("nl", "Dutch");
        m.insert("en", "English");
        m.insert("eo", "Esperanto");
        m.insert("et", "Estonian");
        m.insert("tl", "Filipino");
        m.insert("fi", "Finnish");
        m.insert("fr", "French");
        m.insert("fy", "Frisian");
        m.insert("gl", "Galician");
        m.insert("ka", "Georgian");
        m.insert("de", "German");
        m.insert("el", "Greek");
        m.insert("gu", "Gujarati");
        m.insert("ht", "Haitian Creole");
        m.insert("ha", "Hausa");
        m.insert("haw", "Hawaiian");
        m.insert("iw", "Hebrew");
        m.insert("hi", "Hindi");
        m.insert("hmn", "Hmong");
        m.insert("hu", "Hungarian");
        m.insert("is", "Icelandic");
        m.insert("ig", "Igbo");
        m.insert("id", "Indonesian");
        m.insert("ga", "Irish");
        m.insert("it", "Italian");
        m.insert("ja", "Japanese");
        m.insert("jw", "Javanese");
        m.insert("kn", "Kannada");
        m.insert("kk", "Kazakh");
        m.insert("km", "Khmer");
        m.insert("ko", "Korean");
        m.insert("ku", "Kurdish (Kurmanji)");
        m.insert("ky", "Kyrgyz");
        m.insert("lo", "Lao");
        m.insert("la", "Latin");
        m.insert("lv", "Latvian");
        m.insert("lt", "Lithuanian");
        m.insert("lb", "Luxembourgish");
        m.insert("mk", "Macedonian");
        m.insert("mg", "Malagasy");
        m.insert("ms", "Malay");
        m.insert("ml", "Malayalam");
        m.insert("mt", "Maltese");
        m.insert("mt", "Maltese");
        m.insert("mi", "Maori");
        m.insert("mr", "Marathi");
        m.insert("mn", "Mongolian");
        m.insert("my", "Myanmar (Burmese)");
        m.insert("ne", "Nepali");
        m.insert("no", "Norwegian");
        m.insert("ps", "Pashto");
        m.insert("fa", "Persian");
        m.insert("pl", "Polish");
        m.insert("pt", "Portuguese");
        m.insert("ma", "Punjabi");
        m.insert("ro", "Romanian");
        m.insert("ru", "Russian");
        m.insert("sm", "Samoan");
        m.insert("gd", "Scots Gaelic");
        m.insert("sr", "Serbian");
        m.insert("st", "Sesotho");
        m.insert("sn", "Shona");
        m.insert("sd", "Sindhi");
        m.insert("si", "Sinhala");
        m.insert("sk", "Slovak");
        m.insert("sl", "Slovenian");
        m.insert("so", "Somali");
        m.insert("es", "Spanish");
        m.insert("su", "Sundanese");
        m.insert("sw", "Swahili");
        m.insert("sv", "Swedish");
        m.insert("tg", "Tajik");
        m.insert("ta", "Tamil");
        m.insert("te", "Telugu");
        m.insert("th", "Thai");
        m.insert("tr", "Turkish");
        m.insert("uk", "Ukrainian");
        m.insert("ur", "Urdu");
        m.insert("uz", "Uzbek");
        m.insert("vi", "Vietnamese");
        m.insert("cy", "Welsh");
        m.insert("xh", "Xhosa");
        m.insert("yi", "Yiddish");
        m.insert("yo", "Yoruba");
        m.insert("zu", "Zulu");
        m
    };
}

/// Returns the ISO 639-1 code of the desiredLang – if it is supported by Google Translate
/// Returns None if the language is not supported
pub fn get_code(desired_lang: &str) -> Option<&'static str> {
    let desired_lang = desired_lang.to_lowercase();

    // Check if the language name is directly in the LANGS HashMap
    if let Some(code) = LANGS.get(&desired_lang[..]) {
        return Some(code);
    }

    // If not, search for the language name in LANGS
    let mut matching_langs = LANGS
        .iter()
        .filter(|(_, lang)| lang.to_lowercase() == desired_lang)
        .map(|(&code, _)| code);

    matching_langs.next()
}

/// Returns true if the desiredLang is supported by Google Translate and false otherwise
pub fn is_supported(desired_lang: &str) -> bool {
    get_code(desired_lang).is_some()
}
