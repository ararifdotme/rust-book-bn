## আমাদের I/O project উন্নত করা {#improving-our-io-project}

iterator সম্পর্কে এই নতুন জ্ঞান দিয়ে আমরা অধ্যায় 12-এর I/O project উন্নত করতে পারি, iterator ব্যবহার করে কোডের কিছু জায়গা আরও পরিষ্কার ও
সংক্ষিপ্ত করে। চলুন দেখি iterator কীভাবে আমাদের `Config::build` function আর `search` function-এর implementation উন্নত করতে পারে।

### iterator দিয়ে একটি `clone` সরানো {#removing-a-clone-using-an-iterator}

Listing 12-6-এ আমরা এমন কোড যোগ করেছিলাম যা `String` মানের একটি slice নিয়ে, slice-এ index করে এবং মানগুলো clone করে `Config` struct-এর
একটি instance তৈরি করত, যাতে `Config` struct সেই মানগুলোর মালিক হতে পারে। Listing 13-17-এ আমরা `Config::build` function-এর implementation
আবার দিয়েছি, যেমনটি Listing 12-23-এ ছিল।

<Listing number="13-17" file-name="src/main.rs" caption="Listing 12-23-এর `Config::build` function আবার দেওয়া হলো">

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-23-reproduced/src/main.rs:ch13}}
```

</Listing>

তখন আমরা বলেছিলাম অদক্ষ `clone` call নিয়ে চিন্তা না করতে, কারণ ভবিষ্যতে এগুলো সরিয়ে দেব। সেই সময় এখন এসে গেছে!

এখানে `clone` দরকার ছিল, কারণ `args` parameter-এ আমাদের `String` উপাদানের একটি slice আছে, কিন্তু `build` function `args`-এর মালিক নয়।
একটি `Config` instance-এর ownership return করতে আমাদের `Config`-এর `query` আর `file_path` field-এর মানগুলো clone করতে হয়েছিল, যাতে `Config`
instance নিজের মানগুলোর মালিক হতে পারে।

iterator সম্পর্কে নতুন জ্ঞান দিয়ে আমরা `build` function বদলে দিতে পারি, যাতে এটি একটি slice borrow না করে argument হিসেবে একটি iterator-এর
ownership নেয়। slice-এর দৈর্ঘ্য যাচাই আর নির্দিষ্ট জায়গায় index করার কোডের বদলে আমরা iterator-এর কার্যক্ষমতা ব্যবহার করব। এতে `Config::build`
function কী করছে তা আরও পরিষ্কার হবে, কারণ মানগুলো নেবে iterator।

`Config::build` একবার iterator-এর ownership নিয়ে নিলে এবং borrow করে এমন index operation ব্যবহার বন্ধ করলে, `clone` call করে নতুন allocation
না করে আমরা iterator থেকে `String` মানগুলো সরাসরি `Config`-এ সরিয়ে নিতে পারব।

#### return করা iterator সরাসরি ব্যবহার করা {#using-the-returned-iterator-directly}

আপনার I/O project-এর _src/main.rs_ ফাইলটি খুলুন, যা দেখতে এমন হওয়ার কথা:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-24-reproduced/src/main.rs:ch13}}
```

প্রথমে Listing 12-24-এ থাকা `main` function-এর শুরুটা বদলে Listing 13-18-এর কোড করব, যা এবার একটি iterator ব্যবহার করে। `Config::build`-ও
হালনাগাদ না করা পর্যন্ত এটি compile হবে না।

<Listing number="13-18" file-name="src/main.rs" caption="`env::args`-এর return মান `Config::build`-এ পাঠানো">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-18/src/main.rs:here}}
```

</Listing>

`env::args` function একটি iterator return করে! iterator-এর মানগুলো একটি vector-এ জড়ো করে তারপর `Config::build`-এ একটি slice পাঠানোর বদলে
এখন আমরা `env::args` থেকে return হওয়া iterator-এর ownership সরাসরি `Config::build`-এ পাঠাচ্ছি।

এরপর `Config::build`-এর সংজ্ঞা হালনাগাদ করতে হবে। চলুন `Config::build`-এর signature বদলে Listing 13-19-এর মতো করি। এটি এখনো compile হবে না,
কারণ function-এর body হালনাগাদ করতে হবে।

<Listing number="13-19" file-name="src/main.rs" caption="`Config::build`-এর signature হালনাগাদ করে একটি iterator আশা করানো">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-19/src/main.rs:here}}
```

</Listing>

`env::args` function-এর standard library documentation দেখায় যে এটি যে iterator return করে তার type হলো `std::env::Args`, আর সেই type
`Iterator` trait implement করে এবং `String` মান return করে।

আমরা `Config::build` function-এর signature হালনাগাদ করেছি, যাতে `args` parameter-এর type `&[String]`-এর বদলে `impl Iterator<Item =
String>` trait bound-যুক্ত একটি generic type হয়। অধ্যায় 10-এর [“trait-কে parameter হিসেবে ব্যবহার করা”][impl-trait]<!-- ignore -->
অংশে আলোচিত `impl Trait` syntax-এর এই ব্যবহারের মানে হলো `args` এমন যেকোনো type হতে পারে যা `Iterator` trait implement করে এবং `String` item
return করে।

যেহেতু আমরা `args`-এর ownership নিচ্ছি এবং এর ওপর দিয়ে iterate করে `args`-কে পরিবর্তন করব, তাই `args` parameter-কে mutable করতে এর বিবরণে
`mut` keyword যোগ করতে পারি।

<!-- Old headings. Do not remove or links may break. -->

<a id="using-iterator-trait-methods-instead-of-indexing"></a>

#### `Iterator` trait-এর method ব্যবহার করা {#using-iterator-trait-methods}

এরপর `Config::build`-এর body ঠিক করব। `args` যেহেতু `Iterator` trait implement করে, তাই জানি এর ওপর `next` method call করা যায়! Listing
13-20 Listing 12-23-এর কোড হালনাগাদ করে `next` method ব্যবহার করায়।

<Listing number="13-20" file-name="src/main.rs" caption="`Config::build`-এর body বদলে iterator method ব্যবহার করা">

```rust,ignore,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-20/src/main.rs:here}}
```

</Listing>

মনে রাখবেন, `env::args`-এর return মানের প্রথম মান হলো প্রোগ্রামের নাম। আমরা সেটি উপেক্ষা করে পরের মানে যেতে চাই, তাই প্রথমে `next` call করি
এবং return মান দিয়ে কিছুই করি না। তারপর `Config`-এর `query` field-এ যে মান রাখতে চাই তা পেতে আবার `next` call করি। `next` যদি `Some` return
করে, তাহলে মানটি বের করতে একটি `match` ব্যবহার করি। যদি `None` return করে, তার মানে যথেষ্ট argument দেওয়া হয়নি, আর আমরা একটি `Err` মান
দিয়ে আগেভাগেই return করি। `file_path` মানের জন্যও একই কাজ করি।

<!-- Old headings. Do not remove or links may break. -->

<a id="making-code-clearer-with-iterator-adapters"></a>

### iterator adapter দিয়ে কোড আরও পরিষ্কার করা {#clarifying-code-with-iterator-adapters}

আমাদের I/O project-এর `search` function-এও iterator-এর সুবিধা নিতে পারি, যা এখানে Listing 13-21-এ আবার দেওয়া হলো, যেমনটি Listing 12-19-এ
ছিল।

<Listing number="13-21" file-name="src/lib.rs" caption="Listing 12-19-এর `search` function-এর implementation">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:ch13}}
```

</Listing>

iterator adapter method ব্যবহার করে এই কোড আরও সংক্ষেপে লেখা যায়। এতে একটি mutable মধ্যবর্তী `results` vector রাখাও এড়ানো যায়। কোড আরও
পরিষ্কার করতে functional programming-এর ধাঁচ mutable অবস্থার পরিমাণ যতটা সম্ভব কমাতে পছন্দ করে। mutable অবস্থা সরিয়ে দিলে ভবিষ্যতে খোঁজার
কাজ parallel-ভাবে করার উন্নতি সম্ভব হতে পারে, কারণ তখন `results` vector-এ একসঙ্গে প্রবেশ সামলাতে হবে না। Listing 13-22-এ এই পরিবর্তন
দেখানো হয়েছে।

<Listing number="13-22" file-name="src/lib.rs" caption="`search` function-এর implementation-এ iterator adapter method ব্যবহার করা">

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-22/src/lib.rs:here}}
```

</Listing>

মনে করুন, `search` function-এর উদ্দেশ্য হলো `contents`-এর যে লাইনগুলোতে `query` আছে সেগুলো সব return করা। Listing 13-16-এর `filter` উদাহরণের
মতোই এই কোড `filter` adapter ব্যবহার করে শুধু সেই লাইনগুলো রাখে, যাদের জন্য `line.contains(query)` `true` return করে। তারপর `collect` দিয়ে
মিলে যাওয়া লাইনগুলো আরেকটি vector-এ জড়ো করি। অনেক সহজ! চাইলে `search_case_insensitive` function-এও iterator method ব্যবহারের একই পরিবর্তন
করতে পারেন।

আরও উন্নতির জন্য, `collect` call সরিয়ে আর return type বদলে `impl
Iterator<Item = &'a str>` করে `search` function থেকে একটি iterator return করুন, যাতে function-টি নিজেই একটি iterator adapter হয়ে যায়। খেয়াল
করুন, test-গুলোও হালনাগাদ করতে হবে! আচরণের পার্থক্য দেখতে এই পরিবর্তনের আগে ও পরে আপনার `minigrep` tool দিয়ে একটি বড় ফাইলে খুঁজে দেখুন।
এই পরিবর্তনের আগে প্রোগ্রাম সব ফল জড়ো না করা পর্যন্ত কোনো ফল print করবে না, কিন্তু পরিবর্তনের পরে প্রতিটি মিলে যাওয়া লাইন পাওয়ার সঙ্গে সঙ্গে
ফল print হবে, কারণ `run` function-এর `for` loop iterator-এর lazy স্বভাবের সুবিধা নিতে পারে।

<!-- Old headings. Do not remove or links may break. -->

<a id="choosing-between-loops-or-iterators"></a>

### loop নাকি iterator, কোনটি বেছে নেবেন {#choosing-between-loops-and-iterators}

পরের যৌক্তিক প্রশ্ন হলো, নিজের কোডে কোন ধাঁচ বেছে নেবেন এবং কেন: Listing 13-21-এর মূল implementation, নাকি Listing 13-22-এর iterator ব্যবহার
করা সংস্করণ (ধরে নিচ্ছি iterator return না করে return করার আগে সব ফল জড়ো করছি)। বেশিরভাগ Rust প্রোগ্রামার iterator-এর ধাঁচ পছন্দ করেন।
শুরুতে এটি রপ্ত করা একটু কঠিন, কিন্তু বিভিন্ন iterator adapter আর সেগুলো কী করে তা একবার বুঝে ফেললে iterator বোঝা সহজ হতে পারে। loop-এর
নানা খুঁটিনাটি আর নতুন vector তৈরি নিয়ে নাড়াচাড়া না করে কোডটি loop-এর উচ্চ-স্তরের লক্ষ্যের ওপর মনোযোগ দেয়। এতে কিছু সাধারণ কোড আড়ালে চলে
যায়, ফলে এই কোডের নিজস্ব ধারণাগুলো, যেমন iterator-এর প্রতিটি উপাদানকে যে filter-এর শর্ত পার হতে হবে, তা দেখা সহজ হয়।

কিন্তু দুটি implementation কি সত্যিই সমতুল্য? সহজাত ধারণা হতে পারে যে নিম্ন-স্তরের loop বেশি দ্রুত হবে। চলুন performance নিয়ে কথা বলি।

[impl-trait]: ch10-02-traits.html#traits-as-parameters
