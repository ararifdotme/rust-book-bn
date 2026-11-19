## পরিশিষ্ট B: operator ও চিহ্ন {#appendix-b-operators-and-symbols}

এই পরিশিষ্টে Rust-এর syntax-এর একটি শব্দকোষ আছে, যার মধ্যে আছে operator আর অন্যান্য চিহ্ন, যেগুলো নিজে নিজে অথবা path, generic, trait bound, macro, attribute, comment, tuple ও বন্ধনীর প্রসঙ্গে দেখা যায়।

### operator {#operators}

Table B-1-এ Rust-এর operator-গুলো আছে, সঙ্গে প্রসঙ্গের মধ্যে operator-টি কেমন দেখাবে তার একটি উদাহরণ, একটি সংক্ষিপ্ত ব্যাখ্যা, এবং operator-টি overload করা যায় কি না। কোনো operator overload করা গেলে সেটি overload করতে যে trait ব্যবহার করতে হয় তা-ও তালিকায় দেওয়া আছে।

<span class="caption">Table B-1: operator</span>

| operator | উদাহরণ | ব্যাখ্যা | overload করা যায়? |
| ------------------------- | ------------------------------------------------------- | --------------------------------------------------------------------- | -------------- |
| `!` | `ident!(...)`, `ident!{...}`, `ident![...]` | macro expansion |  |
| `!` | `!expr` | bitwise বা যৌক্তিক complement | `Not` |
| `!=` | `expr != expr` | অসমতার তুলনা | `PartialEq` |
| `%` | `expr % expr` | গাণিতিক ভাগশেষ | `Rem` |
| `%=` | `var %= expr` | গাণিতিক ভাগশেষ ও assignment | `RemAssign` |
| `&` | `&expr`, `&mut expr` | borrow |  |
| `&` | `&type`, `&mut type`, `&'a type`, `&'a mut type` | borrow করা pointer type |  |
| `&` | `expr & expr` | bitwise AND | `BitAnd` |
| `&=` | `var &= expr` | bitwise AND ও assignment | `BitAndAssign` |
| `&&` | `expr && expr` | short-circuit করা যৌক্তিক AND |  |
| `*` | `expr * expr` | গাণিতিক গুণ | `Mul` |
| `*=` | `var *= expr` | গাণিতিক গুণ ও assignment | `MulAssign` |
| `*` | `*expr` | dereference | `Deref` |
| `*` | `*const type`, `*mut type` | raw pointer |  |
| `+` | `trait + trait`, `'a + trait` | যৌগিক type constraint |  |
| `+` | `expr + expr` | গাণিতিক যোগ | `Add` |
| `+=` | `var += expr` | গাণিতিক যোগ ও assignment | `AddAssign` |
| `,` | `expr, expr` | argument ও উপাদানের বিভাজক |  |
| `-` | `- expr` | গাণিতিক ঋণাত্মককরণ | `Neg` |
| `-` | `expr - expr` | গাণিতিক বিয়োগ | `Sub` |
| `-=` | `var -= expr` | গাণিতিক বিয়োগ ও assignment | `SubAssign` |
| `->` | `fn(...) -> type`, <code>&vert;...&vert; -> type</code> | function ও closure-এর return type |  |
| `.` | `expr.ident` | field-এ প্রবেশ |  |
| `.` | `expr.ident(expr, ...)` | method call |  |
| `.` | `expr.0`, `expr.1` ইত্যাদি | tuple indexing |  |
| `..` | `..`, `expr..`, `..expr`, `expr..expr` | ডান প্রান্ত বাদ দেওয়া range literal | `PartialOrd` |
| `..=` | `..=expr`, `expr..=expr` | ডান প্রান্ত অন্তর্ভুক্ত range literal | `PartialOrd` |
| `..` | `..expr` | struct literal update syntax |  |
| `..` | `variant(x, ..)`, `struct_type { x, .. }` | “এবং বাকিগুলো” pattern binding |  |
| `...` | `expr...expr` | (অপ্রচলিত, এর বদলে `..=` ব্যবহার করুন) pattern-এ: অন্তর্ভুক্তিমূলক range pattern |  |
| `/` | `expr / expr` | গাণিতিক ভাগ | `Div` |
| `/=` | `var /= expr` | গাণিতিক ভাগ ও assignment | `DivAssign` |
| `:` | `pat: type`, `ident: type` | constraint |  |
| `:` | `ident: expr` | struct field initializer |  |
| `:` | `'a: loop {...}` | loop label |  |
| `;` | `expr;` | statement ও item-এর সমাপ্তিসূচক |  |
| `;` | `[...; len]` | নির্দিষ্ট আকারের array syntax-এর অংশ |  |
| `<<` | `expr << expr` | left-shift | `Shl` |
| `<<=` | `var <<= expr` | left-shift ও assignment | `ShlAssign` |
| `<` | `expr < expr` | ছোট কি না তার তুলনা | `PartialOrd` |
| `<=` | `expr <= expr` | ছোট বা সমান কি না তার তুলনা | `PartialOrd` |
| `=` | `var = expr`, `ident = type` | assignment/সমতুল্যতা |  |
| `==` | `expr == expr` | সমতার তুলনা | `PartialEq` |
| `=>` | `pat => expr` | match arm syntax-এর অংশ |  |
| `>` | `expr > expr` | বড় কি না তার তুলনা | `PartialOrd` |
| `>=` | `expr >= expr` | বড় বা সমান কি না তার তুলনা | `PartialOrd` |
| `>>` | `expr >> expr` | right-shift | `Shr` |
| `>>=` | `var >>= expr` | right-shift ও assignment | `ShrAssign` |
| `@` | `ident @ pat` | pattern binding |  |
| `^` | `expr ^ expr` | bitwise exclusive OR | `BitXor` |
| `^=` | `var ^= expr` | bitwise exclusive OR ও assignment | `BitXorAssign` |
| <code>&vert;</code> | <code>pat &vert; pat</code> | pattern-এর বিকল্প |  |
| <code>&vert;</code> | <code>expr &vert; expr</code> | bitwise OR | `BitOr` |
| <code>&vert;=</code> | <code>var &vert;= expr</code> | bitwise OR ও assignment | `BitOrAssign` |
| <code>&vert;&vert;</code> | <code>expr &vert;&vert; expr</code> | short-circuit করা যৌক্তিক OR |  |
| `?` | `expr?` | error propagation |  |

### operator নয় এমন চিহ্ন {#non-operator-symbols}

নিচের table-গুলোতে এমন সব চিহ্ন আছে, যেগুলো operator হিসেবে কাজ করে না; অর্থাৎ এগুলো function বা method call-এর মতো আচরণ করে না।

Table B-2-এ এমন চিহ্ন দেখানো হয়েছে, যেগুলো নিজে নিজে দেখা যায় এবং নানা জায়গায় বৈধ।

<span class="caption">Table B-2: স্বতন্ত্র syntax</span>

| চিহ্ন | ব্যাখ্যা |
| ---------------------------------------------------------------------- | ---------------------------------------------------------------------- |
| `'ident` | নামযুক্ত lifetime বা loop label |
| অঙ্কের ঠিক পরে `u8`, `i32`, `f64`, `usize` ইত্যাদি | নির্দিষ্ট type-এর সংখ্যা literal |
| `"..."` | string literal |
| `r"..."`, `r#"..."#`, `r##"..."##` ইত্যাদি | raw string literal; escape character প্রক্রিয়া করা হয় না |
| `b"..."` | byte string literal; string-এর বদলে byte-এর একটি array তৈরি করে |
| `br"..."`, `br#"..."#`, `br##"..."##` ইত্যাদি | raw byte string literal; raw ও byte string literal-এর সমন্বয় |
| `'...'` | character literal |
| `b'...'` | ASCII byte literal |
| <code>&vert;...&vert; expr</code> | closure |
| `!` | diverging function-এর জন্য সবসময় খালি bottom type |
| `_` | “উপেক্ষিত” pattern binding; integer literal পড়ার উপযোগী করতেও ব্যবহৃত হয় |

Table B-3-এ এমন চিহ্ন দেখানো হয়েছে, যেগুলো module hierarchy-র মধ্য দিয়ে একটি item-এ পৌঁছানোর path-এর প্রসঙ্গে দেখা যায়।

<span class="caption">Table B-3: path-সংক্রান্ত syntax</span>

| চিহ্ন | ব্যাখ্যা |
| --------------------------------------- | -------------------------------------------------------------------------------------------------------------|
| `ident::ident` | namespace path |
| `::path` | crate root-এর সাপেক্ষে path (অর্থাৎ স্পষ্টভাবে absolute path) |
| `self::path` | বর্তমান module-এর সাপেক্ষে path (অর্থাৎ স্পষ্টভাবে relative path) |
| `super::path` | বর্তমান module-এর parent-এর সাপেক্ষে path |
| `type::ident`, `<type as trait>::ident` | associated constant, function ও type |
| `<type>::...` | এমন type-এর associated item, যার সরাসরি নাম দেওয়া যায় না (যেমন `<&T>::...`, `<[T]>::...` ইত্যাদি) |
| `trait::method(...)` | method সংজ্ঞায়িত করা trait-এর নাম দিয়ে method call-এর দ্ব্যর্থতা দূর করা |
| `type::method(...)` | যে type-এর জন্য method সংজ্ঞায়িত, তার নাম দিয়ে method call-এর দ্ব্যর্থতা দূর করা |
| `<type as trait>::method(...)` | trait ও type-এর নাম দিয়ে method call-এর দ্ব্যর্থতা দূর করা |

Table B-4-এ এমন চিহ্ন দেখানো হয়েছে, যেগুলো generic type parameter ব্যবহারের প্রসঙ্গে দেখা যায়।

<span class="caption">Table B-4: generic</span>

| চিহ্ন | ব্যাখ্যা |
| ------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------- |
| `path<...>` | একটি type-এর ভেতরে generic type-এর parameter উল্লেখ করে (যেমন `Vec<u8>`) |
| `path::<...>`, `method::<...>` | একটি expression-এ generic type, function বা method-এর parameter উল্লেখ করে; প্রায়ই একে _turbofish_ বলা হয় (যেমন `"42".parse::<i32>()`) |
| `fn ident<...> ...` | generic function সংজ্ঞায়িত করা |
| `struct ident<...> ...` | generic structure সংজ্ঞায়িত করা |
| `enum ident<...> ...` | generic enumeration সংজ্ঞায়িত করা |
| `impl<...> ...` | generic implementation সংজ্ঞায়িত করা |
| `for<...> type` | higher ranked lifetime bound |
| `type<ident=type>` | এমন generic type, যার এক বা একাধিক associated type-এ নির্দিষ্ট মান বসানো আছে (যেমন `Iterator<Item=T>`) |

Table B-5-এ এমন চিহ্ন দেখানো হয়েছে, যেগুলো trait bound দিয়ে generic type parameter সীমাবদ্ধ করার প্রসঙ্গে দেখা যায়।

<span class="caption">Table B-5: trait bound constraint</span>

| চিহ্ন | ব্যাখ্যা |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| `T: U` | generic parameter `T` শুধু `U` implement করা type-এ সীমাবদ্ধ |
| `T: 'a` | generic type `T`-কে lifetime `'a`-এর চেয়ে বেশি দিন টিকতে হবে (অর্থাৎ type-টি পরোক্ষভাবেও `'a`-এর চেয়ে ছোট lifetime-যুক্ত কোনো reference রাখতে পারবে না) |
| `T: 'static` | generic type `T`-এ `'static` ছাড়া অন্য কোনো borrow করা reference নেই |
| `'b: 'a` | generic lifetime `'b`-কে lifetime `'a`-এর চেয়ে বেশি দিন টিকতে হবে |
| `T: ?Sized` | generic type parameter-কে dynamically sized type হতে দেওয়া |
| `'a + trait`, `trait + trait` | যৌগিক type constraint |

Table B-6-এ এমন চিহ্ন দেখানো হয়েছে, যেগুলো macro call বা সংজ্ঞায়িত করা এবং একটি item-এ attribute উল্লেখ করার প্রসঙ্গে দেখা যায়।

<span class="caption">Table B-6: macro ও attribute</span>

| চিহ্ন | ব্যাখ্যা |
| ------------------------------------------- | ------------------ |
| `#[meta]` | outer attribute |
| `#![meta]` | inner attribute |
| `$ident` | macro substitution |
| `$ident:kind` | macro metavariable |
| `$(...)...` | macro repetition |
| `ident!(...)`, `ident!{...}`, `ident![...]` | macro invocation |

Table B-7-এ comment তৈরি করা চিহ্নগুলো দেখানো হয়েছে।

<span class="caption">Table B-7: comment</span>

| চিহ্ন | ব্যাখ্যা |
| ---------- | ----------------------- |
| `//` | লাইন comment |
| `//!` | inner লাইন doc comment |
| `///` | outer লাইন doc comment |
| `/*...*/` | block comment |
| `/*!...*/` | inner block doc comment |
| `/**...*/` | outer block doc comment |

Table B-8-এ দেখানো হয়েছে কোন কোন প্রসঙ্গে প্রথম বন্ধনী ব্যবহৃত হয়।

<span class="caption">Table B-8: প্রথম বন্ধনী</span>

| চিহ্ন | ব্যাখ্যা |
| ------------------------ | ------------------------------------------------------------------------------------------- |
| `()` | খালি tuple (অর্থাৎ unit), literal ও type দুই-ই |
| `(expr)` | বন্ধনীযুক্ত expression |
| `(expr,)` | এক-উপাদানের tuple expression |
| `(type,)` | এক-উপাদানের tuple type |
| `(expr, ...)` | tuple expression |
| `(type, ...)` | tuple type |
| `expr(expr, ...)` | function call expression; tuple `struct` আর tuple `enum` variant-এর প্রাথমিক মান দিতেও ব্যবহৃত হয় |

Table B-9-এ দেখানো হয়েছে কোন কোন প্রসঙ্গে দ্বিতীয় বন্ধনী (curly bracket) ব্যবহৃত হয়।

<span class="caption">Table B-9: দ্বিতীয় বন্ধনী</span>

| প্রসঙ্গ | ব্যাখ্যা |
| ------------ | ---------------- |
| `{...}` | block expression |
| `Type {...}` | struct literal |

Table B-10-এ দেখানো হয়েছে কোন কোন প্রসঙ্গে তৃতীয় বন্ধনী (square bracket) ব্যবহৃত হয়।

<span class="caption">Table B-10: তৃতীয় বন্ধনী</span>

| প্রসঙ্গ | ব্যাখ্যা |
| -------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `[...]` | array literal |
| `[expr; len]` | `expr`-এর `len` সংখ্যক কপিসহ array literal |
| `[type; len]` | `type`-এর `len` সংখ্যক instance-সহ array type |
| `expr[expr]` | collection indexing; overload করা যায় (`Index`, `IndexMut`) |
| `expr[..]`, `expr[a..]`, `expr[..b]`, `expr[a..b]` | collection slicing-এর ভান করা collection indexing, যেখানে “index” হিসেবে `Range`, `RangeFrom`, `RangeTo` বা `RangeFull` ব্যবহৃত হয় |
