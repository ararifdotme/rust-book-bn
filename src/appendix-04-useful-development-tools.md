## পরিশিষ্ট D: কাজের development tool {#appendix-d-useful-development-tools}

এই পরিশিষ্টে আমরা Rust project-এর দেওয়া কিছু কাজের development tool নিয়ে কথা বলব। আমরা দেখব স্বয়ংক্রিয় formatting, warning-এর সমাধান দ্রুত প্রয়োগ করার উপায়, একটি linter, আর IDE-র সঙ্গে সংযুক্তি।

### `rustfmt` দিয়ে স্বয়ংক্রিয় formatting {#automatic-formatting-with-rustfmt}

`rustfmt` tool community-র কোড style অনুযায়ী আপনার কোড নতুন করে format করে। অনেক যৌথ project `rustfmt` ব্যবহার করে, যাতে Rust লেখার সময় কোন style ব্যবহার হবে তা নিয়ে তর্ক না হয়: সবাই এই tool দিয়ে নিজের কোড format করে।

Rust installation-এ default-ভাবে `rustfmt` থাকে, তাই আপনার system-এ ইতিমধ্যে `rustfmt` আর `cargo-fmt` প্রোগ্রাম থাকার কথা। এই দুটি command `rustc` আর `cargo`-এর মতো: `rustfmt` আরও সূক্ষ্ম নিয়ন্ত্রণ দেয়, আর `cargo-fmt` Cargo ব্যবহার করা project-এর রীতিনীতি বোঝে। যেকোনো Cargo project format করতে নিচেরটি লিখুন:

```console
$ cargo fmt
```

এই command চালালে বর্তমান crate-এর সব Rust কোড নতুন করে format হয়। এতে শুধু কোডের style বদলানোর কথা, কোডের অর্থ নয়। `rustfmt` সম্পর্কে আরও তথ্যের জন্য [এর documentation][rustfmt] দেখুন।

### `rustfix` দিয়ে কোড ঠিক করা {#fix-your-code-with-rustfix}

`rustfix` tool Rust installation-এর সঙ্গেই থাকে, এবং এমন compiler warning স্বয়ংক্রিয়ভাবে ঠিক করতে পারে, যেগুলোর সমস্যা সমাধানের একটি স্পষ্ট উপায় আছে এবং যা সম্ভবত আপনি চান। হয়তো আগেও compiler warning দেখেছেন। যেমন, এই কোডটি দেখুন:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let mut x = 42;
    println!("{x}");
}
```

এখানে আমরা `x` variable-কে mutable হিসেবে সংজ্ঞায়িত করছি, কিন্তু আসলে কখনো এটি পরিবর্তন করি না। Rust এ নিয়ে আমাদের সতর্ক করে:

```console
$ cargo build
   Compiling myprogram v0.1.0 (file:///projects/myprogram)
warning: variable does not need to be mutable
 --> src/main.rs:2:9
  |
2 |     let mut x = 0;
  |         ----^
  |         |
  |         help: remove this `mut`
  |
  = note: `#[warn(unused_mut)]` on by default
```

warning আমাদের `mut` keyword সরিয়ে ফেলার পরামর্শ দেয়। `cargo
fix` command চালিয়ে `rustfix` tool দিয়ে সেই পরামর্শ স্বয়ংক্রিয়ভাবে প্রয়োগ করতে পারি:

```console
$ cargo fix
    Checking myprogram v0.1.0 (file:///projects/myprogram)
      Fixing src/main.rs (1 fix)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

আবার _src/main.rs_ দেখলে দেখব `cargo fix` কোড বদলে দিয়েছে:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let x = 42;
    println!("{x}");
}
```

`x` variable এখন immutable, আর warning আর দেখা যায় না।

বিভিন্ন Rust edition-এর মধ্যে কোড স্থানান্তর করতেও `cargo fix` command ব্যবহার করা যায়। edition নিয়ে [পরিশিষ্ট E][editions]<!--
ignore -->-তে আলোচনা করা হয়েছে।

### Clippy দিয়ে আরও lint {#more-lints-with-clippy}

Clippy tool হলো আপনার কোড বিশ্লেষণ করার lint-এর একটি সংগ্রহ, যাতে আপনি সাধারণ ভুল ধরতে এবং Rust কোড উন্নত করতে পারেন। আদর্শ Rust installation-এ Clippy থাকে।

যেকোনো Cargo project-এ Clippy-র lint চালাতে নিচেরটি লিখুন:

```console
$ cargo clippy
```

যেমন ধরুন, আপনি এমন একটি প্রোগ্রাম লিখলেন, যা pi-এর মতো একটি গাণিতিক constant-এর আসন্ন মান ব্যবহার করে, যেমনটি এই প্রোগ্রাম করে:

<Listing file-name="src/main.rs">

```rust
fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

</Listing>

এই project-এ `cargo clippy` চালালে এই error হয়:

```text
error: approximate value of `f{32, 64}::consts::PI` found
 --> src/main.rs:2:13
  |
2 |     let x = 3.1415;
  |             ^^^^^^
  |
  = note: `#[deny(clippy::approx_constant)]` on by default
  = help: consider using the constant directly
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#approx_constant
```

এই error আপনাকে জানায় যে Rust-এ ইতিমধ্যে আরও নিখুঁত একটি `PI` constant সংজ্ঞায়িত আছে, এবং এর বদলে সেই constant ব্যবহার করলে আপনার প্রোগ্রাম আরও সঠিক হতো। তখন `PI` constant ব্যবহার করতে আপনি কোড বদলে নেবেন।

নিচের কোডে Clippy থেকে কোনো error বা warning আসে না:

<Listing file-name="src/main.rs">

```rust
fn main() {
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

</Listing>

Clippy সম্পর্কে আরও তথ্যের জন্য [এর documentation][clippy] দেখুন।

### `rust-analyzer` দিয়ে IDE সংযুক্তি {#ide-integration-using-rust-analyzer}

IDE সংযুক্তিতে সাহায্যের জন্য Rust community [`rust-analyzer`][rust-analyzer]<!-- ignore --> ব্যবহারের সুপারিশ করে। এই tool হলো compiler-কেন্দ্রিক কিছু utility-র একটি সেট, যেগুলো [Language Server Protocol][lsp]<!--
ignore --> বোঝে; এটি IDE আর programming language-এর পরস্পরের সঙ্গে যোগাযোগের একটি specification। বিভিন্ন client `rust-analyzer` ব্যবহার করতে পারে, যেমন
[Visual Studio Code-এর জন্য Rust analyzer plug-in][vscode]।

installation-এর নির্দেশনার জন্য `rust-analyzer` project-এর [home page][rust-analyzer]<!-- ignore --> দেখুন, তারপর আপনার নির্দিষ্ট IDE-তে language server support install করুন। আপনার IDE autocompletion, definition-এ লাফিয়ে যাওয়া আর inline error-এর মতো সক্ষমতা পাবে।

[rustfmt]: https://github.com/rust-lang/rustfmt
[editions]: appendix-05-editions.md
[clippy]: https://github.com/rust-lang/rust-clippy
[rust-analyzer]: https://rust-analyzer.github.io
[lsp]: http://langserver.org/
[vscode]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer
