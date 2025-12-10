//! Math module - mathematical functions and constants
//!
//! This module provides trigonometric, logarithmic, and other mathematical functions.
//! Most functions are already registered in stdlib/mod.rs.

use std::f64::consts;

/// PI constant
pub const PI: f64 = consts::PI;

/// E constant (base of natural logarithm)
pub const E: f64 = consts::E;

/// Compute sine of a number (in radians)
pub fn sin(x: f64) -> f64 {
    x.sin()
}

/// Compute cosine of a number (in radians)
pub fn cos(x: f64) -> f64 {
    x.cos()
}

/// Compute tangent of a number (in radians)
pub fn tan(x: f64) -> f64 {
    x.tan()
}

/// Compute arcsine
pub fn asin(x: f64) -> f64 {
    x.asin()
}

/// Compute arccosine
pub fn acos(x: f64) -> f64 {
    x.acos()
}

/// Compute arctangent
pub fn atan(x: f64) -> f64 {
    x.atan()
}

/// Compute natural logarithm
pub fn ln(x: f64) -> f64 {
    x.ln()
}

/// Compute base-10 logarithm
pub fn log10(x: f64) -> f64 {
    x.log10()
}

/// Compute logarithm with arbitrary base
pub fn log(x: f64, base: f64) -> f64 {
    x.log(base)
}

/// Compute square root
pub fn sqrt(x: f64) -> f64 {
    x.sqrt()
}

/// Compute power (x^y)
pub fn pow(x: f64, y: f64) -> f64 {
    x.powf(y)
}

/// Compute exponential (e^x)
pub fn exp(x: f64) -> f64 {
    x.exp()
}

/// Compute ceiling (round up)
pub fn ceil(x: f64) -> i64 {
    x.ceil() as i64
}

/// Compute floor (round down)
pub fn floor(x: f64) -> i64 {
    x.floor() as i64
}

/// Round to nearest integer
pub fn round(x: f64) -> i64 {
    x.round() as i64
}

/// Get the absolute value
pub fn abs(x: f64) -> f64 {
    x.abs()
}

/// Get minimum of two numbers
pub fn min(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

/// Get maximum of two numbers
pub fn max(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}
