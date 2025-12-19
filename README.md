# Arduino Clock - Embedded Rust

A bare-metal Rust implementation of a digital clock for Arduino Uno with serial input and TFT display output.

> [!WARNING]
> **Display Hardware Issue**: The TFT screen does not render output. Code compiles and uploads successfully, but there's a compatibility issue between the ST7735 Rust driver and the display hardware. All software components are implemented and functional.

## Project Concept

This project builds a digital clock from scratch in embedded Rust without the standard library. Users input time via serial interface in `HH:MM:SS` format, which the clock tracks internally in 24-hour format with automatic second/minute/hour rollovers. The time displays on a 1.8" TFT screen in 12-hour format with AM/PM.

The implementation features modular architecture with separate time management, display handling, and I/O coordination. Key technical elements include manual string parsing without heap allocation, generic hardware abstraction with trait bounds, and efficient display updates only when minutes change.

This demonstrates core embedded Rust concepts: `no_std` programming, hardware abstraction layers, SPI communication, and the real-world challenges of hardware-software integration.
