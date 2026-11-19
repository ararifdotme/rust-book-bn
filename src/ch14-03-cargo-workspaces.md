## Cargo workspace {#cargo-workspaces}

অধ্যায় 12-এ আমরা একটি package বানিয়েছিলাম, যাতে একটি binary crate আর একটি library crate ছিল। আপনার project এগোতে থাকলে হয়তো দেখবেন library
crate বড় হতেই থাকছে, আর আপনি package-টিকে আরও ভাগ করে একাধিক library crate-এ বিভক্ত করতে চাইছেন। Cargo _workspaces_ (workspace) নামের একটি
বৈশিষ্ট্য দেয়, যা একসঙ্গে তৈরি হওয়া একাধিক সম্পর্কিত package সামলাতে সাহায্য করে।

### একটি workspace তৈরি করা {#creating-a-workspace}

_workspace_ হলো এমন কিছু package-এর একটি সেট, যারা একই _Cargo.lock_ আর output directory ভাগ করে নেয়। চলুন workspace ব্যবহার করে একটি project
বানাই—আমরা খুব সাধারণ কোড ব্যবহার করব, যাতে workspace-এর গঠনের দিকে মনোযোগ দিতে পারি। workspace সাজানোর একাধিক উপায় আছে, তাই আমরা শুধু
একটি প্রচলিত উপায় দেখাব। আমাদের workspace-এ একটি binary আর দুটি library থাকবে। binary-টি মূল কার্যক্ষমতা দেবে এবং দুটি library-র ওপর নির্ভর করবে।
একটি library দেবে একটি `add_one` function, আর অন্য library-টি একটি `add_two` function। এই তিনটি crate একই workspace-এর অংশ হবে। শুরু করব workspace-এর
জন্য একটি নতুন directory তৈরি করে:

```console
$ mkdir add
$ cd add
```

এরপর _add_ directory-তে আমরা _Cargo.toml_ ফাইল তৈরি করি, যা পুরো workspace configure করবে। এই ফাইলে কোনো `[package]` অংশ থাকবে না। বরং এটি শুরু
হবে একটি `[workspace]` অংশ দিয়ে, যা দিয়ে workspace-এ সদস্য যোগ করা যাবে। `resolver`-এর মান `"3"` করে আমরা workspace-এ Cargo-র resolver
algorithm-এর সবচেয়ে নতুন ও সেরা সংস্করণ ব্যবহারের ব্যবস্থাও করি:

<span class="filename">Filename: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-01-workspace/add/Cargo.toml}}
```

এরপর _add_ directory-র ভেতরে `cargo new` চালিয়ে `adder` binary crate তৈরি করব:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-01-adder-crate/add
remove `members = ["adder"]` from Cargo.toml
rm -rf adder
cargo new adder
copy output below
-->

```console
$ cargo new adder
     Created binary (application) `adder` package
      Adding `adder` as member of workspace at `file:///projects/add`
```

workspace-এর ভেতরে `cargo new` চালালে নতুন তৈরি package-টি স্বয়ংক্রিয়ভাবে workspace-এর _Cargo.toml_-এর `[workspace]` সংজ্ঞার `members` key-তেও
যুক্ত হয়, এভাবে:

```toml
{{#include ../listings/ch14-more-about-cargo/output-only-01-adder-crate/add/Cargo.toml}}
```

এই পর্যায়ে `cargo build` চালিয়ে workspace build করতে পারি। আপনার _add_ directory-র ফাইলগুলো দেখতে এমন হওয়ার কথা:

```text
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

workspace-এর শীর্ষ স্তরে একটি _target_ directory থাকে, যেখানে compile করা artifact-গুলো রাখা হবে; `adder` package-এর নিজস্ব কোনো _target_
directory নেই। এমনকি _adder_ directory-র ভেতর থেকে `cargo build` চালালেও compile করা artifact-গুলো _add/adder/target_-এ নয়, _add/target_-এই
যাবে। Cargo workspace-এর _target_ directory এভাবে সাজায়, কারণ workspace-এর crate-গুলো একে অপরের ওপর নির্ভর করার জন্যই। প্রতিটি crate-এর নিজস্ব
_target_ directory থাকলে প্রতিটি crate-কে নিজের _target_ directory-তে artifact রাখতে workspace-এর অন্য প্রতিটি crate আবার compile করতে হতো। একটি
_target_ directory ভাগ করে নেওয়ায় crate-গুলো অপ্রয়োজনীয় rebuild এড়াতে পারে।

### workspace-এ দ্বিতীয় package তৈরি করা {#creating-the-second-package-in-the-workspace}

এরপর workspace-এ আরেকটি সদস্য package তৈরি করি, যার নাম দেব `add_one`। `add_one` নামের একটি নতুন library crate তৈরি করুন:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-02-add-one/add
remove `"add_one"` from `members` list in Cargo.toml
rm -rf add_one
cargo new add_one --lib
copy output below
-->

```console
$ cargo new add_one --lib
     Created library `add_one` package
      Adding `add_one` as member of workspace at `file:///projects/add`
```

শীর্ষ স্তরের _Cargo.toml_-এর `members` তালিকায় এখন _add_one_ path থাকবে:

<span class="filename">Filename: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/Cargo.toml}}
```

আপনার _add_ directory-তে এখন এই directory আর ফাইলগুলো থাকার কথা:

```text
├── Cargo.lock
├── Cargo.toml
├── add_one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

_add_one/src/lib.rs_ ফাইলে একটি `add_one` function যোগ করি:

<span class="filename">Filename: add_one/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/add_one/src/lib.rs}}
```

এখন আমাদের binary-যুক্ত `adder` package-কে আমাদের library-যুক্ত `add_one` package-এর ওপর নির্ভরশীল করতে পারি। প্রথমে _adder/Cargo.toml_-এ
`add_one`-এর ওপর একটি path dependency যোগ করতে হবে।

<span class="filename">Filename: adder/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/add/adder/Cargo.toml:6:7}}
```

Cargo ধরে নেয় না যে workspace-এর crate-গুলো একে অপরের ওপর নির্ভর করবে, তাই নির্ভরতার সম্পর্কগুলো স্পষ্ট করে বলে দিতে হয়।

এরপর `adder` crate-এ (`add_one` crate-এর) `add_one` function ব্যবহার করি। _adder/src/main.rs_ ফাইলটি খুলে `main` function বদলে `add_one` function
call করান, যেমনটি Listing 14-7-এ আছে।

<Listing number="14-7" file-name="adder/src/main.rs" caption="`adder` crate থেকে `add_one` library crate ব্যবহার করা">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-07/add/adder/src/main.rs}}
```

</Listing>

চলুন শীর্ষ স্তরের _add_ directory-তে `cargo build` চালিয়ে workspace build করি!

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-07/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
```

_add_ directory থেকে binary crate চালাতে `cargo run`-এর সঙ্গে `-p` argument আর package-এর নাম দিয়ে বলে দিতে পারি workspace-এর কোন package চালাতে চাই:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-07/add
cargo run -p adder
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo run -p adder
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/adder`
Hello, world! 10 plus one is 11!
```

এটি _adder/src/main.rs_-এর কোড চালায়, যা `add_one` crate-এর ওপর নির্ভরশীল।

<!-- Old headings. Do not remove or links may break. -->

<a id="depending-on-an-external-package-in-a-workspace"></a>

### একটি বাইরের package-এর ওপর নির্ভর করা {#depending-on-an-external-package}

খেয়াল করুন, প্রতিটি crate-এর directory-তে একটি করে _Cargo.lock_ না থেকে workspace-এর শীর্ষ স্তরে মাত্র একটি _Cargo.lock_ ফাইল আছে। এতে নিশ্চিত হয়
যে সব crate সব dependency-র একই version ব্যবহার করছে। _adder/Cargo.toml_ আর _add_one/Cargo.toml_ ফাইলে `rand` package যোগ করলে Cargo দুটিকেই
`rand`-এর একটি version-এ মিলিয়ে নেবে এবং সেটি ওই একটি _Cargo.lock_-এ লিখে রাখবে। workspace-এর সব crate একই dependency ব্যবহার করলে crate-গুলো
সবসময় একে অপরের সঙ্গে সামঞ্জস্যপূর্ণ থাকে। চলুন _add_one/Cargo.toml_ ফাইলের `[dependencies]` অংশে `rand` crate যোগ করি, যাতে `add_one` crate-এ
`rand` crate ব্যবহার করতে পারি:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
-->

<span class="filename">Filename: add_one/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/add/add_one/Cargo.toml:6:7}}
```

এখন _add_one/src/lib.rs_ ফাইলে `use rand;` যোগ করতে পারি, আর _add_ directory-তে `cargo build` চালিয়ে পুরো workspace build করলে `rand` crate নামিয়ে
এনে compile হবে। আমরা একটি warning পাব, কারণ scope-এ আনা `rand`-কে আমরা কোথাও ব্যবহার করছি না:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.5
   --snip--
   Compiling rand v0.8.5
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
warning: unused import: `rand`
 --> add_one/src/lib.rs:1:5
  |
1 | use rand;
  |     ^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `add_one` (lib) generated 1 warning (run `cargo fix --lib -p add_one` to apply 1 suggestion)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.95s
```

শীর্ষ স্তরের _Cargo.lock_-এ এখন `rand`-এর ওপর `add_one`-এর নির্ভরতার তথ্য আছে। তবে workspace-এর কোথাও `rand` ব্যবহৃত হলেও workspace-এর অন্য
crate-গুলোতে এটি ব্যবহার করা যাবে না, যতক্ষণ না তাদের _Cargo.toml_ ফাইলেও `rand` যোগ করি। যেমন, `adder` package-এর _adder/src/main.rs_ ফাইলে `use rand;`
যোগ করলে আমরা error পাব:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-03-use-rand/add
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
  --snip--
   Compiling adder v0.1.0 (file:///projects/add/adder)
error[E0432]: unresolved import `rand`
 --> adder/src/main.rs:2:5
  |
2 | use rand;
  |     ^^^^ no external crate `rand`
```

এটি ঠিক করতে `adder` package-এর _Cargo.toml_ ফাইল সম্পাদনা করে জানিয়ে দিন যে `rand` এরও একটি dependency। `adder` package build করলে _Cargo.lock_-এ
`adder`-এর dependency তালিকায় `rand` যুক্ত হবে, কিন্তু `rand`-এর কোনো বাড়তি কপি নামানো হবে না। workspace-এর প্রতিটি package-এর যে crate-গুলো `rand`
package ব্যবহার করে, তারা `rand`-এর সামঞ্জস্যপূর্ণ version উল্লেখ করলে Cargo নিশ্চিত করবে যে সবাই একই version ব্যবহার করছে, এতে জায়গা বাঁচে এবং
নিশ্চিত হয় যে workspace-এর crate-গুলো একে অপরের সঙ্গে সামঞ্জস্যপূর্ণ।

workspace-এর crate-গুলো একই dependency-র অসামঞ্জস্যপূর্ণ version উল্লেখ করলে Cargo প্রতিটির সমাধান করবে, তবে তবুও যত কম সম্ভব version ব্যবহারের
চেষ্টা করবে।

খেয়াল করুন, Cargo শুধু [Semantic Versioning]-এর নিয়মের মধ্যেই সামঞ্জস্য নিশ্চিত করে।
যেমন ধরুন, একটি workspace-এর একটি crate `rand` 0.8.0-এর ওপর নির্ভর করে, আর আরেকটি crate
`rand` 0.8.1-এর ওপর। semver-এর নিয়ম বলে 0.8.1 0.8.0-এর সঙ্গে সামঞ্জস্যপূর্ণ,
তাই দুটি crate-ই 0.8.1-এর ওপর (বা সম্ভবত আরও নতুন কোনো patch, যেমন 0.8.2-এর ওপর) নির্ভর করবে। কিন্তু যদি
একটি crate `rand` 0.7.0-এর ওপর আর আরেকটি `rand` 0.8.0-এর ওপর নির্ভর করে, তাহলে সেই version-গুলো semver অনুযায়ী অসামঞ্জস্যপূর্ণ।
তাই Cargo প্রতিটি crate-এর জন্য `rand`-এর আলাদা version ব্যবহার করবে।

### workspace-এ একটি test যোগ করা {#adding-a-test-to-a-workspace}

আরেকটি উন্নতি হিসেবে `add_one` crate-এর ভেতরে `add_one::add_one` function-এর একটি test যোগ করি:

<span class="filename">Filename: add_one/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add/add_one/src/lib.rs}}
```

এবার শীর্ষ স্তরের _add_ directory-তে `cargo test` চালান। এভাবে সাজানো একটি workspace-এ `cargo test` চালালে workspace-এর সব crate-এর test চলবে:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add
cargo test
copy output below; the output updating script doesn't handle subdirectories in
paths properly
-->

```console
$ cargo test
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running unittests src/lib.rs (target/debug/deps/add_one-93c49ee75dc46543)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/adder-3a47283c568d2b6a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

output-এর প্রথম অংশ দেখায় যে `add_one` crate-এর `it_works` test pass করেছে। পরের অংশ দেখায় যে `adder` crate-এ কোনো test পাওয়া যায়নি, আর শেষ অংশ
দেখায় যে `add_one` crate-এ কোনো documentation test পাওয়া যায়নি।

শীর্ষ স্তরের directory থেকে `-p` flag দিয়ে এবং যে crate test করতে চাই তার নাম উল্লেখ করে workspace-এর নির্দিষ্ট একটি crate-এর test-ও চালাতে পারি:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/add
cargo test -p add_one
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo test -p add_one
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/add_one-93c49ee75dc46543)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

এই output দেখায় যে `cargo test` শুধু `add_one` crate-এর test চালিয়েছে, `adder` crate-এর test চালায়নি।

workspace-এর crate-গুলো [crates.io](https://crates.io/)<!-- ignore -->-তে প্রকাশ করলে workspace-এর প্রতিটি crate আলাদাভাবে প্রকাশ করতে হবে।
`cargo test`-এর মতোই `-p` flag দিয়ে এবং যে crate প্রকাশ করতে চাই তার নাম উল্লেখ করে আমাদের workspace-এর নির্দিষ্ট একটি crate প্রকাশ করতে পারি।

বাড়তি অনুশীলনের জন্য `add_one` crate-এর মতো করে এই workspace-এ একটি `add_two` crate যোগ করুন!

আপনার project বড় হতে থাকলে workspace ব্যবহারের কথা ভাবুন: এতে একটি বড় কোডের স্তূপের বদলে ছোট, সহজে বোঝা যায় এমন অংশ নিয়ে কাজ করা যায়। তা ছাড়া
crate-গুলো প্রায়ই একসঙ্গে বদলালে সেগুলোকে একটি workspace-এ রাখলে crate-গুলোর মধ্যে সমন্বয় সহজ হয়।

{{#quiz ../quizzes/ch14-03-cargo-workspaces.toml}}

[Semantic Versioning]: https://semver.org/
