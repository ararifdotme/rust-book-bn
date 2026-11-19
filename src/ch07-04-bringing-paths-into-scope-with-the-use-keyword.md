## `use` keyword দিয়ে path-কে scope-এ আনা {#bringing-paths-into-scope-with-the-use-keyword}

function call করতে path লিখে যেতে হওয়াটা অসুবিধাজনক আর একঘেয়ে লাগতে পারে। Listing 7-7-এ
`add_to_waitlist` function-এর জন্য absolute বা relative যে path-ই বেছে নিই না কেন, যতবার
`add_to_waitlist` call করতে চেয়েছি, ততবার `front_of_house` আর `hosting`-ও লিখতে হয়েছে। সৌভাগ্যক্রমে,
এই প্রক্রিয়া সহজ করার একটি উপায় আছে: `use` keyword দিয়ে একবার একটি path-এর শর্টকাট তৈরি করে scope-এর
বাকি সবখানে ছোট নামটি ব্যবহার করতে পারি।

Listing 7-11-এ আমরা `crate::front_of_house::hosting` module-টিকে `eat_at_restaurant` function-এর
scope-এ আনি, যাতে `eat_at_restaurant`-এ `add_to_waitlist` function call করতে শুধু
`hosting::add_to_waitlist` লিখলেই চলে।

<Listing number="7-11" file-name="src/lib.rs" caption="`use` দিয়ে একটি module-কে scope-এ আনা">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-11/src/lib.rs}}
```

</Listing>

একটি scope-এ `use` আর একটি path যোগ করা ফাইল সিস্টেমে একটি symbolic link (ফাইল বা ডিরেক্টরির
শর্টকাট-লিংক) তৈরির মতো। crate root-এ `use crate::front_of_house::hosting` যোগ করায় `hosting` এখন ওই
scope-এ একটি বৈধ নাম, ঠিক যেন `hosting` module crate root-এই সংজ্ঞায়িত হয়েছিল। `use` দিয়ে scope-এ
আনা path-ও অন্য যেকোনো path-এর মতো privacy যাচাই করে।

লক্ষ করুন, `use` শুধু সেই নির্দিষ্ট scope-এর জন্যই শর্টকাট তৈরি করে, যেখানে `use` আছে। Listing 7-12-এ
`eat_at_restaurant` function-কে `customer` নামের একটি নতুন child module-এ সরানো হয়েছে, যা তখন `use`
statement-এর থেকে আলাদা একটি scope, তাই function body compile হবে না।

<Listing number="7-12" file-name="src/lib.rs" caption="একটি `use` statement শুধু সেই scope-এই প্রযোজ্য, যেখানে এটি আছে।">

```rust,noplayground,test_harness,does_not_compile,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-12/src/lib.rs}}
```

</Listing>

compiler error দেখায় যে `customer` module-এর ভেতরে শর্টকাটটি আর প্রযোজ্য
নয়:

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-12/output.txt}}
```

লক্ষ করুন, একটি সতর্কবার্তাও আছে যে `use`-টি তার scope-এ আর ব্যবহার হচ্ছে না! এই সমস্যা ঠিক করতে `use`-টিকেও
`customer` module-এর ভেতরে সরিয়ে নিন, অথবা child `customer` module-এর ভেতর থেকে `super::hosting` দিয়ে
parent module-এর শর্টকাটটি উল্লেখ করুন।

### Rust-সুলভ `use` path তৈরি করা {#creating-idiomatic-use-paths}

Listing 7-11-এ আপনি হয়তো ভেবেছেন, Listing 7-13-এর মতো একই ফল পেতে `use` path-টি একেবারে
`add_to_waitlist` function পর্যন্ত লেখার বদলে আমরা কেন `use
crate::front_of_house::hosting` লিখে তারপর `eat_at_restaurant`-এ `hosting::add_to_waitlist` call করলাম।

<Listing number="7-13" file-name="src/lib.rs" caption="`use` দিয়ে `add_to_waitlist` function-কে scope-এ আনা, যা Rust-সুলভ নয়">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-13/src/lib.rs}}
```

</Listing>

Listing 7-11 আর Listing 7-13 দুটোই একই কাজ করলেও `use` দিয়ে একটি function-কে scope-এ আনার Rust-সুলভ
উপায় হলো Listing
7-11। `use` দিয়ে function-এর parent module-কে scope-এ আনা মানে function call করার সময় আমাদের parent
module লিখতে হবে। function call করার সময় parent module লিখলে স্পষ্ট হয় যে function-টি লোকালভাবে
সংজ্ঞায়িত নয়, আবার পুরো path বারবার লেখাও কম হয়। Listing 7-13-এর কোডে `add_to_waitlist` কোথায়
সংজ্ঞায়িত, তা অস্পষ্ট।

অন্যদিকে, `use` দিয়ে struct, enum আর অন্যান্য item আনার সময় পুরো path লেখাই Rust-সুলভ। Listing 7-14-এ
একটি binary crate-এর scope-এ standard library-র `HashMap` struct আনার Rust-সুলভ উপায় দেখানো হয়েছে।

<Listing number="7-14" file-name="src/main.rs" caption="Rust-সুলভ উপায়ে `HashMap`-কে scope-এ আনা">

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-14/src/main.rs}}
```

</Listing>

এই idiom (Rust-এর প্রচলিত লেখার ধরন)-এর পেছনে জোরালো কোনো কারণ নেই: এটি শুধু একটি রীতি, যা গড়ে উঠেছে,
আর মানুষজন এভাবে Rust কোড পড়তে ও লিখতে অভ্যস্ত হয়ে গেছেন।

এই idiom-এর ব্যতিক্রম হলো, যদি আমরা `use` statement দিয়ে একই নামের দুটি item scope-এ আনি, কারণ Rust
তা করতে দেয় না। Listing 7-15-এ দেখানো হয়েছে, একই নামের কিন্তু আলাদা parent module-এর দুটি `Result`
type কীভাবে scope-এ আনতে হয় এবং কীভাবে সেগুলো উল্লেখ করতে হয়।

<Listing number="7-15" file-name="src/lib.rs" caption="একই নামের দুটি type একই scope-এ আনতে তাদের parent module ব্যবহার করতে হয়।">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-15/src/lib.rs:here}}
```

</Listing>

দেখতেই পাচ্ছেন, parent module ব্যবহার করে দুটি `Result` type আলাদা করা যায়। এর বদলে আমরা যদি
`use std::fmt::Result` আর `use std::io::Result` লিখতাম, তাহলে একই scope-এ দুটি `Result` type থাকত, আর
`Result` ব্যবহার করলে আমরা কোনটি বোঝাচ্ছি, Rust তা জানত না।

### `as` keyword দিয়ে নতুন নাম দেওয়া {#providing-new-names-with-the-as-keyword}

`use` দিয়ে একই নামের দুটি type একই scope-এ আনার সমস্যার আরেকটি সমাধান আছে: path-এর পরে আমরা `as`
আর type-টির জন্য একটি নতুন লোকাল নাম, বা _alias_ (বিকল্প নাম), লিখতে পারি। Listing 7-16-এ `as` ব্যবহার
করে দুটি `Result` type-এর একটির নাম বদলে Listing 7-15-এর কোড লেখার আরেকটি উপায় দেখানো হয়েছে।

<Listing number="7-16" file-name="src/lib.rs" caption="scope-এ আনার সময় `as` keyword দিয়ে একটি type-এর নাম বদলানো">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-16/src/lib.rs:here}}
```

</Listing>

দ্বিতীয় `use` statement-এ আমরা `std::io::Result` type-এর জন্য নতুন নাম `IoResult` বেছে নিয়েছি, যা
`std::fmt`-এর `Result`-এর সঙ্গে সংঘাত ঘটাবে না, যেটিও আমরা scope-এ এনেছি। Listing 7-15 আর Listing
7-16 দুটোই Rust-সুলভ বলে গণ্য, তাই বেছে নেওয়া আপনার ওপর!

### `pub use` দিয়ে নাম re-export করা {#re-exporting-names-with-pub-use}

`use` keyword দিয়ে কোনো নাম scope-এ আনলে নামটি সেই scope-এর কাছে private থাকে, যেখানে আমরা এটি
import করেছি। ওই scope-এর বাইরের কোডকে নামটি এমনভাবে উল্লেখ করতে দিতে, যেন এটি ওই scope-এই সংজ্ঞায়িত,
আমরা `pub` আর `use` মিলিয়ে ব্যবহার করতে পারি। এই কৌশলকে বলা হয় _re-exporting_ (আনা নামকে আবার বাইরে
প্রকাশ করা), কারণ আমরা একটি item-কে scope-এ আনছি, আবার সেই item-কে অন্যদের নিজেদের scope-এ আনার জন্যও
উন্মুক্ত করছি।

Listing 7-17-এ Listing 7-11-এর কোডটি দেখানো হয়েছে, যেখানে root module-এর `use` বদলে `pub use` করা
হয়েছে।

<Listing number="7-17" file-name="src/lib.rs" caption="`pub use` দিয়ে একটি নামকে নতুন scope থেকে যেকোনো কোডের ব্যবহারের জন্য উন্মুক্ত করা">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-17/src/lib.rs}}
```

</Listing>

এই পরিবর্তনের আগে বাইরের কোডকে `add_to_waitlist` function call করতে হতো
`restaurant::front_of_house::hosting::add_to_waitlist()` path ব্যবহার করে, যার জন্য `front_of_house`
module-কেও `pub` হিসেবে চিহ্নিত করতে হতো। এখন এই `pub
use` root module থেকে `hosting` module-টি re-export করায় বাইরের কোড এর বদলে
`restaurant::hosting::add_to_waitlist()` path ব্যবহার করতে পারে।

re-export কাজে আসে, যখন আপনার কোডের ভেতরের কাঠামো আপনার কোড call করা প্রোগ্রামাররা বিষয়টি যেভাবে
ভাবেন, তার থেকে আলাদা। যেমন, এই রেস্তোরাঁর রূপকে রেস্তোরাঁ চালানো মানুষেরা “front of house” আর “back
of house” হিসেবে ভাবেন। কিন্তু রেস্তোরাঁয় আসা খদ্দেররা সম্ভবত রেস্তোরাঁর অংশগুলোকে এই শব্দে ভাবেন না।
`pub
use` দিয়ে আমরা এক কাঠামোয় কোড লিখে ভিন্ন একটি কাঠামো প্রকাশ করতে পারি। এতে আমাদের library, library-তে
কাজ করা প্রোগ্রামার আর library call করা প্রোগ্রামার—দুই দলের জন্যই সুসংগঠিত হয়। `pub use`-এর আরেকটি
উদাহরণ আর এটি কীভাবে আপনার crate-এর ডকুমেন্টেশনকে প্রভাবিত করে, তা আমরা অধ্যায় 14-এর [“সুবিধাজনক
public API export করা”][ch14-pub-use]<!-- ignore --> অংশে দেখব।

### বাইরের package ব্যবহার করা {#using-external-packages}

অধ্যায় 2-এ আমরা একটি সংখ্যা আন্দাজের খেলার প্রজেক্ট প্রোগ্রাম করেছিলাম, যা র‍্যান্ডম সংখ্যা পেতে
`rand` নামের একটি বাইরের package ব্যবহার করেছিল। আমাদের প্রজেক্টে `rand` ব্যবহার করতে আমরা
_Cargo.toml_-এ এই লাইনটি যোগ করেছিলাম:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch14-03-cargo-workspaces.md
-->

<Listing file-name="Cargo.toml">

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:9:}}
```

</Listing>

_Cargo.toml_-এ `rand`-কে dependency হিসেবে যোগ করলে Cargo-কে বলা হয় [crates.io](https://crates.io/)
থেকে `rand` package আর এর সব dependency ডাউনলোড করে `rand`-কে আমাদের প্রজেক্টের জন্য উন্মুক্ত করতে।

তারপর আমাদের package-এর scope-এ `rand`-এর সংজ্ঞাগুলো আনতে আমরা crate-এর নাম `rand` দিয়ে শুরু হওয়া
একটি `use` লাইন যোগ করেছিলাম, আর যে item-গুলো scope-এ আনতে চেয়েছিলাম, সেগুলো তালিকাভুক্ত করেছিলাম।
মনে করুন, অধ্যায় 2-এর [“একটি র‍্যান্ডম সংখ্যা তৈরি
করা”][rand]<!-- ignore --> অংশে আমরা `Rng` trait-কে scope-এ এনে `rand::thread_rng` function call
করেছিলাম:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:ch07-04}}
```

Rust কমিউনিটির সদস্যরা [crates.io](https://crates.io/)-তে অনেক package উন্মুক্ত করেছেন, আর সেগুলোর
যেকোনোটি আপনার package-এ আনতে এই একই ধাপগুলো অনুসরণ করতে হয়: আপনার package-এর _Cargo.toml_ ফাইলে
সেগুলো তালিকাভুক্ত করা, আর তাদের crate থেকে item scope-এ আনতে `use` ব্যবহার করা।

লক্ষ করুন, standard `std` library-ও একটি crate, যা আমাদের package-এর বাইরের। standard library যেহেতু
Rust ভাষার সঙ্গেই আসে, তাই `std` যোগ করতে আমাদের _Cargo.toml_ বদলাতে হয় না। কিন্তু সেখান থেকে item
আমাদের package-এর scope-এ আনতে `use` দিয়ে একে উল্লেখ করতে হয়। যেমন, `HashMap`-এর জন্য আমরা এই লাইনটি
ব্যবহার করতাম:

```rust
use std::collections::HashMap;
```

এটি একটি absolute path, যা শুরু হয়েছে standard library crate-এর নাম `std` দিয়ে।

<!-- Old headings. Do not remove or links may break. -->

<a id="using-nested-paths-to-clean-up-large-use-lists"></a>

### nested path দিয়ে `use` তালিকা পরিষ্কার করা {#using-nested-paths-to-clean-up-use-lists}

একই crate বা একই module-এ সংজ্ঞায়িত একাধিক item ব্যবহার করলে প্রতিটি item আলাদা লাইনে লিখলে আমাদের
ফাইলে অনেকটা খাড়া জায়গা নিতে পারে। যেমন, Listing 2-4-এর সংখ্যা আন্দাজের খেলায় আমাদের এই দুটি `use`
statement `std` থেকে item scope-এ আনে:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-01-use-std-unnested/src/main.rs:here}}
```

</Listing>

এর বদলে আমরা nested path (একই শুরুর অংশের পথগুলো `{}` দিয়ে একত্র করা) ব্যবহার করে একই item-গুলো এক
লাইনেই scope-এ আনতে পারি। এটি করতে path-এর সাধারণ অংশটি লিখি, তারপর দুটি কোলন, আর তারপর কার্লি
ব্র্যাকেটের ভেতরে path-গুলোর আলাদা অংশগুলোর একটি তালিকা, Listing 7-18-এ যেমন দেখানো হয়েছে।

<Listing number="7-18" file-name="src/main.rs" caption="একই উপসর্গের একাধিক item scope-এ আনতে একটি nested path লেখা">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-18/src/main.rs:here}}
```

</Listing>

বড় প্রোগ্রামে nested path ব্যবহার করে একই crate বা module থেকে অনেক item scope-এ আনলে দরকারি আলাদা
`use` statement-এর সংখ্যা অনেক কমে যেতে পারে!

path-এর যেকোনো স্তরে nested path ব্যবহার করা যায়, যা কাজে আসে যখন একটি উপ-path ভাগ করে এমন দুটি `use`
statement মেলাতে হয়। যেমন, Listing 7-19-এ দুটি `use` statement দেখানো হয়েছে: একটি `std::io`-কে scope-এ
আনে, আর অন্যটি `std::io::Write`-কে।

<Listing number="7-19" file-name="src/lib.rs" caption="দুটি `use` statement, যেখানে একটি অন্যটির উপ-path">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-19/src/lib.rs}}
```

</Listing>

এই দুটি path-এর সাধারণ অংশ হলো `std::io`, আর সেটিই প্রথম path-টির পুরোটা। এই দুটি path-কে একটি `use`
statement-এ মেলাতে আমরা nested path-এ `self` ব্যবহার করতে পারি, Listing 7-20-এ যেমন দেখানো হয়েছে।

<Listing number="7-20" file-name="src/lib.rs" caption="Listing 7-19-এর path-গুলোকে একটি `use` statement-এ মেলানো">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-20/src/lib.rs}}
```

</Listing>

এই লাইনটি `std::io` আর `std::io::Write` দুটোকেই scope-এ আনে।

<!-- Old headings. Do not remove or links may break. -->

<a id="the-glob-operator"></a>

### glob operator দিয়ে item import করা {#importing-items-with-the-glob-operator}

একটি path-এ সংজ্ঞায়িত _সব_ public item scope-এ আনতে চাইলে সেই path-এর পরে `*` glob operator (`*` দিয়ে
সব public item একসঙ্গে আনার চিহ্ন) লিখতে পারি:

```rust
use std::collections::*;
```

এই `use` statement `std::collections`-এ সংজ্ঞায়িত সব public item বর্তমান scope-এ আনে। glob operator
ব্যবহারের সময় সতর্ক থাকুন! glob ব্যবহার করলে কোন কোন নাম scope-এ আছে, আর আপনার প্রোগ্রামে ব্যবহৃত
একটি নাম কোথায় সংজ্ঞায়িত, তা বোঝা কঠিন হয়ে যেতে পারে। এছাড়া dependency তার সংজ্ঞা বদলালে আপনি যা
import করেছেন তা-ও বদলে যায়, যার ফলে dependency আপগ্রেড করলে compiler error হতে পারে, যেমন যদি
dependency এমন একটি সংজ্ঞা যোগ করে, যার নাম একই scope-এ আপনার একটি সংজ্ঞার নামের সমান।

glob operator প্রায়ই testing-এর সময় ব্যবহার হয়, পরীক্ষাধীন সবকিছু `tests` module-এ আনতে; এ নিয়ে আমরা
অধ্যায় 11-এর [“test কীভাবে
লিখতে হয়”][writing-tests]<!-- ignore --> অংশে কথা বলব। glob operator কখনো কখনো prelude ধরনের অংশ
হিসেবেও ব্যবহার হয়: এই ধরন সম্পর্কে আরও জানতে [standard library-র
ডকুমেন্টেশন](https://doc.rust-lang.org/std/prelude/index.html#other-preludes)<!-- ignore --> দেখুন।

{{#quiz ../quizzes/ch07-04-use.toml}}

[ch14-pub-use]: ch14-02-publishing-to-crates-io.html#exporting-a-convenient-public-api-with-pub-use
[rand]: ch02-00-guessing-game-tutorial.html#generating-a-random-number
[writing-tests]: ch11-01-writing-tests.html#how-to-write-tests
