use std::borrow::Cow;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "one switch per kind of markdown, as in discord.js"
)]
pub struct EscapeOptions {
    code_block: bool,
    inline_code: bool,
    bold: bool,
    italic: bool,
    underline: bool,
    strikethrough: bool,
    spoiler: bool,
    code_block_content: bool,
    inline_code_content: bool,
    backslash: bool,
    heading: bool,
    bulleted_list: bool,
    numbered_list: bool,
    masked_link: bool,
}

impl Default for EscapeOptions {
    fn default() -> Self {
        Self::new()
    }
}

macro_rules! setters {
    ($($(#[$meta:meta])* $field:ident,)*) => {
        $(
            $(#[$meta])*
            pub const fn $field(mut self, on: bool) -> Self {
                self.$field = on;
                self
            }
        )*
    };
}

impl EscapeOptions {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            code_block: true,
            inline_code: true,
            bold: true,
            italic: true,
            underline: true,
            strikethrough: true,
            spoiler: true,
            code_block_content: true,
            inline_code_content: true,
            backslash: true,
            heading: false,
            bulleted_list: false,
            numbered_list: false,
            masked_link: false,
        }
    }

    #[must_use]
    pub const fn none() -> Self {
        Self {
            code_block: false,
            inline_code: false,
            bold: false,
            italic: false,
            underline: false,
            strikethrough: false,
            spoiler: false,
            code_block_content: true,
            inline_code_content: true,
            backslash: false,
            heading: false,
            bulleted_list: false,
            numbered_list: false,
            masked_link: false,
        }
    }

    setters! {

        #[must_use]
        code_block,

        #[must_use]
        inline_code,

        #[must_use]
        bold,

        #[must_use]
        italic,

        #[must_use]
        underline,

        #[must_use]
        strikethrough,

        #[must_use]
        spoiler,

        #[must_use]
        code_block_content,

        #[must_use]
        inline_code_content,

        #[must_use]
        backslash,

        #[must_use]
        heading,

        #[must_use]
        bulleted_list,

        #[must_use]
        numbered_list,

        #[must_use]
        masked_link,
    }

    #[must_use]
    pub fn escape<'a>(&self, text: &'a str) -> Cow<'a, str> {
        let mut scanner = Scanner {
            options: *self,
            bytes: text.as_bytes(),
            text,
            out: None,
            copied: 0,
            line: Line::NONE,
        };
        scanner.run();
        scanner.finish()
    }
}

#[must_use]
pub fn escape_markdown(text: &str) -> Cow<'_, str> {
    EscapeOptions::new().escape(text)
}

macro_rules! escapers {
    ($($function:ident => $option:ident, $js:literal, $what:literal, $input:literal => $output:literal;)*) => {
        $(
            #[doc = concat!("Escapes only ", $what, " in `text` ([`EscapeOptions::", stringify!($option), "`]).")]
            #[doc = concat!("discord.js counterpart: `", $js, "`.")]
            #[doc = concat!("assert_eq!(crate::formatters::", stringify!($function), "(", $input, "), ", $output, ");")]
            #[must_use]
            pub fn $function(text: &str) -> Cow<'_, str> {
                EscapeOptions::none().$option(true).escape(text)
            }
        )*
    };
}

escapers! {
    escape_code_block => code_block, "escapeCodeBlock", "runs of three or more backticks",
        r#""```rust```""# => r#"r"\`\`\`rust\`\`\`""#;
    escape_inline_code => inline_code, "escapeInlineCode", "runs of one or two backticks",
        r#""`a` ``b``""# => r#"r"\`a\` \`\`b\`\`""#;
    escape_bold => bold, "escapeBold", "`**`", r#""**a**""# => r#"r"\*\*a\*\*""#;
    escape_italic => italic, "escapeItalic", "a single `*` or `_`",
        r#""*a* _b_""# => r#"r"\*a\* \_b\_""#;
    escape_underline => underline, "escapeUnderline", "`__`", r#""__a__""# => r#"r"\_\_a\_\_""#;
    escape_strikethrough => strikethrough, "escapeStrikethrough", "`~~`",
        r#""~~a~~ ~b""# => r#"r"\~\~a\~\~ ~b""#;
    escape_spoiler => spoiler, "escapeSpoiler", "`||`", r#""||a||""# => r#"r"\|\|a\|\|""#;
    escape_escape => backslash, "escapeEscape", "backslashes", r#"r"a\b""# => r#"r"a\\b""#;
    escape_heading => heading, "escapeHeading", "headings at the start of a line",
        r###""## Title""### => r###"r"\## Title""###;
    escape_bulleted_list => bulleted_list, "escapeBulletedList",
        "bullets at the start of a line", r#""- item""# => r#"r"\- item""#;
    escape_numbered_list => numbered_list, "escapeNumberedList",
        "numbered list items at the start of a line", r#""1. item""# => r#"r"1\. item""#;
    escape_masked_link => masked_link, "escapeMaskedLink", "masked links",
        r#""[a](https://b.c)""# => r#"r"\[a](https://b.c)""#;
}

const NONE: usize = usize::MAX;

#[derive(Clone, Copy, Debug)]
struct Line {
    end: usize,

    marks: [usize; 3],

    link_middle: usize,
}

impl Line {
    const NONE: Self = Self {
        end: 0,
        marks: [NONE; 3],
        link_middle: NONE,
    };
}

struct Scanner<'a> {
    options: EscapeOptions,
    text: &'a str,
    bytes: &'a [u8],
    out: Option<String>,
    copied: usize,
    line: Line,
}

const fn is_space(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | b'\n' | b'\r' | 0x0B | 0x0C)
}

const fn table(bytes: &[u8]) -> [bool; 256] {
    let mut table = [false; 256];
    let mut index = 0;
    while index < bytes.len() {
        table[bytes[index] as usize] = true;
        index += 1;
    }
    table
}

static STOPS: [bool; 256] = table(b"<\\*_~|`[:");

static LINE_STOPS: [bool; 256] = table(b"<\\*_~|`[:\n#-.");

impl<'a> Scanner<'a> {
    fn escape_at(&mut self, index: usize) {
        let out = self
            .out
            .get_or_insert_with(|| String::with_capacity(self.text.len() + 16));
        out.push_str(&self.text[self.copied..index]);
        out.push('\\');
        self.copied = index;
    }

    fn finish(self) -> Cow<'a, str> {
        match self.out {
            None => Cow::Borrowed(self.text),
            Some(mut out) => {
                out.push_str(&self.text[self.copied..]);
                Cow::Owned(out)
            }
        }
    }

    fn run_length(&self, start: usize, byte: u8) -> usize {
        self.bytes[start..]
            .iter()
            .position(|&other| other != byte)
            .unwrap_or(self.bytes.len() - start)
    }

    fn is_mark(&self, index: usize) -> bool {
        self.line.marks.contains(&index)
    }

    fn run(&mut self) {
        let options = self.options;
        let lines = options.heading
            || options.bulleted_list
            || options.numbered_list
            || options.masked_link;
        let stops = if lines { &LINE_STOPS } else { &STOPS };
        let len = self.bytes.len();
        let mut index = 0;
        while index < len {
            match self.bytes[index..]
                .iter()
                .position(|&byte| stops[usize::from(byte)])
            {
                Some(offset) => index += offset,
                None => break,
            }
            if lines && index >= self.line.end {
                self.load_line(index);
            }
            index = match self.bytes[index] {
                b':' => self.link(index),
                b'<' => self.token(index),
                b'\\' => {
                    if self.options.backslash {
                        self.escape_at(index);
                    }
                    index + 1
                }
                b'*' => self.emphasis(index, b'*', self.options.bold),
                b'_' => self.emphasis(index, b'_', self.options.underline),
                b'~' => self.pairs(index, b'~', self.options.strikethrough),
                b'|' => self.pairs(index, b'|', self.options.spoiler),
                b'`' => self.backticks(index),
                b'[' => {
                    let middle = self.line.link_middle;
                    if self.options.masked_link && middle != NONE && middle >= index + 2 {
                        self.escape_at(index);
                    }
                    index + 1
                }
                _ => {
                    if self.is_mark(index) {
                        self.escape_at(index);
                    }
                    index + 1
                }
            };
        }
    }

    fn load_line(&mut self, index: usize) {
        let bytes = self.bytes;
        let previous_end = self.line.end.min(index);
        let start = bytes[previous_end..index]
            .iter()
            .rposition(|&byte| byte == b'\n')
            .map_or(previous_end, |offset| previous_end + offset + 1);
        let end = bytes[index..]
            .iter()
            .position(|&byte| byte == b'\n')
            .map_or(bytes.len(), |offset| index + offset + 1);
        let content_end = if end > start && bytes[end - 1] == b'\n' {
            end - 1
        } else {
            end
        };
        let line = &bytes[start..content_end];
        let mut marks = [NONE; 3];
        let options = self.options;
        if options.bulleted_list || options.heading || options.numbered_list {
            let spaces = line.iter().take_while(|&&byte| byte == b' ').count();

            if options.bulleted_list
                && matches!(line.get(spaces), Some(b'-' | b'*'))
                && line.get(spaces + 1) == Some(&b' ')
            {
                marks[0] = start + spaces;
            }

            if options.heading {
                let mut at = spaces;
                if spaces <= 2
                    && matches!(line.get(at), Some(b'-' | b'*'))
                    && line.get(at + 1) == Some(&b' ')
                {
                    at += 2;
                    at += line[at..].iter().take_while(|&&byte| byte == b' ').count();
                }
                let hashes = line[at..].iter().take_while(|&&byte| byte == b'#').count();
                if (1..=3).contains(&hashes) && line.get(at + hashes) == Some(&b' ') {
                    marks[1] = start + at;
                }
            }

            if options.numbered_list {
                let digits = line[spaces..]
                    .iter()
                    .take_while(|byte| byte.is_ascii_digit())
                    .count();
                if digits > 0 && line.get(spaces + digits) == Some(&b'.') {
                    marks[2] = start + spaces + digits;
                }
            }
        }
        let mut link_middle = NONE;
        if options.masked_link
            && let Some(close) = line.iter().rposition(|&byte| byte == b')')
            && close >= 3
        {
            link_middle = line[..close - 1]
                .windows(2)
                .rposition(|pair| pair == b"](")
                .map_or(NONE, |middle| start + middle);
        }
        self.line = Line {
            end,
            marks,
            link_middle,
        };
    }

    fn link(&self, index: usize) -> usize {
        let bytes = self.bytes;
        let is_link = bytes[index..].starts_with(b"://")
            && bytes.get(index + 3).is_some_and(|&byte| !is_space(byte))
            && ((index >= 5 && bytes[index - 5..index].eq_ignore_ascii_case(b"https"))
                || (index >= 4 && bytes[index - 4..index].eq_ignore_ascii_case(b"http")));
        if !is_link {
            return index + 1;
        }
        let rest = &bytes[index + 3..];
        index
            + 3
            + rest
                .iter()
                .position(|&byte| is_space(byte))
                .unwrap_or(rest.len())
    }

    fn token(&self, index: usize) -> usize {
        let rest = &self.bytes[index + 1..];
        let Some(close) = rest
            .iter()
            .position(|&byte| matches!(byte, b'<' | b'>' | b'\n'))
        else {
            return index + 1;
        };
        if rest[close] != b'>' || close == 0 {
            return index + 1;
        }
        let content = &rest[..close];
        let protected = if content[0] == b'/' {
            let digits = content
                .iter()
                .rev()
                .take_while(|byte| byte.is_ascii_digit())
                .count();
            digits > 0 && content.len() > digits + 1 && content[content.len() - digits - 1] == b':'
        } else {
            !content.iter().any(|&byte| is_space(byte))
                && (matches!(content[0], b'@' | b'#' | b'+')
                    || content.iter().any(|&byte| matches!(byte, b':' | b'@')))
        };
        if protected {
            index + close + 2
        } else {
            index + 1
        }
    }

    fn emphasis(&mut self, index: usize, byte: u8, pairs: bool) -> usize {
        let length = self.run_length(index, byte);
        let single = self.options.italic;
        for offset in 0..length {
            let escaped = match (pairs, single) {
                (true, true) => true,
                (true, false) => offset < length / 2 * 2,
                (false, true) => length % 2 == 1 && offset == 0,
                (false, false) => false,
            };
            if escaped || self.is_mark(index + offset) {
                self.escape_at(index + offset);
            }
        }
        index + length
    }

    fn pairs(&mut self, index: usize, byte: u8, on: bool) -> usize {
        let length = self.run_length(index, byte);
        if on {
            for offset in 0..length / 2 * 2 {
                self.escape_at(index + offset);
            }
        }
        index + length
    }

    fn escape_all(&mut self, index: usize, count: usize) {
        for offset in 0..count {
            self.escape_at(index + offset);
        }
    }

    fn backticks(&mut self, index: usize) -> usize {
        let options = self.options;

        if !options.code_block_content && self.bytes[index..].starts_with(b"```") {
            let after = index + 3;
            let close = self.bytes[after..]
                .windows(3)
                .position(|window| window == b"```")
                .map(|offset| after + offset);
            if options.code_block {
                self.escape_all(index, 3);
            }
            let Some(close) = close else {
                return after;
            };
            if options.code_block {
                self.escape_all(close, 3);
            }
            return close + 3;
        }
        let length = self.run_length(index, b'`');
        if length == 1 && !options.inline_code_content {
            let close = self.single_backtick(index + 1);
            if options.inline_code {
                self.escape_at(index);
            }
            let Some(close) = close else {
                return index + 1;
            };
            if options.inline_code {
                self.escape_at(close);
            }
            return close + 1;
        }
        let on = if length >= 3 {
            options.code_block
        } else {
            options.inline_code
        };
        if on {
            self.escape_all(index, length);
        }
        index + length
    }

    fn single_backtick(&self, from: usize) -> Option<usize> {
        let mut index = from;
        while let Some(offset) = self.bytes[index..].iter().position(|&byte| byte == b'`') {
            let start = index + offset;
            let length = self.run_length(start, b'`');
            if length == 1 {
                return Some(start);
            }
            if length >= 3 && !self.options.code_block_content {
                return None;
            }
            index = start + length;
        }
        None
    }
}
