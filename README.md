# Vera

Vera is an open-source bot developed by aero.nu. It uses a Large Language Model (LLM) to detect harmful content in various categories. This project is written in Rust and can be run with `cargo`.

## Features

Vera can detect the following categories of content:

- **Hate:** Content that expresses, incites, or promotes hate based on race, gender, ethnicity, religion, nationality, sexual orientation, disability status, or caste. Hate directed at non-protected groups (e.g., chess players) is considered harassment.
- **Hate/Threatening:** Hateful content that includes violence or serious harm towards the targeted group.
- **Harassment:** Content that expresses, incites, or promotes harassing language towards any target.
- **Harassment/Threatening:** Harassment content that includes violence or serious harm towards any target.
- **Self-Harm:** Content that promotes, encourages, or depicts acts of self-harm, such as suicide, cutting, and eating disorders.
- **Self-Harm/Intent:** Content where the speaker expresses that they are engaging or intend to engage in acts of self-harm.
- **Self-Harm/Instructions:** Content that encourages performing acts of self-harm or provides instructions on how to commit such acts.
- **Sexual:** Content meant to arouse sexual excitement, describe sexual activity, or promote sexual services (excluding sex education and wellness).
- **Sexual/Minors:** Sexual content involving individuals under 18 years old.
- **Violence:** Content depicting death, violence, or physical injury.
- **Violence/Graphic:** Content depicting death, violence, or physical injury in graphic detail.

## Getting Started

### Prerequisites

- Rust: Make sure you have Rust installed. You can download it from [here](https://www.rust-lang.org/tools/install).

### Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/withaero/vera
    cd vera
    ```

2. Copy the `.env.example` file to `.env`:
    ```sh
    cp .env.example .env
    ```

3. Edit the `.env` file with your configuration settings.

### Running Vera

Run the following command to start Vera in release mode:
```sh
cargo run --release
```
