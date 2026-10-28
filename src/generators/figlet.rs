// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2023-2026 LibMake. All rights reserved.

//! A FIGlet renderer for the bundled `standard` font.
//!
//! It reads the FIGfont 2 (`.flf`) format and lays characters out with the
//! font's own layout rules: kerning, and the controlled smushing rules
//! (equal character, underscore, hierarchy, opposite pair, big X and
//! hardblank) or universal smushing. Only left-to-right fonts are
//! supported, which covers `standard`.
//!
//! `standard.flf` is by Glenn Chappell and Ian Chai, from the FIGlet
//! distribution; it is embedded unchanged, and its header comment lines
//! carry its permission notice.

use std::collections::HashMap;
use std::fmt;

const FONT: &str = include_str!("standard.flf");

const SM_EQUAL: i32 = 1;
const SM_LOWLINE: i32 = 2;
const SM_HIERARCHY: i32 = 4;
const SM_PAIR: i32 = 8;
const SM_BIGX: i32 = 16;
const SM_HARDBLANK: i32 = 32;
const SM_KERN: i32 = 64;
const SM_SMUSH: i32 = 128;

/// The 7 Deutsch characters a FIGfont carries after the 95 ASCII ones.
const DEUTSCH: [u32; 7] = [196, 214, 220, 228, 246, 252, 223];

/// A parsed FIGlet font.
#[derive(Debug, Clone)]
pub struct FIGlet {
    hardblank: char,
    height: usize,
    layout: i32,
    glyphs: HashMap<u32, Glyph>,
}

#[derive(Debug, Clone)]
struct Glyph {
    rows: Vec<Vec<char>>,
    width: usize,
}

/// Text rendered with a [`FIGlet`] font; its `Display` writes each row
/// followed by a newline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FIGure {
    lines: Vec<String>,
}

impl fmt::Display for FIGure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.lines.iter().try_for_each(|line| writeln!(f, "{line}"))
    }
}

impl FIGlet {
    /// The bundled `standard` font.
    ///
    /// # Errors
    ///
    /// Returns a description of the problem if the embedded font does not
    /// parse, which would be a packaging error.
    pub fn standard() -> Result<Self, String> {
        Self::from_content(FONT)
    }

    /// Parses a FIGfont 2 (`.flf`) font.
    ///
    /// # Errors
    ///
    /// Returns a description of the problem when the header is malformed
    /// or a character's rows are missing.
    pub fn from_content(contents: &str) -> Result<Self, String> {
        let lines: Vec<&str> = contents.lines().collect();
        let header = Header::parse(lines.first().ok_or("empty font")?)?;
        if lines.len() < header.comment_lines + 1 {
            return Err("font ends inside its comments".to_string());
        }
        let mut glyphs = HashMap::new();
        read_required(&lines, &header, &mut glyphs)?;
        read_codetagged(&lines, &header, &mut glyphs)?;
        Ok(Self {
            hardblank: header.hardblank,
            height: header.height,
            layout: header.layout,
            glyphs,
        })
    }

    /// Renders `message`, skipping characters the font lacks; `None` when
    /// nothing is left to render.
    #[must_use]
    pub fn convert(&self, message: &str) -> Option<FIGure> {
        let glyphs: Vec<&Glyph> = message
            .chars()
            .filter_map(|ch| self.glyphs.get(&(ch as u32)))
            .collect();
        if glyphs.is_empty() {
            return None;
        }
        let mut layout = Layout {
            font: self,
            prev_width: 0,
            cur_width: 0,
        };
        let mut rows = vec![Vec::new(); self.height];
        for glyph in glyphs {
            layout.add(&mut rows, glyph);
        }
        let lines = rows
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|c| if c == self.hardblank { ' ' } else { c })
                    .collect()
            })
            .collect();
        Some(FIGure { lines })
    }
}

struct Header {
    hardblank: char,
    height: usize,
    comment_lines: usize,
    layout: i32,
}

impl Header {
    fn parse(line: &str) -> Result<Self, String> {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 6 {
            return Err("header has fewer than 6 fields".to_string());
        }
        let full_layout = fields.get(7).and_then(|v| v.parse().ok());
        Ok(Self {
            hardblank: hardblank_of(fields[0])?,
            height: count(&fields, 1)?,
            comment_lines: count(&fields, 5)?,
            layout: effective_layout(full_layout, number(&fields, 4)?),
        })
    }
}

/// The hardblank is the last character of the signature (`flf2a$`).
fn hardblank_of(signature: &str) -> Result<char, String> {
    if signature.len() < 6 {
        return Err("header signature is too short".to_string());
    }
    signature[signature.len() - 1..]
        .chars()
        .next()
        .ok_or_else(|| "no hardblank".to_string())
}

fn number(fields: &[&str], i: usize) -> Result<i32, String> {
    fields[i]
        .parse()
        .map_err(|_| format!("header field {i} is not a number"))
}

fn count(fields: &[&str], i: usize) -> Result<usize, String> {
    usize::try_from(number(fields, i)?).map_err(|e| e.to_string())
}

/// The layout a FIGfont asks for: the full layout when the header has
/// one, otherwise the one its old layout implies.
fn effective_layout(full_layout: Option<i32>, old_layout: i32) -> i32 {
    match full_layout {
        Some(layout) => layout,
        None if old_layout == 0 => SM_KERN,
        None if old_layout < 0 => 0,
        None => (old_layout & 31) | SM_SMUSH,
    }
}

/// One row of a character: trailing spaces and the end mark removed, and
/// on the last row of a multi-row character a doubled end mark too.
fn glyph_row(
    line: &str,
    last_row: bool,
    height: usize,
) -> Result<Vec<char>, String> {
    let mut chars: Vec<char> =
        line.trim_end_matches(' ').chars().collect();
    let endmark =
        chars.pop().ok_or("character row without an end mark")?;
    if last_row && height != 1 && chars.last() == Some(&endmark) {
        let _ = chars.pop();
    }
    Ok(chars)
}

fn read_glyph(
    lines: &[&str],
    start: usize,
    height: usize,
) -> Result<Glyph, String> {
    let rows = (0..height)
        .map(|i| {
            let line = lines
                .get(start + i)
                .ok_or("font ends inside a character")?;
            glyph_row(line, i == height - 1, height)
        })
        .collect::<Result<Vec<_>, String>>()?;
    let width = rows.first().map_or(0, Vec::len);
    Ok(Glyph { rows, width })
}

/// The 95 printable ASCII characters, then the 7 Deutsch ones, in order.
fn read_required(
    lines: &[&str],
    header: &Header,
    glyphs: &mut HashMap<u32, Glyph>,
) -> Result<(), String> {
    let codes = (32..=126).chain(DEUTSCH);
    for (i, code) in codes.enumerate() {
        let start = 1 + header.comment_lines + i * header.height;
        if start >= lines.len() {
            break;
        }
        let _ = glyphs
            .insert(code, read_glyph(lines, start, header.height)?);
    }
    Ok(())
}

/// Characters after the required 102, each led by a line naming its code.
fn read_codetagged(
    lines: &[&str],
    header: &Header,
    glyphs: &mut HashMap<u32, Glyph>,
) -> Result<(), String> {
    let offset = 1 + header.comment_lines + 102 * header.height;
    if offset >= lines.len() {
        return Ok(());
    }
    let block = header.height + 1;
    if (lines.len() - offset) % block != 0 {
        return Err("code-tagged characters do not fill whole blocks"
            .to_string());
    }
    for start in (offset..lines.len()).step_by(block) {
        if let Some(code) = codetag(lines[start])? {
            let _ = glyphs.insert(
                code,
                read_glyph(lines, start + 1, header.height)?,
            );
        }
    }
    Ok(())
}

/// The code a code-tag line names, in decimal, `0x` hex or leading-zero
/// octal; `None` for a negative code, which FIGfonts use for unmapped
/// characters.
fn codetag(line: &str) -> Result<Option<u32>, String> {
    let tag = line.split_whitespace().next().ok_or("empty code tag")?;
    let digits = tag.trim_start_matches(['-', '+']);
    let parsed = if let Some(hex) = digits
        .strip_prefix("0x")
        .or_else(|| digits.strip_prefix("0X"))
    {
        i64::from_str_radix(hex, 16)
    } else if digits.len() > 1 && digits.starts_with('0') {
        i64::from_str_radix(&digits[1..], 8)
    } else {
        digits.parse()
    }
    .map_err(|e| e.to_string())?;
    if tag.starts_with('-') {
        return Ok(None);
    }
    u32::try_from(parsed).map(Some).map_err(|e| e.to_string())
}

/// Lays glyphs out left to right, overlapping each one with the rows so
/// far as far as the font's layout allows.
struct Layout<'a> {
    font: &'a FIGlet,
    prev_width: usize,
    cur_width: usize,
}

impl Layout<'_> {
    fn add(&mut self, rows: &mut [Vec<char>], glyph: &Glyph) {
        self.cur_width = glyph.width;
        let overlap = self.overlap(rows, glyph);
        for (row, glyph_row) in rows.iter_mut().zip(&glyph.rows) {
            self.join(row, glyph_row, overlap);
        }
        self.prev_width = self.cur_width;
    }

    /// How many columns the glyph can move left into the rows so far: the
    /// smallest amount any row allows.
    fn overlap(&self, rows: &[Vec<char>], glyph: &Glyph) -> usize {
        if self.font.layout & (SM_SMUSH | SM_KERN) == 0 {
            return 0;
        }
        rows.iter()
            .zip(&glyph.rows)
            .map(|(left, right)| self.row_overlap(left, right))
            .fold(self.cur_width, usize::min)
    }

    fn row_overlap(&self, left: &[char], right: &[char]) -> usize {
        let left_len = trimmed_len(left);
        let right_edge =
            right.iter().position(|c| *c != ' ').unwrap_or(right.len());
        if left_len == 0 {
            return right_edge + left.len();
        }
        let joins =
            self.edges_join(left[left_len - 1], right.get(right_edge));
        right_edge + left.len() - left_len + usize::from(joins)
    }

    /// Whether two rows' facing edge characters may share a column.
    fn edges_join(&self, ch1: char, ch2: Option<&char>) -> bool {
        ch1 == '\0'
            || ch1 == ' '
            || matches!(ch2, Some(&c) if c != '\0'
                && self.smush(ch1, c).is_some())
    }

    fn join(
        &self,
        row: &mut Vec<char>,
        glyph_row: &[char],
        overlap: usize,
    ) {
        for i in 0..overlap {
            let idx =
                isize_of(row.len()) - isize_of(overlap) + isize_of(i);
            let index = usize::try_from(idx).ok();
            let left =
                index.and_then(|j| row.get(j).copied()).unwrap_or('\0');
            let right = glyph_row.get(i).copied().unwrap_or('\0');
            if let (Some(smushed), Some(j)) =
                (self.smush(left, right), index)
            {
                row[j] = smushed;
            }
        }
        row.extend(glyph_row.iter().skip(overlap));
    }

    /// The character two overlapping characters become, or `None` when
    /// they cannot overlap.
    fn smush(&self, left: char, right: char) -> Option<char> {
        if left == ' ' {
            return Some(right);
        }
        if right == ' ' {
            return Some(left);
        }
        if left == '\0'
            || right == '\0'
            || self.prev_width < 2
            || self.cur_width < 2
        {
            return None;
        }
        let layout = self.font.layout;
        if layout & SM_SMUSH == 0 {
            return None;
        }
        if layout & 63 == 0 {
            return Some(self.universal(left, right));
        }
        self.controlled(layout, left, right)
    }

    /// Universal smushing: the later character wins, except over a hardblank.
    fn universal(&self, left: char, right: char) -> char {
        if right == self.font.hardblank {
            left
        } else {
            right
        }
    }

    fn controlled(
        &self,
        layout: i32,
        left: char,
        right: char,
    ) -> Option<char> {
        if let Some(result) = self.hardblank_rule(layout, left, right) {
            return result;
        }
        if layout & SM_EQUAL != 0 && left == right {
            return Some(left);
        }
        rule_on(layout, SM_LOWLINE, lowline(left, right))
            .or_else(|| {
                rule_on(layout, SM_HIERARCHY, hierarchy(left, right))
            })
            .or_else(|| {
                rule_on(layout, SM_PAIR, opposite_pair(left, right))
            })
            .or_else(|| rule_on(layout, SM_BIGX, big_x(left, right)))
    }
}

impl Layout<'_> {
    /// `None` when neither character is a hardblank; otherwise the outcome:
    /// two hardblanks merge under the hardblank rule, anything else with a
    /// hardblank does not overlap.
    fn hardblank_rule(
        &self,
        layout: i32,
        left: char,
        right: char,
    ) -> Option<Option<char>> {
        let hardblank = self.font.hardblank;
        if left != hardblank && right != hardblank {
            return None;
        }
        let both = left == hardblank && right == hardblank;
        Some((both && layout & SM_HARDBLANK != 0).then(|| left))
    }
}

fn rule_on(
    layout: i32,
    rule: i32,
    result: Option<char>,
) -> Option<char> {
    (layout & rule != 0).then(|| result).flatten()
}

/// An underscore gives way to a border character.
fn lowline(left: char, right: char) -> Option<char> {
    const BORDERS: &str = "|/\\[]{}()<>";
    if left == '_' && BORDERS.contains(right) {
        Some(right)
    } else if right == '_' && BORDERS.contains(left) {
        Some(left)
    } else {
        None
    }
}

/// Of two characters from different classes, the later class wins.
fn hierarchy(left: char, right: char) -> Option<char> {
    const CLASSES: [(&str, &str); 5] = [
        ("|", "/\\[]{}()<>"),
        ("/\\", "[]{}()<>"),
        ("[]", "{}()<>"),
        ("{}", "()<>"),
        ("()", "<>"),
    ];
    CLASSES.iter().find_map(|(class, later)| {
        if class.contains(left) && later.contains(right) {
            Some(right)
        } else if class.contains(right) && later.contains(left) {
            Some(left)
        } else {
            None
        }
    })
}

/// Opposite brackets, braces or parentheses become a bar.
fn opposite_pair(left: char, right: char) -> Option<char> {
    let pair = [left, right];
    let opposite = [['[', ']'], ['{', '}'], ['(', ')']]
        .iter()
        .any(|p| pair == *p || pair == [p[1], p[0]]);
    opposite.then(|| '|')
}

/// `/\` becomes `|`, `\/` becomes `Y` and `><` becomes `X`.
fn big_x(left: char, right: char) -> Option<char> {
    match (left, right) {
        ('/', '\\') => Some('|'),
        ('\\', '/') => Some('Y'),
        ('>', '<') => Some('X'),
        _ => None,
    }
}

/// The row's length without trailing blanks.
fn trimmed_len(row: &[char]) -> usize {
    row.iter().rposition(|c| *c != ' ').map_or(0, |i| i + 1)
}

fn isize_of(n: usize) -> isize {
    isize::try_from(n).unwrap_or(isize::MAX)
}
