//! The `TextCell` type for the details and lines views.

use ansi_term::{Style, ANSIString, ANSIStrings};
use unicode_width::UnicodeWidthStr;


/// An individual cell that holds text in a table, used in the details and
/// lines views to store ANSI-terminal-formatted data before it is printed.
///
/// A text cell is made up of zero or more strings coupled with the
/// pre-computed length of all the strings combined. When constructing details
/// or grid-details tables, the length will have to be queried multiple times,
/// so it makes sense to cache it.
///
/// (This used to be called `Cell`, but was renamed because there’s a Rust
/// type by that name too.)
#[derive(PartialEq, Debug, Clone, Default)]
pub struct TextCell {

    /// The contents of this cell, as a vector of ANSI-styled strings.
    pub contents: TextCellContents,

    /// The Unicode “display width” of this cell, in characters.
    ///
    /// As with the `File` type’s width, this is related to the number of
    /// *graphemes*, rather than *characters*, in the cell: most are 1 column
    /// wide, but in some contexts, certain characters are two columns wide.
    pub length: usize,
}

impl TextCell {

    /// Creates a new text cell that holds the given text in the given style,
    /// computing the Unicode width of the text.
    pub fn paint(style: Style, text: String) -> Self {
        TextCell {
            length: text.width(),
            contents: vec![ style.paint(text) ],
        }
    }

    /// Creates a new text cell that holds the given text in the given style,
    /// computing the Unicode width of the text. (This could be merged with
    /// `paint`, but.)
    pub fn paint_str(style: Style, text: &'static str) -> Self {
        TextCell {
            length: text.len(),
            contents: vec![ style.paint(text) ],
        }
    }

    /// Creates a new “blank” text cell that contains a single hyphen in the
    /// given style, which should be the “punctuation” style from a `Colours`
    /// value.
    ///
    /// This is used in place of empty table cells, as it is easier to read
    /// tabular data when there is *something* in each cell.
    pub fn blank(style: Style) -> Self {
        TextCell {
            length: 1,
            contents: vec![ style.paint("-") ],
        }
    }

    /// Adds the given number of unstyled spaces after this cell.
    ///
    /// This method allocates a `String` to hold the spaces.
    pub fn add_spaces(&mut self, count: usize) {
        use std::iter::repeat;

        self.length += count;

        let spaces: String = repeat(' ').take(count).collect();
        self.contents.push(Style::default().paint(spaces));
    }

    /// Adds the contents of another `ANSIString` to the end of this cell.
    pub fn push(&mut self, string: ANSIString<'static>, length: usize) {
        self.contents.push(string);
        self.length += length;
    }

    /// Adds all the contents of another `TextCell` to the end of this cell.
    pub fn append(&mut self, other: TextCell) {
        self.length += other.length;
        self.contents.extend(other.contents);
    }

    /// Produces an `ANSIStrings` value that can be used to print the styled
    /// values of this cell as an ANSI-terminal-formatted string.
    pub fn strings(&self) -> ANSIStrings {
        ANSIStrings(&self.contents)
    }
}


// I’d like to eventually abstract cells so that instead of *every* cell
// storing a vector, only variable-length cells would, and individual cells
// would just store an array of a fixed length (which would usually be just 1
// or 2), which wouldn’t require a heap allocation.
//
// For examples, look at the `render_*` methods in the `Table` object in the
// details view:
//
// - `render_blocks`, `inode`, and `links` will always return a
//   one-string-long TextCell;
// - `render_size` will return one or two strings in a TextCell, depending on
//   the size and whether one is present;
// - `render_permissions` will return ten or eleven strings;
// - `filename` and `symlink_filename` in the output module root return six or
//   five strings.
//
// In none of these cases are we dealing with a *truly variable* number of
// strings: it is only when the strings are concatenated together do we need a
// growable, heap-allocated buffer.
//
// So it would be nice to abstract the `TextCell` type so instead of a `Vec`,
// it can use anything of type `T: IntoIterator<Item=ANSIString<’static>>`.
// This would allow us to still hold all the data, but allocate less.
//
// But exa still has bugs and I need to fix those first :(


/// The contents of a text cell, as a vector of ANSI-styled strings.
///
/// It’s possible to use this type directly in the case where you want a
/// `TextCell` but aren’t concerned with tracking its width, because it occurs
/// in the final cell of a table or grid and there’s no point padding it. This
/// happens when dealing with file names.
pub type TextCellContents = Vec<ANSIString<'static>>;