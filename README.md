# Bowling Score

![License](https://img.shields.io/github/license/zS1L3NT/rs-bowling-score?style=for-the-badge) ![Languages](https://img.shields.io/github/languages/count/zS1L3NT/rs-bowling-score?style=for-the-badge) ![Top Language](https://img.shields.io/github/languages/top/zS1L3NT/rs-bowling-score?style=for-the-badge) ![Commit Activity](https://img.shields.io/github/commit-activity/y/zS1L3NT/rs-bowling-score?style=for-the-badge) ![Last commit](https://img.shields.io/github/last-commit/zS1L3NT/rs-bowling-score?style=for-the-badge)

This is a script to calculate someone's bowling score. It will repeatedly ask you how many pins you hit down for each throw of the ball you make. This script will then generate a table to display your score after every throw.
![Example](https://i.ibb.co/V349drv/bowling.png)

## Motivation

Bowling is one of the rare sports that I enjoy playing. I have always taken interest in the scoreboard and how bowling scores are calculated. I build this script in Rust because I wanted to try it out in a smallw project.

## Features

- Calculate bowling scores after every throw
- Error handling for Invalid input
  - Handles invalid pin count for multiple throws<br>
    ![Error 1](https://i.ibb.co/5LLGpy0/bowling-error-1.png)
  - Handles invalid pin count for first throw<br>
    ![Error 2](https://i.ibb.co/34fkG4b/bowling-error-2.png)
  - Handles non-number errors<br>
    ![Error 3](https://i.ibb.co/ChQVhMj/bowling-error-3.png)

## Usage

```
$ cargo run src/main.rs
```

## Built with

- Rust
  - [![prettytable-rs](https://img.shields.io/badge/prettytable--rs-%5E0.8-blue?style=flat-square)](https://docs.rs/prettytable-rs/0.8)
