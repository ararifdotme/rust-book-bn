## `panic!` দিয়ে unrecoverable error {#unrecoverable-errors-with-panic}

কখনো কখনো আপনার কোডে খারাপ কিছু ঘটে, আর তা নিয়ে আপনার কিছুই করার থাকে না। এসব ক্ষেত্রের জন্য Rust-এ আছে
`panic!` macro। বাস্তবে panic ঘটানোর দুটি উপায় আছে: এমন কোনো কাজ করে, যার ফলে আমাদের কোড panic করে (যেমন একটি
array-র শেষের পরে পৌঁছানো), অথবা স্পষ্টভাবে `panic!` macro call করে। দুই ক্ষেত্রেই আমরা আমাদের প্রোগ্রামে একটি
panic ঘটাই। ডিফল্টভাবে এই panic-গুলো একটি ব্যর্থতার বার্তা প্রিন্ট করবে, unwind (stack ধরে ফিরে গিয়ে ডেটা পরিষ্কার)
করবে, stack পরিষ্কার করবে, আর বন্ধ হয়ে যাবে। একটি environment variable (অপারেটিং সিস্টেমে রাখা নামযুক্ত সেটিং-মান)-এর
মাধ্যমে আপনি panic ঘটলে Rust-কে দিয়ে call stack (call হওয়া function-গুলোর ক্রম) দেখাতে পারেন, যাতে panic-এর উৎস
খুঁজে বের করা সহজ হয়।

> ### panic-এর জবাবে stack unwind করা বা abort করা {#unwinding-the-stack-or-aborting-in-response-to-a-panic}
>
> ডিফল্টভাবে panic ঘটলে প্রোগ্রাম _unwinding_ শুরু করে, অর্থাৎ Rust stack ধরে ওপরের দিকে ফিরে যায় এবং পথে পাওয়া
> প্রতিটি function-এর ডেটা পরিষ্কার করে। তবে ফিরে গিয়ে পরিষ্কার করা অনেক কাজের। তাই Rust আপনাকে বিকল্প হিসেবে সঙ্গে
> সঙ্গে _aborting_ (পরিষ্কার না করেই প্রোগ্রাম বন্ধ করা) বেছে নিতে দেয়, যা কোনো পরিষ্কার ছাড়াই প্রোগ্রাম শেষ করে দেয়।
>
> তখন প্রোগ্রাম যে মেমোরি ব্যবহার করছিল, তা অপারেটিং সিস্টেমকে পরিষ্কার করতে হবে। আপনার প্রজেক্টে যদি তৈরি হওয়া
> binary যতটা সম্ভব ছোট রাখতে হয়, তাহলে আপনার _Cargo.toml_ ফাইলের উপযুক্ত `[profile]` অংশে `panic = 'abort'` যোগ
> করে panic-এর ক্ষেত্রে unwind থেকে abort-এ যেতে পারেন। যেমন, release mode-এ panic হলে abort করতে চাইলে এটি যোগ
> করুন:
>
> ```toml
> [profile.release]
> panic = 'abort'
> ```

চলুন একটি সহজ প্রোগ্রামে `panic!` call করে দেখি:

<Listing file-name="src/main.rs">

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-01-panic/src/main.rs}}
```

</Listing>

প্রোগ্রামটি চালালে এরকম কিছু দেখবেন:

```console
{{#include ../listings/ch09-error-handling/no-listing-01-panic/output.txt}}
```

`panic!` call-এর কারণেই শেষ দুই লাইনের error message এসেছে। প্রথম লাইনটি দেখায় আমাদের panic বার্তা আর আমাদের
সোর্স কোডের কোন জায়গায় panic ঘটেছে: _src/main.rs:2:5_ বোঝায় যে এটি আমাদের _src/main.rs_ ফাইলের দ্বিতীয় লাইনের
পঞ্চম অক্ষর।

এই ক্ষেত্রে নির্দেশিত লাইনটি আমাদের কোডেরই অংশ, আর সেই লাইনে গেলে আমরা `panic!` macro call দেখতে পাই। অন্য
ক্ষেত্রে `panic!` call থাকতে পারে এমন কোডে, যা আমাদের কোড call করে, আর তখন error message-এ জানানো ফাইলের নাম ও
লাইন নম্বর হবে অন্য কারও কোডের, যেখানে `panic!` macro call করা হয়েছে, আমাদের কোডের সেই লাইন নয়, যা শেষ পর্যন্ত
`panic!` call-এর দিকে নিয়ে গেছে।

<!-- Old headings. Do not remove or links may break. -->

<a id="using-a-panic-backtrace"></a>

`panic!` call যে function-গুলো থেকে এসেছে, সেগুলোর backtrace (এই পর্যন্ত call হওয়া সব function-এর তালিকা) ব্যবহার
করে আমরা খুঁজে বের করতে পারি আমাদের কোডের কোন অংশ সমস্যা ঘটাচ্ছে। একটি `panic!` backtrace কীভাবে ব্যবহার করতে হয় তা
বুঝতে চলুন আরেকটি উদাহরণ দেখি, যেখানে `panic!` call আমাদের কোড সরাসরি macro call করার কারণে নয়, বরং আমাদের কোডের
একটি বাগের কারণে একটি library থেকে আসে। Listing 9-1-এ এমন কিছু কোড আছে, যা একটি vector-এ বৈধ index-এর পরিসরের বাইরের
একটি index-এ পৌঁছানোর চেষ্টা করে।

<Listing number="9-1" file-name="src/main.rs" caption="একটি vector-এর শেষের পরের একটি উপাদানে পৌঁছানোর চেষ্টা, যা একটি `panic!` call ঘটাবে">

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-01/src/main.rs}}
```

</Listing>

এখানে আমরা আমাদের vector-এর 100তম উপাদানে পৌঁছানোর চেষ্টা করছি (যা আছে index 99-এ, কারণ index শূন্য থেকে শুরু হয়),
কিন্তু vector-এ মাত্র তিনটি উপাদান আছে। এই পরিস্থিতিতে Rust panic করবে। `[]` ব্যবহার করলে একটি উপাদান return করার
কথা, কিন্তু অবৈধ index দিলে এখানে Rust এমন কোনো উপাদান return করতে পারে না, যা সঠিক হতো।

C-তে একটি data structure-এর শেষের পরে পড়ার চেষ্টা করা undefined behavior। data structure-এ ওই উপাদানের সংশ্লিষ্ট
মেমোরি-অবস্থানে যা-ই থাকুক, আপনি হয়তো তা-ই পেয়ে যাবেন, যদিও সেই মেমোরি ওই structure-এর নয়। একে বলা হয় _buffer
overread_ (বাফারের সীমার বাইরে পড়া), আর আক্রমণকারী যদি index এমনভাবে নিয়ন্ত্রণ করতে পারে, যাতে data structure-এর
পরে রাখা এমন ডেটা পড়া যায়, যা তার পড়ার অনুমতি নেই, তাহলে এটি নিরাপত্তা-দুর্বলতার দিকে নিয়ে যেতে পারে।

আপনার প্রোগ্রামকে এ ধরনের দুর্বলতা থেকে রক্ষা করতে, অস্তিত্ব নেই এমন একটি index-এর উপাদান পড়ার চেষ্টা করলে Rust
প্রোগ্রাম চলা থামিয়ে দেবে এবং চালিয়ে যেতে রাজি হবে না। চলুন চেষ্টা করে দেখি:

```console
{{#include ../listings/ch09-error-handling/listing-09-01/output.txt}}
```

এই error আমাদের _main.rs_-এর লাইন 4-কে নির্দেশ করে, যেখানে আমরা `v`-এর vector-এর index 99-এ পৌঁছানোর চেষ্টা
করেছি।

`note:` লাইনটি আমাদের জানায় যে error-টি ঘটার পেছনে ঠিক কী ঘটেছে তার backtrace পেতে আমরা `RUST_BACKTRACE`
environment variable সেট করতে পারি। একটি _backtrace_ হলো এই পর্যায়ে পৌঁছাতে যত function call হয়েছে, তাদের সবগুলোর
একটি তালিকা। Rust-এ backtrace অন্য ভাষার মতোই কাজ করে: backtrace পড়ার মূল কৌশল হলো ওপর থেকে শুরু করে পড়তে থাকা,
যতক্ষণ না আপনার লেখা ফাইল দেখতে পান। সেটিই সেই জায়গা, যেখান থেকে সমস্যার শুরু। ওই জায়গার ওপরের লাইনগুলো হলো সেই
কোড, যা আপনার কোড call করেছে; নিচের লাইনগুলো হলো সেই কোড, যা আপনার কোডকে call করেছে। এই আগে-পরের লাইনগুলোতে থাকতে
পারে Rust-এর মূল কোড, standard library-র কোড, বা আপনার ব্যবহৃত crate। চলুন `RUST_BACKTRACE` environment variable-কে
`0` ছাড়া যেকোনো মানে সেট করে একটি backtrace পাওয়ার চেষ্টা করি। Listing 9-2-এ আপনি যা দেখবেন, তার মতো আউটপুট দেখানো
হয়েছে।

<!-- manual-regeneration
cd listings/ch09-error-handling/listing-09-01
RUST_BACKTRACE=1 cargo run
copy the backtrace output below
check the backtrace number mentioned in the text below the listing
-->

<Listing number="9-2" caption="environment variable `RUST_BACKTRACE` সেট করা থাকলে একটি `panic!` call-এর তৈরি backtrace দেখানো হয়">

```console
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at src/main.rs:4:6:
index out of bounds: the len is 3 but the index is 99
stack backtrace:
   0: rust_begin_unwind
             at /rustc/4d91de4e48198da2e33413efdcd9cd2cc0c46688/library/std/src/panicking.rs:692:5
   1: core::panicking::panic_fmt
             at /rustc/4d91de4e48198da2e33413efdcd9cd2cc0c46688/library/core/src/panicking.rs:75:14
   2: core::panicking::panic_bounds_check
             at /rustc/4d91de4e48198da2e33413efdcd9cd2cc0c46688/library/core/src/panicking.rs:273:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/slice/index.rs:274:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/slice/index.rs:16:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3361:9
   6: panic::main
             at ./src/main.rs:4:6
   7: core::ops::function::FnOnce::call_once
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

</Listing>

অনেক আউটপুট! আপনার অপারেটিং সিস্টেম আর Rust-এর সংস্করণ অনুযায়ী আপনি যে আউটপুট দেখবেন, তা হুবহু আলাদা হতে পারে।
এই তথ্যসহ backtrace পেতে debug symbol (ডিবাগের জন্য প্রোগ্রামে রাখা বাড়তি তথ্য) চালু থাকতে হবে। `--release` flag ছাড়া
`cargo build` বা `cargo run` ব্যবহার করলে, যেমনটি আমরা এখানে করেছি, debug symbol ডিফল্টভাবেই চালু থাকে।

Listing 9-2-এর আউটপুটে backtrace-এর লাইন 6 আমাদের প্রজেক্টের সেই লাইনকে নির্দেশ করে, যা সমস্যা ঘটাচ্ছে:
_src/main.rs_-এর লাইন 4। আমরা যদি না চাই আমাদের প্রোগ্রাম panic করুক, তাহলে আমাদের অনুসন্ধান শুরু করা উচিত আমাদের লেখা
একটি ফাইলের উল্লেখ থাকা প্রথম লাইনটি যে জায়গাকে নির্দেশ করে, সেখান থেকে। Listing 9-1-এ, যেখানে আমরা ইচ্ছে করে এমন কোড
লিখেছি যা panic করবে, panic ঠিক করার উপায় হলো vector-এর index-এর পরিসরের বাইরের কোনো উপাদান না চাওয়া। ভবিষ্যতে আপনার
কোড panic করলে আপনাকে খুঁজে বের করতে হবে, কোড কোন মান দিয়ে কী কাজ করার ফলে panic ঘটছে, আর এর বদলে কোডের কী করা
উচিত।

এই অধ্যায়ের পরের দিকে [“`panic!` করব নাকি
করব না”][to-panic-or-not-to-panic]<!-- ignore --> অংশে আমরা `panic!`-এ ফিরে আসব, আর দেখব error-এর পরিস্থিতি সামলাতে
কখন `panic!` ব্যবহার করা উচিত আর কখন উচিত নয়। এরপর আমরা দেখব `Result` ব্যবহার করে একটি error থেকে কীভাবে সামলে
ওঠা যায়।

{{#quiz ../quizzes/ch09-01-panic.toml}}

[to-panic-or-not-to-panic]: ch09-03-to-panic-or-not-to-panic.html#to-panic-or-not-to-panic
