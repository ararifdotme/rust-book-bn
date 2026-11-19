## package ও crate {#packages-and-crates}

module system-এর যে অংশগুলো আমরা প্রথমে আলোচনা করব, সেগুলো হলো package আর crate।

একটি _crate_ (Rust-এর কোডের একক) হলো এক সময়ে Rust compiler যতটুকু কোড বিবেচনা করে, তার
ক্ষুদ্রতম একক। এমনকি আপনি যদি `cargo`-র বদলে `rustc` চালান এবং একটিমাত্র সোর্স কোড ফাইল দেন
(যেমনটি আমরা সেই অধ্যায় 1-এর [“Rust প্রোগ্রামের প্রাথমিক কথা”][basics]<!-- ignore
-->-তে করেছিলাম), compiler সেই ফাইলটিকেই একটি crate হিসেবে বিবেচনা করে। crate-এ module থাকতে পারে,
আর module-গুলো এমন অন্য ফাইলে সংজ্ঞায়িত থাকতে পারে, যেগুলো crate-এর সঙ্গে compile হয়, যা আমরা
পরের অংশগুলোতে দেখব।

একটি crate দুটি রূপের যেকোনো একটিতে থাকতে পারে: binary crate বা library crate।
_binary crate_ হলো এমন প্রোগ্রাম, যা compile করে চালানো যায় এমন একটি executable তৈরি করা যায়,
যেমন একটি কমান্ড লাইন প্রোগ্রাম বা একটি সার্ভার। প্রতিটির `main` নামে একটি function থাকতে হবে,
যা নির্ধারণ করে executable চললে কী ঘটবে। এ পর্যন্ত আমরা যত crate তৈরি করেছি, সবই binary crate।

_library crate_-এর কোনো `main` function থাকে না, আর এগুলো compile হয়ে executable হয় না। বরং এরা
এমন কার্যকারিতা সংজ্ঞায়িত করে, যা একাধিক প্রজেক্টের সঙ্গে ভাগ করে নেওয়ার উদ্দেশ্যে লেখা। যেমন,
[অধ্যায় 2][rand]<!-- ignore -->-এ আমরা যে `rand` crate ব্যবহার করেছিলাম, তা র‍্যান্ডম সংখ্যা তৈরির
কার্যকারিতা দেয়। বেশিরভাগ সময় Rustacean-রা “crate” বললে library crate বোঝান, আর “crate” শব্দটি
প্রোগ্রামিংয়ের সাধারণ ধারণা “library”-র সঙ্গে অদলবদল করে ব্যবহার করেন।

_crate root_ (crate-এর মূল ফাইল) হলো এমন একটি সোর্স ফাইল, যেখান থেকে Rust compiler শুরু করে এবং
যা আপনার crate-এর root module গঠন করে (module নিয়ে আমরা [“module দিয়ে scope আর privacy
নিয়ন্ত্রণ”][modules]<!-- ignore -->-এ বিস্তারিত ব্যাখ্যা করব)।

একটি _package_ হলো এক বা একাধিক crate-এর একটি বান্ডেল, যা কিছু কার্যকারিতা দেয়। একটি package-এ
একটি _Cargo.toml_ ফাইল থাকে, যা সেই crate-গুলো কীভাবে build করতে হবে তা বর্ণনা করে। Cargo নিজেও
আসলে একটি package, যাতে আছে সেই কমান্ড লাইন টুলের binary crate, যা দিয়ে আপনি এতক্ষণ কোড build
করছেন। Cargo package-এ একটি library crate-ও আছে, যার ওপর binary crate-টি নির্ভর করে। Cargo কমান্ড
লাইন টুল যে যুক্তি ব্যবহার করে, সেটিই ব্যবহার করতে অন্য প্রজেক্টগুলো Cargo library crate-এর ওপর
নির্ভর করতে পারে।

একটি package-এ যত খুশি binary crate থাকতে পারে, কিন্তু library crate থাকতে পারে সর্বোচ্চ একটি।
একটি package-এ অন্তত একটি crate থাকতেই হবে, তা library crate হোক বা binary crate।

চলুন দেখি একটি package তৈরি করলে কী ঘটে। প্রথমে আমরা `cargo new my-project` কমান্ডটি
লিখি:

```console
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

`cargo new my-project` চালানোর পর Cargo কী তৈরি করে তা দেখতে আমরা `ls` ব্যবহার করি।
_my-project_ ডিরেক্টরিতে একটি _Cargo.toml_ ফাইল আছে, যা আমাদের একটি package দেয়। একটি _src_
ডিরেক্টরিও আছে, যাতে আছে _main.rs_। টেক্সট এডিটরে _Cargo.toml_ খুলে লক্ষ করুন, এতে _src/main.rs_-এর
কোনো উল্লেখ নেই। Cargo একটি রীতি মেনে চলে: _src/main.rs_ হলো package-এর নামের সমান নামের একটি
binary crate-এর crate root। একইভাবে Cargo জানে, package ডিরেক্টরিতে _src/lib.rs_ থাকলে package-টিতে
package-এর নামের সমান নামের একটি library crate আছে, আর _src/lib.rs_ হলো তার crate root। library বা
binary build করতে Cargo crate root ফাইলগুলো `rustc`-কে দেয়।

এখানে আমাদের একটি package আছে, যাতে শুধু _src/main.rs_ আছে, অর্থাৎ এতে শুধু `my-project` নামের একটি
binary crate আছে। কোনো package-এ যদি _src/main.rs_ আর _src/lib.rs_ দুটোই থাকে, তাহলে এর দুটি crate
আছে: একটি binary আর একটি library, দুটোরই নাম package-এর নামের সমান। _src/bin_ ডিরেক্টরিতে ফাইল রেখে
একটি package-এ একাধিক binary crate রাখা যায়: প্রতিটি ফাইল হবে একটি আলাদা binary crate।

{{#quiz ../quizzes/ch07-01-packages-and-crates.toml}}

[basics]: ch01-02-hello-world.html#rust-program-basics
[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
[rand]: ch02-00-guessing-game-tutorial.html#generating-a-random-number
