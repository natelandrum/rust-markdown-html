# Overview

In this project, I developed a Rust program to convert Markdown text into HTML. The goal of this project was to deepen my understanding of the Rust programming language while working with regular expressions, file I/O, and string manipulation. As a software engineer, I'm constantly exploring new languages to expand my skill set, and Rust's focus on safety and performance makes it an excellent choice for tasks like parsing and transforming data.

The software processes Markdown files, recognizing elements like headers, bold text, italics, lists, tables, links, and more. It converts these elements into their corresponding HTML representations. I chose this project to demonstrate Rust's strengths in handling complex text transformations efficiently.

One of the future goals for this project is to improve the modularity of the code, breaking down large functions into smaller, more manageable pieces, making the codebase cleaner and easier to maintain.

[Software Demo Video](https://youtu.be/ZzI1AgkFOBo)

# Development Environment

- **Development Tools**: 
  - Visual Studio Code: For code editing, with Rust extensions for syntax highlighting and debugging.
  - Rust CLI (Cargo): For building and running the code.
  
- **Programming Language**: Rust

- **Libraries Used**:
  - `regex`: For processing and matching Markdown patterns.
  - `std::io`: For handling file reading and writing.

# Useful Websites

- [Rust Documentation](https://doc.rust-lang.org) - Official documentation for the Rust programming language.
- [Rust Regex Documentation](https://docs.rs/regex/latest/regex/) - Documentation for the regex crate in Rust.
- [Rust Tutorial](https://www.tutorialspoint.com/rust/index.htm) - A comprehensive Rust tutorial.
- [Rust Ownership](https://medium.com/@TechSavvyScribe/ownership-and-borrowing-in-rust-a-comprehensive-guide-1400d2bae02a) - Further information for Rust ownership and borrowing.
- [Markdown Cheat Sheet](https://www.interviewbit.com/markdown-cheat-sheet/) - A cheat sheet with all the most used markdown syntax.

# Future Work

- Refactor the code to make it more modular by separating concerns into distinct functions.
- Add support for additional Markdown elements, such as footnotes and tables of contents.
- Improve error handling and validation to ensure more robust parsing of malformed Markdown files.
