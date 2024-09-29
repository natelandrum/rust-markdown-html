use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use regex::Regex;

// Function to create the markdown to HTML mapping
fn create_markdown_map() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("#", "h");  // Header
    map.insert("**", "strong");  // Bold
    map.insert("*", "em");  // Italics
    map.insert(">", "blockquote");  // Blockquote
    map.insert("__", "ins");  // Underline
    map.insert("~~", "del");  // Strikethrough
    map.insert("`", "code");  // Code
    map.insert("```", "pre");  // Code block
    map
}

// Function to process a markdown line and convert to HTML
fn process_line(
    line: &str, // Current markdown line as a string
    map: &HashMap<&str, &str>, // Dictionary mapping markdown to HTML
    code: &mut bool, // Track if we're in a multiline code block
    in_table: &mut bool, // Track if we're in a table
    alignment: &mut Vec<String>, // Track alignment of table columns
    references: &HashMap<String, String>, // Reference-style links and images
    list_stack: &mut Vec<String>,  // Stack to track list types (ol/ul)
    current_list_depth: &mut usize,  // Track the nesting level of lists
) -> String {
    let mut html_line = String::new();

    if line.contains("\\") {
        // Handle escaped characters
        let escape_regex = Regex::new(r"\\(.)").unwrap();
        html_line = escape_regex.replace_all(line, "$1").to_string();
        return html_line;
}

    // Regular expression for detecting lists
    let ordered_list_regex = Regex::new(r"^(\s*)(\d+)\.\s").unwrap(); // Ordered list
    let unordered_list_regex = Regex::new(r"^(\s*)[-\*\+]\s").unwrap(); // Unordered list
    let task_list_checked_regex = Regex::new(r"^(\s*)-\s\[x\]\s").unwrap(); // Task list checked
    let task_list_unchecked_regex = Regex::new(r"^(\s*)-\s\[\s\]\s").unwrap(); // Task list unchecked

    if *in_table && !line.contains('|') {
        *in_table = false;
        html_line.push_str("</tbody></table>");
    }
    // Check if the line is empty
    if line.trim().is_empty() {
        // Close all open lists when an empty line is encountered
        close_open_lists(list_stack, &mut html_line);
        *current_list_depth = 0; // Reset nesting level
        // Add a <br> for empty lines
        if !*in_table {
            html_line.push_str("<br>");
        }
        return html_line;
    }

    // Determine indentation (for nesting purposes)
    let current_indentation = line.chars().take_while(|c| c.is_whitespace()).count();
    let depth = current_indentation / 4; // Assume 4 spaces or 1 tab per depth level

    // Close lists if indentation decreases
    if depth < *current_list_depth {
        // Close all lists that are deeper than the current depth
        while *current_list_depth > depth {
            if let Some(list_type) = list_stack.pop() {
                html_line.push_str(&format!("</{}>", list_type));
            }
            *current_list_depth -= 1;
        }
    }

    // Handle task lists (checked)
    if let Some(caps) = task_list_checked_regex.captures(line) {
        let list_depth = caps[1].len() / 4;
        let item_content = &line[caps[0].len()..];

        // Open new lists or close/open lists if the depth changes
        if list_depth > *current_list_depth {
            list_stack.push("ul".to_string());  // Task lists always use <ul>
            html_line.push_str("<ul>");
            *current_list_depth = list_depth;
        } else if *current_list_depth == list_depth && (list_stack.last() != Some(&"ul".to_string())) {
            if let Some(list_type) = list_stack.pop() {
                html_line.push_str(&format!("</{}>", list_type));
            }
            list_stack.push("ul".to_string());
            html_line.push_str("<ul>");
        }
        // Add task list item (checked)
        html_line.push_str(&format!(r#"<li><input type="checkbox" checked> {}</li>"#, item_content.trim()));
        return html_line;
    }

    // Handle task lists (unchecked)
    if let Some(caps) = task_list_unchecked_regex.captures(line) {
        let list_depth = caps[1].len() / 4;
        let item_content = &line[caps[0].len()..];

        // Open new lists or close/open lists if the depth changes
        if list_depth > *current_list_depth {
            list_stack.push("ul".to_string());  // Task lists always use <ul>
            html_line.push_str("<ul>");
            *current_list_depth = list_depth;
        } else if *current_list_depth == list_depth && (list_stack.last() != Some(&"ul".to_string())) {
            if let Some(list_type) = list_stack.pop() {
                html_line.push_str(&format!("</{}>", list_type));
            }
            list_stack.push("ul".to_string());
            html_line.push_str("<ul>");
        }
        // Add task list item (unchecked)
        html_line.push_str(&format!(r#"<li><input type="checkbox"> {}</li>"#, item_content.trim()));
        return html_line;
    }

    // Handle ordered lists
    if let Some(caps) = ordered_list_regex.captures(line) {
        let list_depth = caps[1].len() / 4;
        let item_content = &line[caps[0].len()..];

        // Open new lists or close/open lists if the depth changes
        if list_depth > *current_list_depth {
            list_stack.push("ol".to_string());
            html_line.push_str("<ol>");
            *current_list_depth = list_depth;
        } else if *current_list_depth == list_depth && (list_stack.last() != Some(&"ol".to_string())) {
            if let Some(list_type) = list_stack.pop() {
                html_line.push_str(&format!("</{}>", list_type));
            }
            list_stack.push("ol".to_string());
            html_line.push_str("<ol>");
        }
        // Add list item
        html_line.push_str(&format!("<li>{}</li>", item_content.trim()));
        return html_line;
    }

    // Handle unordered lists
    if let Some(caps) = unordered_list_regex.captures(line) {
        let list_depth = caps[1].len() / 4;
        let item_content = &line[caps[0].len()..];

        // Open new lists or close/open lists if the depth changes
        if list_depth > *current_list_depth {
            list_stack.push("ul".to_string());
            html_line.push_str("<ul>");
            *current_list_depth = list_depth;
        } else if *current_list_depth == list_depth && (list_stack.last() != Some(&"ul".to_string())) {
            if let Some(list_type) = list_stack.pop() {
                html_line.push_str(&format!("</{}>", list_type));
            }
            list_stack.push("ul".to_string());
            html_line.push_str("<ul>");
        }
        // Add list item
        html_line.push_str(&format!("<li>{}</li>", item_content.trim()));
        return html_line;
    }

    // If no list detected, close any open lists
    if !line.trim().is_empty() && !ordered_list_regex.is_match(line) && !unordered_list_regex.is_match(line) && !task_list_checked_regex.is_match(line) && !task_list_unchecked_regex.is_match(line) {
        close_open_lists(list_stack, &mut html_line);
        *current_list_depth = 0;
    }

    let mut html_line = String::from(line.trim());


    // Handle links (inline and reference-style)
    let link_regex = Regex::new(r"\[([^\]]+)\]\s*\(([^)]+)\)").unwrap(); // Inline links
    let ref_link_regex = Regex::new(r"\[([^\]]+)\]\[([^\]]+)\]").unwrap(); // Reference-style links

    if !html_line.trim_start().starts_with('!') {
        html_line = link_regex.replace_all(&html_line, r#"<a href="$2">$1</a>"#).to_string();
        html_line = ref_link_regex.replace_all(&html_line, |caps: &regex::Captures| {
            if let Some(url) = references.get(&caps[2]) {
                format!(r#"<a href="{}">{}</a>"#, url, &caps[1])
            } else {
                caps[0].to_string() // Leave it as is if reference not found
            }
        }).to_string();
    }

    // Handle images (inline and reference-style)
    let image_regex = Regex::new(r"!\[([^\]]*)\]\(([^)]+)\)").unwrap(); // Inline images
    let ref_image_regex = Regex::new(r"!\[([^\]]*)\]\[([^\]]+)\]").unwrap(); // Reference-style images

    html_line = image_regex.replace_all(&html_line, r#"<img alt="$1" src="$2" />"#).to_string();
    html_line = ref_image_regex.replace_all(&html_line, |caps: &regex::Captures| {
        if let Some(url) = references.get(&caps[2]) {
            format!(r#"<img alt="{}" src="{}" />"#, &caps[1], url)
        } else {
            caps[0].to_string()
        }
    }).to_string();

    // Handle reference lines
    for (key, value) in references.iter() {
        let escaped_value = regex::escape(&value);
        let ref_regex = Regex::new(&format!(r"\[{}\]:\s*{}", key, escaped_value)).unwrap();
        html_line = ref_regex.replace_all(&html_line, "").to_string();
    }

    // Handle horizontal rules
    if line == "---" || line == "***" || line == "___" {
        return "<hr />".to_string();
    }

    // Handle table headers and rows
    if line.contains('|') && !*in_table {
        *in_table = true;
        html_line = String::from("<table><thead><tr>");

        let columns: Vec<&str> = line.split('|')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        for column in &columns {
            html_line.push_str(&format!("<th>{}</th>", column));
        }

        html_line.push_str("</tr></thead><tbody>");
        return html_line;
    }

    // Handle table alignment
    if *in_table && line.contains("---") {
        *alignment = line.split('|')
            .filter(|s| !s.is_empty())
            .map(|col| {
                if col.trim().starts_with(':') && col.trim().ends_with(':') {
                    "center".to_string()
                } else if col.trim().starts_with(':') {
                    "left".to_string()
                } else if col.trim().ends_with(':') {
                    "right".to_string()
                } else {
                    "left".to_string()
                }
            })
            .collect();
        return String::new(); // Skip this row in the output
    }

    // If we're in a table but there's no more table content, close the table
    if *in_table && !line.contains('|') {
        *in_table = false;
        return String::from("</tbody></table>");
    }

    // Process table rows
    if *in_table && !line.contains("---") {
        html_line = String::from("<tr>");
        let columns: Vec<&str> = line.split('|')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        for (i, column) in columns.iter().enumerate() {
            let align = if i < alignment.len() {
                &alignment[i]
            } else {
                ""
            };
            html_line.push_str(&format!("<td style=\"text-align: {}\">{}</td>", align, column));
        }

        html_line.push_str("</tr>");
        return html_line;
    }

    // Handle general markdown elements like headers, blockquotes, and text formatting
    let re = Regex::new(r"(\*\*|\*|`{1,3}|#{1,6}|>{2,}|^>|__|~~)").unwrap();
    let words: Vec<&str> = re
        .find_iter(line)
        .map(|mat| mat.as_str())
        .filter(|word| map.contains_key(word) || word.starts_with('>') || word.starts_with('#'))
        .collect();


    // Iterate over each word in the line
    for word in words {
        if let Some(&html_tag) = map.get(word)
        .or_else
        (|| {
            if word.starts_with('>') {
                map.get(">")
            } else {
                None
            }
        })
        .or_else
        (|| {
            if word.starts_with('#') {
                map.get("#")
            } else {
                None
            }
            }
        ) {
            let count = line.matches(word).count();

            if count % 2 == 0 {
                html_line = html_line.replacen(word, &format!("<{}>", html_tag), 1);
                html_line = html_line.replacen(word, &format!("</{}>", html_tag), 1);
            } else {
                match word {
                    word if word.starts_with('>') => {
                        if word.contains('>') {
                            let emsp_count = word.len() - 1;
                            let emsp_str = "&emsp;".repeat(emsp_count);
                            html_line = html_line.replacen(word, &format!("<{}> {}", html_tag, emsp_str), 1);
                            html_line.push_str(&format!("</{}>", html_tag));
                        }
                    }
                    "```" => {
                        if *code {
                            html_line = html_line.replacen(word, &format!("</{}>", html_tag), 1);
                            *code = false;
                        } else {
                            html_line = html_line.replacen(word, &format!("<{}>", html_tag), 1);
                            *code = true;
                        }
                    }
                    word if word.starts_with('#') => {
                        let heading_level = word.matches('#').count();
                        html_line = html_line.replacen(word, &format!("<{}{}>", html_tag, heading_level), 1);
                        html_line.push_str(&format!("</{}{}>", html_tag, heading_level));
                    }
                    _ => {
                        html_line = html_line.replacen(word, &format!("<{}>", html_tag), 1);
                        html_line.push_str(&format!("</{}>", html_tag));
                    }
                }
            }
        }
    }

    html_line
}

// Function to close open lists
fn close_open_lists(stack: &mut Vec<String>, html_line: &mut String) {
    while let Some(list_type) = stack.pop() {
        html_line.push_str(&format!("</{}>", list_type));
    }
}

// Function to collect reference-style links and images
fn collect_references(input_file: &str) -> io::Result<HashMap<String, String>> {
    let mut references = HashMap::new();
    let input_path = Path::new(input_file);
    let file = File::open(&input_path)?;

    let ref_regex = Regex::new(r"\[([^\]]+)\]:\s*(.+)").unwrap(); // Regex for capturing references

    for line in io::BufReader::new(file).lines() {
        let line = line?;  // Handle potential errors

        if let Some(caps) = ref_regex.captures(&line) {
            references.insert(caps[1].to_string(), caps[2].to_string());
        }
    }

    Ok(references)
}



// Function to read markdown file and convert to HTML
fn markdown_to_html(input_file: &str, output_file: &str) -> io::Result<()> {
    let mut code = false;
    let mut in_table = false;
    let mut alignment: Vec<String> = Vec::new();
    let markdown_map = create_markdown_map();

    let mut list_stack: Vec<String> = Vec::new(); // Stack to track list types
    let mut current_list_depth = 0;  // Track the current depth of nested lists

    // First, collect references from the markdown file
    let references = collect_references(input_file)?;

    // Open the input file
    let input_path = Path::new(input_file);
    let file = File::open(&input_path)?;

    // Create the output file
    let mut output = File::create(output_file)?;

    // Process each line of the markdown file
    for line in io::BufReader::new(file).lines() {
        let line = line?;  // Handle potential errors
        let html_line = process_line(
            &line, 
            &markdown_map, 
            &mut code, 
            &mut in_table, 
            &mut alignment, 
            &references,
            &mut list_stack,  // Pass the list stack
            &mut current_list_depth  // Pass the current list depth
        );
        writeln!(output, "{}", html_line)?;  // Write HTML to the output file
    }

    Ok(())
}

fn main() {
    // Define your input and output file paths
    let input_file = "input.md";
    let output_file = "output.html";

    // Call the function to convert markdown to HTML
    match markdown_to_html(input_file, output_file) {
        Ok(_) => println!("Conversion complete!"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
