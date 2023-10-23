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

2. Build and run the project:

   ```bash
   cargo run
   ```

## Acknowledgments

- Inspired by the [browser engine](https://limpet.net/mbrubeck/2014/08/08/toy-layout-engine-1.html) series by Matt Brubeck.

- Inspired by "The Hard Parts of UI Development" course by Will Sentance on [Frontend Masters](https://frontendmasters.com/courses/hard-parts-ui-dev/).

- [HTML specification](https://html.spec.whatwg.org/multipage/introduction.html)

- [W3C CSS ](https://www.w3.org/TR/CSS/#css)

- [rust-cssparser](https://github.com/servo/rust-cssparser)
