# br-ow-ser

## Overview

Welcome to `br-ow-ser`, a toy browser project created for learning purposes. This project is designed to help me explore the fundamentals of web browsers, including HTML parsing, CSS parsing, a layout engine, render tree, a painting engine and more. The goal is to provide a simplified implementation to aid in understanding the core concepts behind browsers.

## Disclaimer

This project is not intended for production use. It's a study project meant to deepen your understanding of browser internals.

This project intentionally omits considerations for real-world complexities, such as:

- Practical Usability
- Adherence to Industry Standards
- Performance Optimization
- Interoperability Concerns

This intentional exclusion allows for a focused exploration of fundamental concepts without the constraints imposed by real-world implementation challenges.

## Features

- [x] **HTML Parsing:** Converts HTML strings into a Document Object Model (DOM) tree. This parser works for a few subset of HTML syntax:

  - Balanced tags;
  - Attributes with quoted values;
  - Text nodes;
  - Comments;
  - Processing Instruction;
  - Error handling (Non-well-formed markup).

- [x] **CSS Parsing:** Parses CSS strings to handle styling and layout. This parser works for a few subset of CSS syntax:
  - Selector (Simple Selector);
- [x] **Render Tree:** Combines information from the DOM tree and layout engine for rendering.
- [x] **Layout Engine:** Determines the size and position of each element on the page.
- [x] **Painting Engine:** Draws the render tree on the screen.
- [ ] **JavaScript Engine:** Comprising parsers, interpreters, and compilers.

## Installation

Ensure you have Rust installed on your machine. If not, you can install it by following the instructions at [Rust Installation Guide](https://www.rust-lang.org/tools/install).

1. Clone the repository:

   ```bash
   git clone https://github.com/andraderaul/br-ow-ser.git
   cd br-ow-ser
   ```

2. Building and Running with Optimizations:

   - For optimized builds, use `cargo build --release` to build and `cargo run --release` to run.

3. Customizing Input Files:

   - By default, br-ow-ser loads `test.html` and `test.css` from the examples directory.
   - To specify custom HTML and CSS files, use the `--html` and `--css` arguments:

   ```bash
     ./target/debug/br-ow-ser --html examples/custom.html --css examples/custom.css
   ```

4. Output Configuration:

   - The rendered page is saved as `output.png` by default.
   - To change the output filename, use the `-o` option:

   ```bash
     ./target/debug/br-ow-ser --html examples/test.html --css examples/test.css -o custom_output.png
   ```

   - To switch to PDF output, add `--format pdf`:

   ```bash
     ./target/debug/br-ow-ser --html examples/test.html --css examples/test.css -o custom_output.pdf --format pdf
   ```

## What I learned in this journey

In this journey, I delved into several key aspects:

1. **Rust Programming:** Explored Rust, leveraging its performance and safety features.

2. **Web Development Basics:** Covered core concepts like HTML, CSS, and the Document Object Model (DOM).

3. **Toy Rendering Engine Features:**

   - HTML and CSS Parsing
   - Layout Engines
   - Render Trees
   - Painting Engines

4. **HTML Parser in Rust:** Developed an HTML parser in Rust, enabling the processing of HTML documents.

5. **CSS Parser in Rust:** Developed a CSS parser in Rust, including rules, selectors, and unit conversions.

6. **Styling the DOM:** Explored styling elements in the DOM using specified CSS values, considering selector matching and specificity.

7. **Unit Conversions:** Extended unit conversions to support various units such as pixels, percentages, em, and rem.

8. **Testing:** Emphasized the importance of testing, creating tests for different value types and scenarios.

## Acknowledgments

- Inspired by the [browser engine](https://limpet.net/mbrubeck/2014/08/08/toy-layout-engine-1.html) series by Matt Brubeck.

- Inspired by "The Hard Parts of UI Development" course by Will Sentance on [Frontend Masters](https://frontendmasters.com/courses/hard-parts-ui-dev/).

- [HTML specification](https://html.spec.whatwg.org/multipage/introduction.html)

- [W3C CSS ](https://www.w3.org/TR/CSS/#css)

- [rust-cssparser](https://github.com/servo/rust-cssparser)
