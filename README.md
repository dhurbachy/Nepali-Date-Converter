---

# Nepali-Date-Converter

A fast and reliable **Bikram Sambat (BS) ↔ Gregorian (AD)** date converter written in Rust. This library provides accurate Nepali date conversion using embedded calendar data, requiring zero setup from the user.

---

## Features

* **High Performance**: Optimized date arithmetic with zero-cost abstractions.
* **Embedded Data**: No need to manage external JSON files; the calendar data is compiled into your binary.
* **Simple API**: Convert dates with a single one-liner function call.

---

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
nepali-date-converter = "0.1.0"
```

---

## Usage

You can convert an AD date string directly into a Nepali date struct without any manual initialization:

```rust
use nepali_date_converter;

fn main() {
    // Simply pass an AD date string in YYYY-MM-DD format
    let ad_date = "2026-02-20";

    match nepali_date_converter::ad_to_bs(ad_date) {
        Some(date) => {
             // 1. Direct Field Access
            // Great for logic, calculations, or manual formatting
            println!("--- Raw Data ---");
            println!("Nepali Year: {}", date.year);   // 2082
            println!("Nepali Month: {}", date.month); // 11
            println!("Nepali Day: {}", date.day);     // 13
            

            // Best for displaying dates to users in different styles
            println!("\n--- Formatted Output ---");
            println!("Standard:  {}", date.format("YYYY-MM-DD"));
            println!("Long Form: {}", date.format("DD MMMM, YYYY"));
            println!("Month Only: {}", date.format("MMMM"));
            println!("Full Date: {}", date.format("EEEE, DD MMMM YYYY"));
            println!("Nepali Day: {}", date.format("NNNN")); 
            
              // Useful if you need specific padding (like 08 instead of 8)
            println!("\n--- Manual Format ---");
            println!(
                "Converted Date: {}-{:02}-{:02}",
                date.year, date.month, date.day
            );
        },
        None => println!("The date is invalid or out of the supported range."),
    }
}
```

---

## Supported Range

The current version supports conversions starting from:

* **BS Epoch**: Baishakh 1, 1975
* **AD Epoch**: April 13, 1918

It supports all years included in your `calendar_data.json` starting from the BS year 1975.

---

## License

This project is licensed under the MIT License.

---

## Author

**Dhrub Chaudhary**
📧 [dhurbac66@gmail.com](mailto:dhurbac66@gmail.com)

---