//! A driver for generic GPIO driven seven segment LED display
//!
//! This driver was built using [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal/~0.2
//!
//! # Examples
//!
//! You should find at least one example in the [nucleo-f042k6] crate.
//!
//! [nucleo-f042k6]: https://github.com/therealprof/nucleo-f042k6

#![deny(warnings)]
#![no_std]

extern crate embedded_hal as hal;

use hal::digital::OutputPin;

/// A structure representing the 7 segments of a 7-segment display
pub struct SevenSeg<A, B, C, D, E, F, G> {
    seg_a: A,
    seg_b: B,
    seg_c: C,
    seg_d: D,
    seg_e: E,
    seg_f: F,
    seg_g: G,
}

impl<A, B, C, D, E, F, G> SevenSeg<A, B, C, D, E, F, G>
where
    A: OutputPin,
    B: OutputPin,
    C: OutputPin,
    D: OutputPin,
    E: OutputPin,
    F: OutputPin,
    G: OutputPin,
{
    /// Create a new 7-segment display structure by passing in 7 GPIOs implementing the
    /// `OutputPin` trait for the segments `a`, `b`, `c`, `d`, `e` and `f` respectively
    pub fn new(seg_a: A, seg_b: B, seg_c: C, seg_d: D, seg_e: E, seg_f: F, seg_g: G) -> Self {
        Self {
            seg_a,
            seg_b,
            seg_c,
            seg_d,
            seg_e,
            seg_f,
            seg_g,
        }
    }

    /// Release the 7 GPIOs previously occupied by the 7-segment display
    pub fn release(self) -> (A, B, C, D, E, F, G) {
        (
            self.seg_a, self.seg_b, self.seg_c, self.seg_d, self.seg_e, self.seg_f, self.seg_g,
        )
    }

    /// Disable the 7-segment display by pulling all GPIOs low
    pub fn clear(&mut self) {
        self.seg_a(false);
        self.seg_b(false);
        self.seg_c(false);
        self.seg_d(false);
        self.seg_e(false);
        self.seg_f(false);
        self.seg_g(false);
    }

    /// Enable or disable segment `a` according to the `state`
    pub fn seg_a(&mut self, state: bool) {
        if state {
            self.seg_a.set_high();
        } else {
            self.seg_a.set_low();
        }
    }

    /// Enable or disable segment `b` according to the `state`
    pub fn seg_b(&mut self, state: bool) {
        if state {
            self.seg_b.set_high();
        } else {
            self.seg_b.set_low();
        }
    }

    /// Enable or disable segment `c` according to the `state`
    pub fn seg_c(&mut self, state: bool) {
        if state {
            self.seg_c.set_high();
        } else {
            self.seg_c.set_low();
        }
    }

    /// Enable or disable segment `d` according to the `state`
    pub fn seg_d(&mut self, state: bool) {
        if state {
            self.seg_d.set_high();
        } else {
            self.seg_d.set_low();
        }
    }

    /// Enable or disable segment `e` according to the `state`
    pub fn seg_e(&mut self, state: bool) {
        if state {
            self.seg_e.set_high();
        } else {
            self.seg_e.set_low();
        }
    }

    /// Enable or disable segment `f` according to the `state`
    pub fn seg_f(&mut self, state: bool) {
        if state {
            self.seg_f.set_high();
        } else {
            self.seg_f.set_low();
        }
    }

    /// Enable or disable segment `g` according to the `state`
    pub fn seg_g(&mut self, state: bool) {
        if state {
            self.seg_g.set_high();
        } else {
            self.seg_g.set_low();
        }
    }

    /// Display the digit specified in `num`. Supported are all values in the hexadecimal system,
    /// that is `0` through `9` and `A` through `F`. Any other value will turn off the display.
    pub fn display(&mut self, num: u8) {
        match num {
            0 => {
                self.seg_a(true);
                self.seg_b(true);
                self.seg_c(true);
                self.seg_d(true);
                self.seg_e(true);
                self.seg_f(true);
                self.seg_g(false);
            }
            1 => {
                self.seg_a(false);
                self.seg_b(false);
                self.seg_c(false);
                self.seg_d(false);
                self.seg_e(true);
                self.seg_f(true);
                self.seg_g(false);
            }
            2 => {
                self.seg_a(true);
                self.seg_b(true);
                self.seg_c(false);
                self.seg_d(true);
                self.seg_e(true);
                self.seg_f(false);
                self.seg_g(true);
            }
            3 => {
                self.seg_a(true);
                self.seg_b(false);
                self.seg_c(false);
                self.seg_d(true);
                self.seg_e(true);
                self.seg_f(true);
                self.seg_g(true);
            }
            4 => {
                self.seg_a(false);
                self.seg_b(false);
                self.seg_c(true);
                self.seg_d(false);
                self.seg_e(true);
                self.seg_f(true);
                self.seg_g(true);
            }
            5 => {
                self.seg_a(true);
                self.seg_b(false);
                self.seg_c(true);
                self.seg_d(true);
                self.seg_e(false);
                self.seg_f(true);
                self.seg_g(true);
            }
            6 => {
                self.seg_a(true);
                self.seg_b(true);
                self.seg_c(true);
                self.seg_d(true);
                self.seg_e(false);
                self.seg_f(true);
                self.seg_g(true);
            }
            7 => {
                self.seg_a(false);
                self.seg_b(false);
                self.seg_c(false);
                self.seg_d(true);
                self.seg_e(true);
                self.seg_f(true);
                self.seg_g(false);
            }
            8 => {
                self.seg_a(true);
                self.seg_b(true);
                self.seg_c(true);
                self.seg_d(true);
                self.seg_e(true);
                self.seg_f(true);
                self.seg_g(true);
            }
            9 => {
                self.seg_a(true);
                self.seg_b(false);
                self.seg_c(true);
                self.seg_d(true);
                self.seg_e(true);
                self.seg_f(true);
                self.seg_g(true);
            }
            10 => {
                self.seg_a(false);
                self.seg_b(true);
                self.seg_c(true);
                self.seg_d(true);
                self.seg_e(true);
                self.seg_f(true);
                self.seg_g(true);
            }
            11 => {
                self.seg_a(true);
                self.seg_b(true);
                self.seg_c(true);
                self.seg_d(false);
                self.seg_e(false);
                self.seg_f(true);
                self.seg_g(true);
            }
            12 => {
                self.seg_a(true);
                self.seg_b(true);
                self.seg_c(true);
                self.seg_d(true);
                self.seg_e(false);
                self.seg_f(false);
                self.seg_g(false);
            }
            13 => {
                self.seg_a(true);
                self.seg_b(true);
                self.seg_c(false);
                self.seg_d(false);
                self.seg_e(true);
                self.seg_f(true);
                self.seg_g(true);
            }
            14 => {
                self.seg_a(true);
                self.seg_b(true);
                self.seg_c(true);
                self.seg_d(true);
                self.seg_e(false);
                self.seg_f(false);
                self.seg_g(true);
            }
            15 => {
                self.seg_a(false);
                self.seg_b(true);
                self.seg_c(true);
                self.seg_d(true);
                self.seg_e(false);
                self.seg_f(false);
                self.seg_g(true);
            }
            _ => {
                self.clear()
            }
        }
    }
}
