<p align="center">
  <a href="" rel="noopener">
 <img src="https://i.imgur.com/On9HPQN.jpeg" alt="Project Safe Math for Rust logo" width="300" height="300"></a>
</p>

<h3 align="center">Project Safe Math for Rust</h3>

<div align="center">

[![Status](https://img.shields.io/badge/status-active-success.svg)]()
[![GitHub Issues](https://img.shields.io/badge/issues-0%20open-red)](https://github.com/malledugean/rust_safe_math/issues)
[![GitHub Pull Requests](https://img.shields.io/badge/pull%20requests-0%20pull-yellow)](https://github.com/malledugean/rust_safe_math/pulls)
[![License](https://img.shields.io/badge/license-CC--BY--4.0-blue) ](#license)

</div>

---

<p align="center"> A Rust library for simplified safe math operations.
    <br> 
</p>

## 📝 Table of Contents

-   [About](#about)
-   [Getting Started](#getting_started)
-   [Deployment](#deployment)
-   [Usage](#usage)
-   [Built Using](#built_using)
-   [License](#license)
-   [Authors](#authors)
-   [Acknowledgments](#acknowledgement)

## 🧐 About <a name = "about"></a>

Safe Math Rust is a Rust library designed to simplify and ensure safe mathematical operations. It provides functions that perform common arithmetic operations while preventing potential overflows and underflows that can lead to program crashes or unexpected behavior.

## 🏁 Getting Started <a name = "getting_started"></a>

This guide will help you get started using Safe Math Rust in your projects.

### Prerequisites

-   Rust compiler: Download and install Rust from the official website https://www.rust-lang.org/tools/install.
-   Basic understanding of Rust: Familiarity with core Rust concepts like functions, variables, and data types is recommended.

### Installing

Add Safe Math Rust as a dependency in your Cargo.toml file:

content_copy

1. Run cargo update to install the library.

```
Ini, TOML
[dependencies]
dicoco_safe_math = "0.1.0"
```

Use code with caution.

2. Run cargo update to install the library.

```
cargo update
```

## 🔧 Running the tests <a name = "tests"></a>

The library includes unit tests to ensure the functionality of its functions. To run the tests, navigate to your project directory in the terminal and execute:

```
Bash
cargo test
```

## 🎈 Usage <a name="usage"></a>

Safe Math Rust offers functions for various mathematical operations, all designed to handle potential overflows and underflows gracefully. Here are some examples:

```
use dicoco_safe_math::calc_basic;

fn main() {
  let a = 100;
  let b = 50;

  // Safe addition
  let safe_sum = calc_basic::sub_x_y(a,b);

  // Safe subtraction
  let safe_difference = calc_basic::sum_x_y(a,b);


  println!("Safe sum: {}", safe_sum);
  println!("Safe difference: {}", safe_difference);
}
```

## 🚀 Deployment <a name = "deployment"></a>

Once you have integrated Safe Math Rust into your project and are satisfied with its functionality, you can deploy your application following your preferred method for Rust projects. Common options include building standalone executables or deploying as a web service.

## ⛏️ Built Using <a name = "built_using"></a>

-   [Rust](https://www.rust-lang.org) - Rust

## 📜 License: <a name = "license"></a>

This project is licensed under the license Creative Commons Attribution 4.0 International (CC-BY-4.0).

## ✍️ Authors <a name = "authors"></a>

-   [@malledugean](https://github.com/malledugean) - Idea & Initial work

## 🎉 Acknowledgements <a name = "acknowledgement"></a>

-   NearX Rust learning
