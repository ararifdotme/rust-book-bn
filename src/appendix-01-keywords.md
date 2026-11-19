## পরিশিষ্ট A: keyword {#appendix-a-keywords}

নিচের তালিকাগুলোতে এমন keyword আছে, যেগুলো Rust ভাষার বর্তমান বা ভবিষ্যৎ ব্যবহারের জন্য সংরক্ষিত। তাই এগুলো identifier হিসেবে ব্যবহার করা যায় না (raw identifier ছাড়া, যা নিয়ে আমরা [“raw
identifier”][raw-identifiers]<!-- ignore --> অংশে আলোচনা করেছি)। _identifier_ হলো function, variable, parameter, struct field, module, crate, constant, macro, static মান, attribute, type, trait বা lifetime-এর নাম।

[raw-identifiers]: #raw-identifiers

### বর্তমানে ব্যবহৃত keyword {#keywords-currently-in-use}

নিচে বর্তমানে ব্যবহৃত keyword-গুলোর তালিকা দেওয়া হলো, সঙ্গে প্রতিটির কাজের বর্ণনা।

- **`as`**: primitive casting করা, কোনো item ধারণকারী নির্দিষ্ট trait-এর দ্ব্যর্থতা দূর করা, অথবা `use` statement-এ item-এর নাম বদলানো।
- **`async`**: বর্তমান thread block না করে একটি `Future` return করা।
- **`await`**: একটি `Future`-এর ফল প্রস্তুত না হওয়া পর্যন্ত চলা স্থগিত রাখা।
- **`break`**: সঙ্গে সঙ্গে একটি loop থেকে বেরিয়ে যাওয়া।
- **`const`**: constant item বা constant raw pointer সংজ্ঞায়িত করা।
- **`continue`**: loop-এর পরের iteration-এ চলে যাওয়া।
- **`crate`**: module path-এ crate root বোঝায়।
- **`dyn`**: একটি trait object-এ dynamic dispatch।
- **`else`**: `if` আর `if let` control flow কাঠামোর বিকল্প পথ।
- **`enum`**: একটি enumeration সংজ্ঞায়িত করা।
- **`extern`**: একটি বাইরের function বা variable link করা।
- **`false`**: Boolean মিথ্যা literal।
- **`fn`**: একটি function বা function pointer type সংজ্ঞায়িত করা।
- **`for`**: একটি iterator-এর item-গুলোর ওপর দিয়ে loop করা, একটি trait implement করা, অথবা একটি higher ranked lifetime উল্লেখ করা।
- **`if`**: একটি শর্তযুক্ত expression-এর ফলের ভিত্তিতে শাখায় যাওয়া।
- **`impl`**: inherent বা trait কার্যক্ষমতা implement করা।
- **`in`**: `for` loop syntax-এর অংশ।
- **`let`**: একটি variable bind করা।
- **`loop`**: শর্তহীনভাবে loop করা।
- **`match`**: একটি মানকে pattern-এর সঙ্গে মেলানো।
- **`mod`**: একটি module সংজ্ঞায়িত করা।
- **`move`**: একটি closure-কে তার capture করা সবকিছুর ownership নেওয়ানো।
- **`mut`**: reference, raw pointer বা pattern binding-এ mutability বোঝানো।
- **`pub`**: struct field, `impl` block বা module-এ public visibility বোঝানো।
- **`ref`**: reference দিয়ে bind করা।
- **`return`**: function থেকে return করা।
- **`Self`**: আমরা যে type সংজ্ঞায়িত বা implement করছি তার একটি type alias।
- **`self`**: method-এর subject বা বর্তমান module।
- **`static`**: global variable বা পুরো প্রোগ্রাম চলার সময় জুড়ে টিকে থাকা lifetime।
- **`struct`**: একটি structure সংজ্ঞায়িত করা।
- **`super`**: বর্তমান module-এর parent module।
- **`trait`**: একটি trait সংজ্ঞায়িত করা।
- **`true`**: Boolean সত্য literal।
- **`type`**: একটি type alias বা associated type সংজ্ঞায়িত করা।
- **`union`**: একটি [union][union]<!-- ignore --> সংজ্ঞায়িত করা; শুধু union declaration-এ ব্যবহার করা হলেই এটি keyword।
- **`unsafe`**: unsafe কোড, function, trait বা implementation বোঝানো।
- **`use`**: symbol-কে scope-এ আনা।
- **`where`**: একটি type-কে সীমাবদ্ধ করা clause বোঝানো।
- **`while`**: একটি expression-এর ফলের ভিত্তিতে শর্তসাপেক্ষে loop করা।

[union]: https://doc.rust-lang.org/reference/items/unions.html

### ভবিষ্যৎ ব্যবহারের জন্য সংরক্ষিত keyword {#keywords-reserved-for-future-use}

নিচের keyword-গুলোর এখনো কোনো কার্যক্ষমতা নেই, তবে সম্ভাব্য ভবিষ্যৎ ব্যবহারের জন্য Rust এগুলো সংরক্ষিত রেখেছে:

- `abstract`
- `become`
- `box`
- `do`
- `final`
- `gen`
- `macro`
- `override`
- `priv`
- `try`
- `typeof`
- `unsized`
- `virtual`
- `yield`

### raw identifier {#raw-identifiers}

_raw identifier_ হলো এমন syntax, যা সাধারণত অনুমতি নেই এমন জায়গায় keyword ব্যবহার করতে দেয়। একটি keyword-এর আগে `r#` বসিয়ে raw identifier ব্যবহার করা হয়।

যেমন, `match` একটি keyword। নাম হিসেবে `match` ব্যবহার করা নিচের function compile করার চেষ্টা করলে:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
fn match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}
```

আপনি এই error পাবেন:

```text
error: expected identifier, found keyword `match`
 --> src/main.rs:4:4
  |
4 | fn match(needle: &str, haystack: &str) -> bool {
  |    ^^^^^ expected identifier, found keyword
```

error দেখায় যে function-এর identifier হিসেবে `match` keyword ব্যবহার করা যায় না। function-এর নাম হিসেবে `match` ব্যবহার করতে আপনাকে raw identifier syntax ব্যবহার করতে হবে, এভাবে:

<span class="filename">Filename: src/main.rs</span>

```rust
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
    assert!(r#match("foo", "foobar"));
}
```

এই কোড কোনো error ছাড়াই compile হবে। খেয়াল করুন, function-এর সংজ্ঞায় তার নামের আগে এবং `main`-এ যেখানে function call করা হয়েছে সেখানেও `r#` prefix আছে।

raw identifier আপনার পছন্দের যেকোনো শব্দকে identifier হিসেবে ব্যবহার করতে দেয়, এমনকি শব্দটি একটি সংরক্ষিত keyword হলেও। এতে identifier-এর নাম বাছাইয়ে আমরা আরও স্বাধীনতা পাই, আবার এমন ভাষায় লেখা প্রোগ্রামের সঙ্গে যুক্ত হতে পারি,
যেখানে এই শব্দগুলো keyword নয়। তা ছাড়া raw identifier আপনার crate যে Rust edition ব্যবহার করে, তার থেকে ভিন্ন edition-এ লেখা library ব্যবহার করতে দেয়। যেমন, 2015 edition-এ `try` keyword নয়, কিন্তু 2018, 2021 আর 2024
edition-এ keyword। আপনি যদি 2015 edition দিয়ে লেখা এমন একটি library-র ওপর নির্ভর করেন, যার একটি `try` function আছে, তাহলে পরের edition-এ আপনার কোড থেকে সেই function call করতে raw identifier syntax, এই ক্ষেত্রে
`r#try`, ব্যবহার করতে হবে। edition সম্পর্কে আরও তথ্যের জন্য [পরিশিষ্ট E][appendix-e]<!-- ignore --> দেখুন।

[appendix-e]: appendix-05-editions.html
