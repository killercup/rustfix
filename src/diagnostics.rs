//! Rustc Diagnostic JSON Output
//!
//! The following data types are copied from [rust-lang/rust](https://github.com/rust-lang/rust/blob/de78655bca47cac8e783dbb563e7e5c25c1fae40/src/libsyntax/json.rs)

use std::fmt::Write; // <- unused on purpose!

#[derive(Serialize, Deserialize, Debug)]
pub struct Diagnostic {
    /// The primary error message.
    pub message: String,
    pub code: Option<DiagnosticCode>,
    /// "error: internal compiler error", "error", "warning", "note", "help".
    pub level: String,
    pub spans: Vec<DiagnosticSpan>,
    /// Associated diagnostic messages.
    pub children: Vec<Diagnostic>,
    /// The message as rustc would render it. Currently this is only
    /// `Some` for "suggestions", but eventually it will include all
    /// snippets.
    pub rendered: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiagnosticSpan {
    pub file_name: String,
    pub byte_start: usize,
    pub byte_end: usize,
    /// 1-based.
    pub line_start: usize,
    pub line_end: usize,
    /// 1-based, character offset.
    pub column_start: usize,
    pub column_end: usize,
    /// Is this a "primary" span -- meaning the point, or one of the points,
    /// where the error occurred?
    pub is_primary: bool,
    /// Source text from the start of line_start to the end of line_end.
    pub text: Vec<DiagnosticSpanLine>,
    /// Label that should be placed at this location (if any)
    pub label: Option<String>,
    /// If we are suggesting a replacement, this will contain text
    /// that should be sliced in atop this span. You may prefer to
    /// load the fully rendered version from the parent `Diagnostic`,
    /// however.
    pub suggested_replacement: Option<String>,
    /// Macro invocations that created the code at this span, if any.
    #[serde(bound="")]
    pub expansion: Option<Box<DiagnosticSpanMacroExpansion>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiagnosticSpanLine {
    pub text: String,

    /// 1-based, character offset in self.text.
    pub highlight_start: usize,
    pub highlight_end: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiagnosticSpanMacroExpansion {
    /// span where macro was applied to generate this code; note that
    /// this may itself derive from a macro (if
    /// `span.expansion.is_some()`)
    pub span: DiagnosticSpan,

    /// name of macro that was applied (e.g., "foo!" or "#[derive(Eq)]")
    pub macro_decl_name: String,

    /// span where macro was defined (if known)
    pub def_site_span: Option<DiagnosticSpan>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiagnosticCode {
    /// The code itself.
    pub code: String,
    /// An explanation for the code.
    pub explanation: Option<String>,
}
