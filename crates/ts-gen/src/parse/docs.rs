//! Extract JSDoc comments from the oxc comment list.
//!
//! oxc stores all comments in a flat sorted `Vec<Comment>` on `Program`.
//! Each `Comment` has an `attached_to` field — the byte offset of the token
//! the comment is leading. We match JSDoc (`/** ... */`) comments to AST
//! nodes by comparing `comment.attached_to` with `node.span.start`.

use oxc_ast::ast::Comment;

/// Provides JSDoc lookup by span position.
pub struct DocComments<'a> {
    comments: &'a [Comment],
    source: &'a str,
}

impl<'a> DocComments<'a> {
    pub fn new(comments: &'a [Comment], source: &'a str) -> Self {
        Self { comments, source }
    }

    /// Find the JSDoc comment attached to the node starting at `span_start`.
    ///
    /// Returns the cleaned doc text (leading `*` and whitespace stripped per line),
    /// or `None` if no JSDoc is attached.
    pub fn for_span(&self, span_start: u32) -> Option<String> {
        // Find the last JSDoc comment attached to this position.
        // (There could be multiple leading comments; we want the JSDoc one closest to the node.)
        let jsdoc = self
            .comments
            .iter()
            .rev()
            .find(|c| c.attached_to == span_start && c.is_jsdoc())?;

        let content_span = jsdoc.content_span();
        let raw = &self.source[content_span.start as usize..content_span.end as usize];

        Some(clean_jsdoc(raw))
    }
}

/// Clean raw JSDoc content (between `/**` and `*/`) and convert to Rust doc conventions.
///
/// - Strips leading `*` and whitespace per line
/// - Converts `@param name - desc` → `# Arguments` section with `* \`name\` - desc`
/// - Converts `@returns desc` → `# Returns` section
/// - Converts `@example` blocks into fenced ` ```js ` code blocks
/// - Removes empty leading/trailing lines
fn clean_jsdoc(raw: &str) -> String {
    let lines: Vec<&str> = raw.lines().collect();
    let mut cleaned: Vec<&str> = Vec::new();

    for line in &lines {
        let trimmed = line.trim();
        // Strip leading `* ` or `*`
        let stripped = if let Some(rest) = trimmed.strip_prefix("* ") {
            rest
        } else if let Some(rest) = trimmed.strip_prefix('*') {
            rest
        } else {
            trimmed
        };
        cleaned.push(stripped);
    }

    // Remove empty leading and trailing lines
    while cleaned.first().is_some_and(|l| l.is_empty()) {
        cleaned.remove(0);
    }
    while cleaned.last().is_some_and(|l| l.is_empty()) {
        cleaned.pop();
    }

    convert_jsdoc_tags(&cleaned)
}

/// Convert JSDoc tags in cleaned lines to Rust doc conventions.
///
/// Collects description lines, `@param` entries, `@returns`, and `@example` blocks,
/// then re-emits them in idiomatic Rust doc order.
fn convert_jsdoc_tags(lines: &[&str]) -> String {
    let mut description: Vec<String> = Vec::new();
    let mut params: Vec<String> = Vec::new();
    let mut returns: Option<String> = None;
    let mut examples: Vec<Vec<String>> = Vec::new();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];

        if let Some(rest) = line.strip_prefix("@param ") {
            // @param name - description  or  @param name description
            params.push(format_param(rest));
        } else if let Some(rest) = line
            .strip_prefix("@returns ")
            .or_else(|| line.strip_prefix("@return "))
        {
            returns = Some(rest.to_string());
        } else if line == "@example" {
            // Collect all lines until the next tag or end
            let mut code_lines = Vec::new();
            i += 1;
            while i < lines.len() && !lines[i].starts_with('@') {
                code_lines.push(lines[i].to_string());
                i += 1;
            }
            // Trim empty leading/trailing lines from example
            while code_lines.first().is_some_and(|l| l.is_empty()) {
                code_lines.remove(0);
            }
            while code_lines.last().is_some_and(|l| l.is_empty()) {
                code_lines.pop();
            }
            if !code_lines.is_empty() {
                examples.push(code_lines);
            }
            continue; // don't increment i again
        } else if line.starts_with('@') {
            // Unknown tag — pass through as-is
            description.push(line.to_string());
        } else {
            description.push(line.to_string());
        }

        i += 1;
    }

    // Build the output
    let mut out: Vec<String> = Vec::new();

    // Description
    out.extend(description);

    // Arguments section
    if !params.is_empty() {
        // Add blank line separator if we have preceding content
        if !out.is_empty() && !out.last().is_none_or(|l| l.is_empty()) {
            out.push(String::new());
        }
        out.push("## Arguments".to_string());
        out.push(String::new());
        for p in &params {
            out.push(p.clone());
        }
    }

    // Returns section
    if let Some(ret) = &returns {
        if !out.is_empty() && !out.last().is_none_or(|l| l.is_empty()) {
            out.push(String::new());
        }
        out.push("## Returns".to_string());
        out.push(String::new());
        out.push(ret.clone());
    }

    // Examples
    for example in &examples {
        if !out.is_empty() && !out.last().is_none_or(|l| l.is_empty()) {
            out.push(String::new());
        }
        out.push("## Example".to_string());
        out.push(String::new());
        out.push("```js".to_string());
        for line in example {
            out.push(line.clone());
        }
        out.push("```".to_string());
    }

    // Trim trailing empty lines
    while out.last().is_some_and(|l| l.is_empty()) {
        out.pop();
    }

    out.join("\n")
}

/// Format a `@param` rest string into a Rust-style argument list item.
///
/// Input forms:
/// - `name - description`
/// - `name description`
/// - `{type} name - description` (type is stripped)
///
/// Output: `* \`name\` - description`
fn format_param(rest: &str) -> String {
    let rest = rest.trim();

    // Strip optional JSDoc type annotation `{...}`
    let rest = if rest.starts_with('{') {
        if let Some(end) = rest.find('}') {
            rest[end + 1..].trim()
        } else {
            rest
        }
    } else {
        rest
    };

    // Split into name and description
    if let Some((name, desc)) = rest.split_once(" - ") {
        format!("* `{}` - {}", name.trim(), desc.trim())
    } else if let Some((name, desc)) = rest.split_once(' ') {
        format!("* `{}` - {}", name.trim(), desc.trim())
    } else {
        format!("* `{rest}`")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_single_line() {
        assert_eq!(
            clean_jsdoc(" A simple description "),
            "A simple description"
        );
    }

    #[test]
    fn test_clean_multi_line() {
        let raw = "\n * First line\n * Second line\n ";
        assert_eq!(clean_jsdoc(raw), "First line\nSecond line");
    }

    #[test]
    fn test_param_conversion() {
        let raw = "\n * Does a thing.\n * @param x - the value\n * @returns the result\n ";
        assert_eq!(
            clean_jsdoc(raw),
            "Does a thing.\n\n## Arguments\n\n* `x` - the value\n\n## Returns\n\nthe result"
        );
    }

    #[test]
    fn test_param_without_dash() {
        let raw = "\n * Hello.\n * @param source Source code to parse\n ";
        assert_eq!(
            clean_jsdoc(raw),
            "Hello.\n\n## Arguments\n\n* `source` - Source code to parse"
        );
    }

    #[test]
    fn test_multiple_params() {
        let raw = "\n * Parse it.\n * @param source Source code\n * @param name Optional name\n * @returns The parsed result.\n ";
        assert_eq!(
            clean_jsdoc(raw),
            "Parse it.\n\n## Arguments\n\n* `source` - Source code\n* `name` - Optional name\n\n## Returns\n\nThe parsed result."
        );
    }

    #[test]
    fn test_example_block() {
        let raw = "\n * Do something.\n * @example\n * const x = foo();\n * console.log(x);\n ";
        assert_eq!(
            clean_jsdoc(raw),
            "Do something.\n\n## Example\n\n```js\nconst x = foo();\nconsole.log(x);\n```"
        );
    }

    #[test]
    fn test_multiple_examples() {
        let raw = "\n * Thing.\n * @example\n * foo();\n * @example\n * bar();\n ";
        assert_eq!(
            clean_jsdoc(raw),
            "Thing.\n\n## Example\n\n```js\nfoo();\n```\n\n## Example\n\n```js\nbar();\n```"
        );
    }

    #[test]
    fn test_param_with_jsdoc_type() {
        assert_eq!(
            format_param("{string} name - the name"),
            "* `name` - the name"
        );
    }

    #[test]
    fn test_description_only() {
        let raw = "\n * Just a description with `inline code`.\n ";
        assert_eq!(clean_jsdoc(raw), "Just a description with `inline code`.");
    }

    #[test]
    fn test_example_between_tags() {
        let raw = "\n * Desc.\n * @example\n * code();\n * @returns result\n ";
        assert_eq!(
            clean_jsdoc(raw),
            "Desc.\n\n## Returns\n\nresult\n\n## Example\n\n```js\ncode();\n```"
        );
    }
}
