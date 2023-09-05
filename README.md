# Moon Phase Demo 🌕🌖🌗🌘🌑

## Overview

The Moon Phase Demo is a web development project designed to showcase the power of modern web technologies like HTMX and Rust. This project serves as a playground for sharpening skills with these new tools, offering a practical example of how they can be used in tandem to create a dynamic and efficient web application.

## Table of Contents

- [Overview](#overview)
- [Technologies](#technologies)
- [Getting Started](#getting-started)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Technologies

### Rust 🦀

- **File**: [`Cargo.toml`](https://github.com/donedgardo/moon_phase_demo/blob/main/Cargo.toml), [`src/lib.rs`](https://github.com/donedgardo/moon_phase_demo/blob/main/src/lib.rs), [`src/main.rs`](https://github.com/donedgardo/moon_phase_demo/blob/main/src/main.rs)
- **Purpose**: Rust is used for the backend logic, offering memory safety and blazing speed. The project uses Rust's package manager, Cargo, to manage dependencies and build the project.

### HTMX 🌐

- **File**: [`src/input.css`](https://github.com/donedgardo/moon_phase_demo/blob/main/src/input.css), [`static/output.css`](https://github.com/donedgardo/moon_phase_demo/blob/main/static/output.css)
- **Purpose**: HTMX allows you to access AJAX, CSS Transitions, WebSockets, and Server Sent Events directly in HTML, without requiring any JavaScript. It's used here to create a dynamic and interactive frontend.

### Tailwind CSS 💨

- **File**: [`tailwind.config.js`](https://github.com/donedgardo/moon_phase_demo/blob/main/tailwind.config.js), [`package.json`](https://github.com/donedgardo/moon_phase_demo/blob/main/package.json)
- **Purpose**: Tailwind CSS is a utility-first CSS framework for rapidly building custom user interfaces. It's configured via `tailwind.config.js` and managed through npm as seen in `package.json`.

### CSS Builder 🎨

- **File**: [`src/bin/build-css.rs`](https://github.com/donedgardo/moon_phase_demo/blob/main/src/bin/build-css.rs)
- **Purpose**: A custom Rust script to build and optimize the CSS for the project.

## Getting Started

1. **Clone the Repository**
    ```bash
    git clone https://github.com/donedgardo/moon_phase_demo.git
    ```

2. **Install Rust**
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

3. **Install Node.js and npm**
    ```bash
    # Using nvm
    nvm install node
    ```

4. **Install Dependencies**
    ```bash
    cargo install
    npm install
    ```

5. **Run the Project**
    ```bash
    cargo run
    ```

## Usage

To view the moon phases, simply navigate to `http://localhost:8000/`.

## Contributing

Feel free to contribute to this project. Fork it, create a new branch, commit your changes, and create a pull request.

## License

This project is licensed under the MIT License.

---

Created by [Edgardo Carreras](https://github.com/donedgardo) to explore and demonstrate the capabilities of HTMX and Rust in modern web development.