/*!
This crate provides the `Ini` struct which implements a basic configuration language which provides a structure similar to what’s found in Windows' `ini` files.
You can use this to write Rust programs which can be customized by end users easily.

This is a simple configuration parsing utility with no dependencies built on Rust. It is inspired by Python's `configparser`.

The current release is experimental, this means that future releases will be swift until we reach `stable` (1.0.0).
The codebase is thus subject to change for now.

## Quick Start

A basic `ini`-syntax file (we say ini-syntax files because the files don't need to be necessarily `*.ini`) looks like this:
```INI
[DEFAULT]
key1 = value1
pizzatime = yes
cost = 9

[topsecrets]
nuclear launch codes = topsecret

[github.com]
User = QEDK
```
Essentially, the syntax consists of sections, each of which can which contains keys with values. The `Ini` struct can read and write such values.

## Supported datatypes
`configparser` does not guess the datatype of values in configuration files and stores everything as strings. However, some datatypes are so common
that it's a safe bet that some values need to be parsed in other types. For this, the `Ini` struct provides easy functions like `getint()`, `getuint()`,
`getfloat()` and `getbool()`. The only bit of extra magic involved is that the `getbool()` function will treat boolean values case-insensitively (so
`true` is the same as `True` just like `TRUE`). You can ofcourse just choose to parse the string values yourself.
```ignore,rust
let my_string = String::from("1984");
let my_int = my_string.parse::<i32>().unwrap();
let my_value = config.getint("somesection", "someintvalue")?.unwrap();
```


## Supported `ini` file structure
A configuration file can consist of sections, each led by a `[section-name]` header, followed by key-value entries separated by a `=`. By default, section names and key names are case-insensitive. All leading and trailing whitespace is removed from stored keys, values and section names.
Key values can be omitted, in which case the key-value delimiter (`=`) may also be left out (but this is different from putting a delimiter, we'll
explain it later). Key-value pairs or section headers cannot span multiple lines.
Owing to how ini files usually are, this means that `[`, `]` and `=` are special symbols (this crate will allow you to use `]` sparingly).

Let's take for example:
```INI
[section headers are case-insensitive]
[   section headers are case-insensitive    ]
are the section headers above same? = yes
sectionheaders_and_keysarestored_in_lowercase? = yes
keys_are_also_case_insensitive = Values are case sensitive
spaces in keys=allowed
spaces in values=allowed as well
spaces around the delimiter = also OK


[All values are strings]
values like this= 0000
or this= 0.999
are they treated as numbers? = no
integers, floats and booleans are held as= strings

[value-less?]
a_valueless_key_has_None
this key has an empty string value has Some("") =

	[indented sections]
		can_values_be_as_well = True
		purpose = formatting for readability
		is_this_same     =        yes
			is_this_same=yes
```
An important thing to note is that values with the same keys will get updated, this means that the last inserted key (whether that's a section header
or property key) is the one that remains in the `HashMap`.
The only bit of magic the API does is the section-less properties are put in a section called "default". It is planned to allow configuring this variable.

## Usage
Let's take another simple `ini` file and talk about working with it:
```INI
[topsecret]
KFC = the secret herb is orega-

[values]
Int = -31415
```
If you read the above sections carefully, you'll know that 1) all the keys are stored in lowercase, 2) `get()` can make access in a case-insensitive
manner and 3) we can use `getint()` to parse the `Int` value into an `i64`. Let's see that in action.

```rust
use configparser::ini::Ini;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  let mut config = Ini::new();

  // You can easily load a file to get a clone of the map:
  let map = config.load("tests/test.ini")?;
  println!("{:?}", map);
  // You can also safely not store the reference and access it later with get_map_ref() or get a clone with get_map()

  // You can then access it like a normal hashmap:
  let innermap = map["topsecret"].clone(); // Remember this is a hashmap!

  // If you want to access the value, then you can simply do:
  let val = map["topsecret"]["kfc"].clone().unwrap();
  // Lowercasing when accessing map directly is important because all keys are stored in lower-case!
  // Note: The .clone().unwrap() is required because it's an Option<String> type.

  assert_eq!(val, "the secret herb is orega-"); // value accessible!

  // What if you want to mutate the parser and remove KFC's secret recipe? Just use get_mut_map():
  let mut_map = config.get_mut_map();
  mut_map.get_mut("topsecret").unwrap().insert(String::from("kfc"), None);
  // And the secret is back in safety, remember that these are normal HashMap functions chained for convenience.

  // However very quickly see how that becomes cumbersome, so you can use the handy get() function from Ini
  // The get() function accesses the map case-insensitively, so you can use uppercase as well:
  let val = config.get("topsecret", "KFC"); // unwrapping will be an error because we just emptied it!
  assert_eq!(val, None); // as expected!

  // What if you want to get a number?
  let my_number = config.getint("values", "Int")?.unwrap();
  assert_eq!(my_number, -31415); // and we got it!
  // The Ini struct provides more getters for primitive datatypes.

  Ok(())
}
```
*/
pub use configparser;

macro_rules! ini {
	{$path:literal}
}