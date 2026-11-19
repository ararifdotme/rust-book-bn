## release profile দিয়ে build নিজের মতো সাজানো {#customizing-builds-with-release-profiles}

Rust-এ _release profiles_ (release profile) হলো ভিন্ন ভিন্ন configuration-যুক্ত আগে থেকে সংজ্ঞায়িত, পরিবর্তনযোগ্য profile, যা একজন
প্রোগ্রামারকে কোড compile করার বিভিন্ন বিকল্পের ওপর আরও বেশি নিয়ন্ত্রণ দেয়। প্রতিটি profile অন্যগুলো থেকে স্বাধীনভাবে configure করা হয়।

Cargo-র দুটি প্রধান profile আছে: `dev` profile, যা Cargo ব্যবহার করে যখন আপনি `cargo
build` চালান, আর `release` profile, যা Cargo ব্যবহার করে যখন আপনি `cargo build
--release` চালান। `dev` profile development-এর জন্য ভালো default দিয়ে সংজ্ঞায়িত, আর `release` profile-এ release build-এর জন্য ভালো
default আছে।

এই profile-এর নামগুলো হয়তো আপনার build-এর output থেকে পরিচিত লাগবে:

<!-- manual-regeneration
anywhere, run:
cargo build
cargo build --release
and ensure output below is accurate
-->

```console
$ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
$ cargo build --release
    Finished `release` profile [optimized] target(s) in 0.32s
```

`dev` আর `release` হলো compiler-এর ব্যবহৃত এই ভিন্ন ভিন্ন profile।

project-এর _Cargo.toml_ ফাইলে আপনি স্পষ্টভাবে কোনো `[profile.*]` অংশ যোগ না করলে প্রতিটি profile-এর জন্য Cargo-র default setting প্রযোজ্য
হয়। যে profile নিজের মতো সাজাতে চান তার জন্য `[profile.*]` অংশ যোগ করে default setting-গুলোর যেকোনো অংশ override করতে পারেন। যেমন, `dev`
আর `release` profile-এর `opt-level` setting-এর default মানগুলো এই:

<span class="filename">Filename: Cargo.toml</span>

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

`opt-level` setting নিয়ন্ত্রণ করে Rust আপনার কোডে কতগুলো optimization প্রয়োগ করবে, যার পরিসর 0 থেকে 3। বেশি optimization প্রয়োগ করলে
compile-এর সময় বাড়ে, তাই development-এর সময় ঘন ঘন কোড compile করলে দ্রুত compile হওয়ার জন্য কম optimization চাইবেন, যদিও তাতে তৈরি কোড
ধীরে চলে। তাই `dev`-এর default `opt-level` হলো `0`। কোড release করার জন্য প্রস্তুত হলে compile করতে বেশি সময় দেওয়াই ভালো। release mode-এ
আপনি মাত্র একবার compile করবেন, কিন্তু compile করা প্রোগ্রাম চালাবেন অনেকবার, তাই release mode দ্রুত চলা কোডের বিনিময়ে বেশি compile সময় নেয়।
এ কারণেই `release` profile-এর default `opt-level` হলো `3`।

_Cargo.toml_-এ একটি default setting-এর জন্য ভিন্ন মান যোগ করে সেটি override করতে পারেন। যেমন, development profile-এ optimization level 1
ব্যবহার করতে চাইলে আমাদের project-এর _Cargo.toml_ ফাইলে এই দুটি লাইন যোগ করতে পারি:

<span class="filename">Filename: Cargo.toml</span>

```toml
[profile.dev]
opt-level = 1
```

এই কোড `0`-এর default setting override করে। এখন `cargo build` চালালে Cargo `dev` profile-এর default-গুলোর সঙ্গে আমাদের `opt-level`-এর
পরিবর্তনটি ব্যবহার করবে। `opt-level` আমরা `1` করেছি বলে Cargo default-এর চেয়ে বেশি optimization প্রয়োগ করবে, তবে release build-এর মতো অতগুলো
নয়।

প্রতিটি profile-এর configuration বিকল্প ও default-এর পূর্ণ তালিকার জন্য [Cargo-র documentation](https://doc.rust-lang.org/cargo/reference/profiles.html) দেখুন।

{{#quiz ../quizzes/ch14-01-release-profiles.toml}}