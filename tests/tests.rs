// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.
#[macro_use]
mod common;

#[cfg(feature = "pwdx")]
#[path = "by-util/test_pwdx.rs"]
mod test_pwdx;
