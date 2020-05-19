### The Rust Programming Language
Rust is definitely the newest programming language that I've tried out. The [book](https://doc.rust-lang.org/book/) was pretty well laid out and easy to work through.

### Thoughts on Rust
In general I liked the language. I have some concerns about how mature the ecosystem of crates is, but in general I liked the object oriented approach of rust more than C++. I prefer the Trait mechanism for specifying behavior as opposed to inheriting from abstract classes.

The **good**:
 - The compiler catches almost everything
 - Traits provide a nice source for polymorphism
 - The `match` expressions with pattern matching were quite nice

The **bad**:
 - The compiler catches almost everything
 - Have to bring in external, community crates for a lot of functionality. Small set of "official" crates. 
 - Pretty difficult to get used to `String` vs `str`
 
The **ugly**:
 - String concatentation should be `string1 + string2` not:
 ```rust
let mut string1 = String::from("hello, ");
let string2 = "world!";
string1.push_str(string2);
 ```
