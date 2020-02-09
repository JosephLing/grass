use std::collections::BTreeMap;
use std::convert::TryInto;

use num_bigint::BigInt;

use super::Builtin;
use crate::color::Color;
use crate::common::QuoteKind;
use crate::units::Unit;
use crate::value::{Number, Value};

pub(crate) fn register(f: &mut BTreeMap<String, Builtin>) {
    decl!(f "rgb", |args, _| {
        let channels = args.get("channels").unwrap_or(&Value::Null);
        if channels.is_null() {
            let red = match arg!(args, 0, "red").clone().eval() {
                Value::Dimension(n, Unit::None) => n,
                Value::Dimension(n, Unit::Percent) => (n / Number::from(100)) * Number::from(255),
                _ => todo!("expected either unitless or % number for alpha"),
            };
            let green = match arg!(args, 1, "green").clone().eval() {
                Value::Dimension(n, Unit::None) => n,
                Value::Dimension(n, Unit::Percent) => (n / Number::from(100)) * Number::from(255),
                _ => todo!("expected either unitless or % number for alpha"),
            };
            let blue = match arg!(args, 2, "blue").clone().eval() {
                Value::Dimension(n, Unit::None) => n,
                Value::Dimension(n, Unit::Percent) => (n / Number::from(100)) * Number::from(255),
                _ => todo!("expected either unitless or % number for alpha"),
            };
            Some(Value::Color(Color::from_rgba(red, green, blue, Number::from(1))))
        } else {
            todo!("channels variable in `rgb`")
        }
    });
    decl!(f "rgba", |args, _| {
        let channels = args.get("channels").unwrap_or(&Value::Null);
        if channels.is_null() {
            let red = match arg!(args, 0, "red").clone().eval() {
                Value::Dimension(n, Unit::None) => n,
                Value::Dimension(n, Unit::Percent) => (n / Number::from(100)) * Number::from(255),
                _ => todo!("expected either unitless or % number for alpha"),
            };
            let green = match arg!(args, 1, "green").clone().eval() {
                Value::Dimension(n, Unit::None) => n,
                Value::Dimension(n, Unit::Percent) => (n / Number::from(100)) * Number::from(255),
                _ => todo!("expected either unitless or % number for alpha"),
            };
            let blue = match arg!(args, 2, "blue").clone().eval() {
                Value::Dimension(n, Unit::None) => n,
                Value::Dimension(n, Unit::Percent) => (n / Number::from(100)) * Number::from(255),
                _ => todo!("expected either unitless or % number for alpha"),
            };
            let alpha = match arg!(args, 3, "alpha").clone().eval() {
                Value::Dimension(n, Unit::None) => n,
                Value::Dimension(n, Unit::Percent) => n / Number::from(100),
                _ => todo!("expected either unitless or % number for alpha"),
            };
            Some(Value::Color(Color::from_rgba(red, green, blue, alpha)))
        } else {
            todo!("channels variable in `rgba`")
        }
    });
    decl!(f "hsl", |args, _| {
        let hue = match arg!(args, 0, "hue").clone().eval() {
            Value::Dimension(n, Unit::None)
            | Value::Dimension(n, Unit::Percent)
            | Value::Dimension(n, Unit::Deg) => n,
            _ => todo!("expected either unitless or % number for alpha"),
        };
        let saturation = match arg!(args, 1, "saturation").clone().eval() {
            Value::Dimension(n, Unit::None)
            | Value::Dimension(n, Unit::Percent) => n / Number::from(100),
            _ => todo!("expected either unitless or % number for alpha"),
        };
        let luminance = match arg!(args, 2, "luminance").clone().eval() {
            Value::Dimension(n, Unit::None)
            | Value::Dimension(n, Unit::Percent) => n / Number::from(100),
            _ => todo!("expected either unitless or % number for alpha"),
        };
        Some(Value::Color(Color::from_hsla(hue, saturation, luminance, Number::from(1))))
    });
    decl!(f "hsla", |args, _| {
        let hue = match arg!(args, 0, "hue").clone().eval() {
            Value::Dimension(n, Unit::None)
            | Value::Dimension(n, Unit::Percent)
            | Value::Dimension(n, Unit::Deg) => n,
            _ => todo!("expected either unitless or % number for alpha"),
        };
        let saturation = match arg!(args, 1, "saturation").clone().eval() {
            Value::Dimension(n, Unit::None)
            | Value::Dimension(n, Unit::Percent) => n / Number::from(100),
            _ => todo!("expected either unitless or % number for alpha"),
        };
        let luminance = match arg!(args, 2, "luminance").clone().eval() {
            Value::Dimension(n, Unit::None)
            | Value::Dimension(n, Unit::Percent) => n / Number::from(100),
            _ => todo!("expected either unitless or % number for alpha"),
        };
        let alpha = match arg!(args, 3, "alpha").clone().eval() {
            Value::Dimension(n, Unit::None) => n,
            Value::Dimension(n, Unit::Percent) => n / Number::from(100),
            _ => todo!("expected either unitless or % number for alpha"),
        };
        Some(Value::Color(Color::from_hsla(hue, saturation, luminance, alpha)))
    });
    decl!(f "red", |args, _| {
        match arg!(args, 0, "red") {
            Value::Color(c) => Some(Value::Dimension(Number::from(BigInt::from(c.red())), Unit::None)),
            _ => todo!("non-color given to builtin function `red()`")
        }
    });
    decl!(f "green", |args, _| {
        match arg!(args, 0, "green") {
            Value::Color(c) => Some(Value::Dimension(Number::from(BigInt::from(c.green())), Unit::None)),
            _ => todo!("non-color given to builtin function `green()`")
        }
    });
    decl!(f "blue", |args, _| {
        match arg!(args, 0, "blue") {
            Value::Color(c) => Some(Value::Dimension(Number::from(BigInt::from(c.blue())), Unit::None)),
            _ => todo!("non-color given to builtin function `blue()`")
        }
    });
    decl!(f "opacity", |args, _| {
        match arg!(args, 0, "color") {
            Value::Color(c) => Some(Value::Dimension(c.alpha() / Number::from(255), Unit::None)),
            Value::Dimension(num, unit) => Some(Value::Ident(format!("opacity({}{})", num , unit), QuoteKind::None)),
            _ => todo!("non-color given to builtin function `opacity()`")
        }
    });
    decl!(f "alpha", |args, _| {
        match arg!(args, 0, "color") {
            Value::Color(c) => Some(Value::Dimension(c.alpha() / Number::from(255), Unit::None)),
            _ => todo!("non-color given to builtin function `alpha()`")
        }
    });
}
