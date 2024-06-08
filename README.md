
# Scrutator

Scrutator is an advanced file searching tool developed in Rust. It provides exceptional performance improvements over traditional file search methods by leveraging the power of hash maps for fast file storage and retrieval.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)
- [Benchmark](#benchmark)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

## Features

- **Blazing Fast Search**: Achieves up to 105,000 times faster searches compared to standard Windows search.
- **Efficient Caching**: Uses hash map caching for significantly improved performance on subsequent searches.
- **Rust-Powered**: Built with Rust for enhanced safety, concurrency, and performance.
- **Cross-Platform**: Works seamlessly on various operating systems supported by Rust.
- **Customizable**: Configurable settings to fine-tune search parameters according to user needs.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (ensure it is installed and updated)

### Steps

1. Clone the repository:
    ```sh
    git clone https://github.com/ashborn3/scrutator.git
    cd scrutator
    ```

2. Build the project in release mode:
    ```sh
    cargo build --release
    ```

## Usage

### Basic Search

Run Scrutator with a search query:

```sh
./target/release/scrutator <search_query>
```

Replace `<search_query>` with the term you are searching for. Scrutator will output the paths of matching files.

### Advanced Search Options

Scrutator supports various command-line options to customize the search behavior. For detailed usage instructions, run:

```sh
./target/release/scrutator --help
```

## Configuration

Scrutator can be configured using a configuration file to set default search parameters, caching preferences, and more. Place the configuration file in the root directory of the project. A sample configuration file (`config.toml`) is provided:

```toml
# config.toml
[search]
ignore_case = true
max_results = 100

[cache]
enabled = true
path = "cache/scrutator_cache"
```

## Benchmark

On the developer's benchmark tests, Scrutator has shown to be approximately 105,000 times faster than the native Windows search utility, making it an ideal tool for users requiring rapid and repeated file searches.

## Contributing

Contributions are highly encouraged and appreciated. To contribute:

1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/YourFeature`).
3. Commit your changes (`git commit -m 'Add some feature'`).
4. Push to the branch (`git push origin feature/YourFeature`).
5. Open a pull request.

Please ensure your code follows Rust’s coding standards and includes relevant tests.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Contact

For any issues or feature requests, please open an [issue](https://github.com/ashborn3/scrutator/issues) on GitHub.

---
