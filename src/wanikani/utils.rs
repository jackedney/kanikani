pub mod utils {
    use anyhow::{anyhow, Result};
    use image::{DynamicImage, ImageBuffer, Luma};
    use resvg::tiny_skia::Pixmap;
    use resvg::tiny_skia::Transform;
    use resvg::usvg::{Options, Tree};

    /// Normalizes strings for comparison by trimming whitespace and converting to lowercase
    pub fn normalize_string(input: &str) -> String {
        input.trim().to_lowercase()
    }

    /// Converts SVG data to a DynamicImage
    /// Returns a black and white image where SVG content is rendered in white on a black background
    ///
    /// # Arguments
    /// * `svg_data` - A string containing the SVG XML data
    ///
    /// # Returns
    /// * `Result<DynamicImage>` - The converted image or an error
    ///
    /// # Errors
    /// Will return an error if:
    /// - SVG parsing fails
    /// - Image creation fails
    /// - Pixel manipulation fails
    pub fn svg_to_dynamic_image(svg_data: &str) -> Result<DynamicImage> {
        // Parse the SVG data
        let svg_tree = Tree::from_str(&svg_data, &Options::default())
            .map_err(|e| anyhow!("Failed to parse SVG: {}", e))?;

        // Get the SVG dimensions
        let svg_size = svg_tree.size();
        let width = svg_size.width() as u32;
        let height = svg_size.height() as u32;

        if width == 0 || height == 0 {
            return Err(anyhow!("Invalid SVG dimensions: {}x{}", width, height));
        }

        // Create a pixmap and render the SVG to it
        let mut pixmap = Pixmap::new(width, height)
            .ok_or_else(|| anyhow!("Failed to create pixmap of size {}x{}", width, height))?;
        resvg::render(&svg_tree, Transform::default(), &mut pixmap.as_mut());

        // Convert the pixmap to a black and white image buffer
        let mut image_buffer = ImageBuffer::new(width, height);
        for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
            let alpha = pixmap
                .pixel(x, y)
                .ok_or_else(|| anyhow!("Failed to get pixel at ({}, {})", x, y))?
                .alpha();

            // Convert to black and white based on alpha value
            // Using 127 as threshold (half of 255)
            let value = if alpha > 127 { 255 } else { 0 };
            *pixel = Luma([value]);
        }

        // Create and return the dynamic image
        Ok(DynamicImage::ImageLuma8(image_buffer))
    }

    /// Processes and validates user input for Japanese readings
    /// Converts both the user input and accepted readings to hiragana before comparison
    pub fn validate_reading(user_input: &str, accepted_readings: &[String]) -> bool {
        let normalized_user = normalize_string(&user_input);

        // Try direct hiragana match
        let hiragana_match = accepted_readings
            .iter()
            .any(|reading| normalize_string(reading) == normalized_user);

        if hiragana_match {
            return true;
        }

        // If no direct match, try converting romaji to hiragana
        let hiragana_from_romaji = romaji_to_hiragana(&normalized_user);
        accepted_readings
            .iter()
            .map(|reading| reading) // Convert accepted readings to hiragana
            .any(|reading| normalize_string(&reading) == hiragana_from_romaji)
    }

    const ROMAJI_TO_HIRAGANA: [(&str, &str); 100] = [
        // Youon (contracted sounds)
        ("kya", "きゃ"),
        ("kyu", "きゅ"),
        ("kyo", "きょ"),
        ("sha", "しゃ"),
        ("shu", "しゅ"),
        ("sho", "しょ"),
        ("cha", "ちゃ"),
        ("chu", "ちゅ"),
        ("cho", "ちょ"),
        ("nya", "にゃ"),
        ("nyu", "にゅ"),
        ("nyo", "にょ"),
        ("hya", "ひゃ"),
        ("hyu", "ひゅ"),
        ("hyo", "ひょ"),
        ("mya", "みゃ"),
        ("myu", "みゅ"),
        ("myo", "みょ"),
        ("rya", "りゃ"),
        ("ryu", "りゅ"),
        ("ryo", "りょ"),
        ("gya", "ぎゃ"),
        ("gyu", "ぎゅ"),
        ("gyo", "ぎょ"),
        ("ja", "じゃ"),
        ("ju", "じゅ"),
        ("jo", "じょ"),
        ("bya", "びゃ"),
        ("byu", "びゅ"),
        ("byo", "びょ"),
        ("pya", "ぴゃ"),
        ("pyu", "ぴゅ"),
        ("pyo", "ぴょ"),
        // Basic sounds with dakuten and handakuten
        ("ka", "か"),
        ("ki", "き"),
        ("ku", "く"),
        ("ke", "け"),
        ("ko", "こ"),
        ("ga", "が"),
        ("gi", "ぎ"),
        ("gu", "ぐ"),
        ("ge", "げ"),
        ("go", "ご"),
        ("sa", "さ"),
        ("shi", "し"),
        ("su", "す"),
        ("se", "せ"),
        ("so", "そ"),
        ("za", "ざ"),
        ("ji", "じ"),
        ("zu", "ず"),
        ("ze", "ぜ"),
        ("zo", "ぞ"),
        ("ta", "た"),
        ("chi", "ち"),
        ("tsu", "つ"),
        ("te", "て"),
        ("to", "と"),
        ("da", "だ"),
        ("di", "ぢ"),
        ("du", "づ"),
        ("de", "で"),
        ("do", "ど"),
        ("na", "な"),
        ("ni", "に"),
        ("nu", "ぬ"),
        ("ne", "ね"),
        ("no", "の"),
        ("ha", "は"),
        ("hi", "ひ"),
        ("fu", "ふ"),
        ("he", "へ"),
        ("ho", "ほ"),
        ("ba", "ば"),
        ("bi", "び"),
        ("bu", "ぶ"),
        ("be", "べ"),
        ("bo", "ぼ"),
        ("pa", "ぱ"),
        ("pi", "ぴ"),
        ("pu", "ぷ"),
        ("pe", "ぺ"),
        ("po", "ぽ"),
        ("ma", "ま"),
        ("mi", "み"),
        ("mu", "む"),
        ("me", "め"),
        ("mo", "も"),
        ("ya", "や"),
        ("yu", "ゆ"),
        ("yo", "よ"),
        ("ra", "ら"),
        ("ri", "り"),
        ("ru", "る"),
        ("re", "れ"),
        ("ro", "ろ"),
        ("wa", "わ"),
        ("wo", "を"),
        ("nn", "ん"), // Handle 'nn' before 'n'
        ("n", "ん"),
    ];

    const ROMAJI_COMBINATIONS: [(&str, &str); 6] = [
        ("ssh", "っsh"),
        ("tch", "っch"),
        ("tt", "っt"),
        ("kk", "っk"),
        ("pp", "っp"),
        ("ss", "っs"),
    ];

    const BASIC_VOWELS: [(&str, &str); 5] = [
        ("a", "あ"),
        ("i", "い"),
        ("u", "う"),
        ("e", "え"),
        ("o", "お"),
    ];

    /// Converts romaji input to hiragana
    pub fn romaji_to_hiragana(input: &str) -> String {
        let mut result = input.to_lowercase();

        // Special case: handle 'n' before vowels
        result = result.replace("n'", "ん"); // Handle n' cases
         // Handle special combinations first
        for (romaji, replacement) in ROMAJI_COMBINATIONS.iter() {
            result = result.replace(romaji, replacement);
        }

        // Replace repeated consonants with っ
        let mut chars: Vec<char> = result.chars().collect();
        let mut i = 0;
        while i < chars.len().saturating_sub(1) {
            if chars[i] == chars[i + 1] && "kstpgdrfbjm".contains(chars[i]) {
                chars[i] = 'っ';
                chars.remove(i + 1);
            }
            i += 1;
        }
        result = chars.into_iter().collect();

        // Handle special cases
        result = result.replace("nb", "mb"); // Convert nb to mb
        result = result.replace("np", "mp"); // Convert np to mp

        // Replace compound sounds first (longer patterns should be replaced first)
        for (romaji, kana) in ROMAJI_TO_HIRAGANA.iter() {
            result = result.replace(romaji, kana);
        }

        // Replace basic vowels last
        for (romaji, kana) in BASIC_VOWELS.iter() {
            result = result.replace(romaji, kana);
        }

        // Handle long vowels
        result = result.replace("ou", "おう");
        result = result.replace("oo", "おう");
        result = result.replace("ei", "えい");

        result
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_romaji_conversion() {
            // Basic conversion tests
            assert_eq!(romaji_to_hiragana("konnichiwa"), "こんにちわ");
            assert_eq!(romaji_to_hiragana("kyoto"), "きょと");
            assert_eq!(romaji_to_hiragana("sakura"), "さくら");
            assert_eq!(romaji_to_hiragana("ninja"), "にんじゃ");
            assert_eq!(romaji_to_hiragana("gakkou"), "がっこう");

            // Test double consonants
            assert_eq!(romaji_to_hiragana("kitte"), "きって");
            assert_eq!(romaji_to_hiragana("zasshi"), "ざっし");

            // Test long vowels
            assert_eq!(romaji_to_hiragana("toukyou"), "とうきょう");
            assert_eq!(romaji_to_hiragana("obaasan"), "おばあさん");

            // Test special cases
            assert_eq!(romaji_to_hiragana("shinbun"), "しんぶん");
            assert_eq!(romaji_to_hiragana("sempai"), "せんぱい");

            // Test mixed case input
            assert_eq!(romaji_to_hiragana("YAMATO"), "やまと");
            assert_eq!(romaji_to_hiragana("KiMoNo"), "きもの");
        }

        #[test]
        fn test_reading_validation_with_romaji() {
            let accepted = vec!["かたかな".to_string(), "ひらがな".to_string()];
            assert!(validate_reading("katakana", &accepted));
            assert!(validate_reading("かたかな", &accepted));
            assert!(validate_reading("hiragana", &accepted));
            assert!(!validate_reading("kanji", &accepted));
        }

        #[test]
        fn test_string_normalization() {
            assert_eq!(normalize_string(" Test "), "test");
            assert_eq!(normalize_string("  UPPER  "), "upper");
            assert_eq!(normalize_string("\twhitespace\n"), "whitespace");
        }

        #[test]
        fn test_reading_validation() {
            let accepted = vec!["かたかな".to_string(), "ひらがな".to_string()];
            assert!(validate_reading("カタカナ", &accepted));
            assert!(validate_reading("かたかな", &accepted));
            assert!(!validate_reading("まちがい", &accepted));
        }

        #[test]
        fn test_svg_conversion() {
            let simple_svg = r#"
                <svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
                    <rect width="100" height="100" fill="black"/>
                </svg>"#;
            let result = svg_to_dynamic_image(simple_svg);
            assert!(result.is_ok());

            // Test invalid SVG
            let invalid_svg = "not an svg";
            assert!(svg_to_dynamic_image(invalid_svg).is_err());
        }
    }
}
