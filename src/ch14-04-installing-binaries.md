<!-- Old headings. Do not remove or links may break. -->

<a id="installing-binaries-from-cratesio-with-cargo-install"></a>

## `cargo install` দিয়ে binary install করা {#installing-binaries-with-cargo-install}

`cargo install` command দিয়ে binary crate স্থানীয়ভাবে install করে ব্যবহার করা যায়। এটি system package-এর বিকল্প হওয়ার জন্য নয়; বরং
[crates.io](https://crates.io/)<!-- ignore -->-তে অন্যদের ভাগ করা tool Rust ডেভেলপারদের সহজে install করার একটি সুবিধাজনক উপায়। খেয়াল করুন, শুধু
binary target আছে এমন package-ই install করা যায়। _binary target_ হলো চালানো যায় এমন প্রোগ্রাম, যা তৈরি হয় যদি crate-এ একটি _src/main.rs_ ফাইল
বা binary হিসেবে উল্লেখ করা অন্য কোনো ফাইল থাকে; এর বিপরীতে library target নিজে নিজে চালানো যায় না, তবে অন্য প্রোগ্রামের ভেতরে যুক্ত করার উপযোগী।
সাধারণত crate-এর README ফাইলে তথ্য থাকে যে crate-টি একটি library, নাকি এর binary target আছে, নাকি দুটিই।

`cargo install` দিয়ে install করা সব binary installation root-এর _bin_ folder-এ রাখা হয়। আপনি যদি _rustup.rs_ দিয়ে Rust install করে থাকেন এবং কোনো
নিজস্ব configuration না থাকে, তাহলে এই directory হবে *$HOME/.cargo/bin*। `cargo install` দিয়ে install করা প্রোগ্রাম চালাতে পারতে নিশ্চিত করুন যে
এই directory আপনার `$PATH`-এ আছে।

যেমন, অধ্যায় 12-এ আমরা উল্লেখ করেছিলাম যে ফাইলে খোঁজার জন্য `grep` tool-এর একটি Rust implementation আছে, যার নাম `ripgrep`। `ripgrep` install
করতে এটি চালাতে পারি:

<!-- manual-regeneration
cargo install something you don't have, copy relevant output below
-->

```console
$ cargo install ripgrep
    Updating crates.io index
  Downloaded ripgrep v14.1.1
  Downloaded 1 crate (213.6 KB) in 0.40s
  Installing ripgrep v14.1.1
--snip--
   Compiling grep v0.3.2
    Finished `release` profile [optimized + debuginfo] target(s) in 6.73s
  Installing ~/.cargo/bin/rg
   Installed package `ripgrep v14.1.1` (executable `rg`)
```

output-এর শেষের আগের লাইন install করা binary-র অবস্থান আর নাম দেখায়, যা `ripgrep`-এর ক্ষেত্রে `rg`। আগে যেমন বলা হয়েছে, installation directory
আপনার `$PATH`-এ থাকলে আপনি `rg --help` চালিয়ে ফাইলে খোঁজার জন্য আরও দ্রুত, আরও Rust-সুলভ একটি tool ব্যবহার শুরু করতে পারেন!
