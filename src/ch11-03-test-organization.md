## test সাজানো {#test-organization}

অধ্যায়ের শুরুতে যেমন বলা হয়েছে, testing একটি জটিল বিষয়, আর ভিন্ন ভিন্ন মানুষ ভিন্ন পরিভাষা ও সাজানোর ধরন ব্যবহার করে। Rust
community test-কে দুটি প্রধান শ্রেণিতে ভাবে: unit test আর integration test। _Unit tests_ (unit test) ছোট ও বেশি
কেন্দ্রীভূত, একবারে একটি module-কে আলাদা করে test করে, এবং private interface-ও test করতে পারে। _Integration tests_
(integration test) পুরোপুরি আপনার library-র বাইরের, এবং অন্য যেকোনো বাইরের কোড যেভাবে আপনার কোড ব্যবহার করত সেভাবেই ব্যবহার
করে, শুধু public interface দিয়ে, আর প্রতিটি test-এ সম্ভবত একাধিক module পরীক্ষা করে।

আপনার library-র অংশগুলো আলাদাভাবে ও একসঙ্গে আপনার প্রত্যাশামতো কাজ করছে কি না তা নিশ্চিত করতে দুই ধরনের test লেখাই
গুরুত্বপূর্ণ।

### unit test {#unit-tests}

unit test-এর উদ্দেশ্য হলো কোডের প্রতিটি একককে বাকি কোড থেকে আলাদা করে test করা, যাতে দ্রুত চিহ্নিত করা যায় কোথায় কোড
প্রত্যাশামতো কাজ করছে আর কোথায় করছে না। unit test রাখবেন _src_ directory-তে, যে কোড test করছে সেই কোডের সঙ্গে একই ফাইলে।
প্রচলিত রীতি হলো প্রতিটি ফাইলে test function-গুলো রাখার জন্য `tests` নামের একটি module তৈরি করা এবং module-টিকে `cfg(test)`
দিয়ে annotate করা।

#### `tests` module ও `#[cfg(test)]` {#the-tests-module-and-cfgtest}

`tests` module-এর ওপর `#[cfg(test)]` annotation Rust-কে বলে test কোড শুধু `cargo test` চালালেই compile করে চালাতে, `cargo
build` চালালে নয়। এতে শুধু library build করতে চাইলে compile-এর সময় বাঁচে, আর তৈরি হওয়া compile করা artifact (ফলাফল ফাইল)-এ
জায়গাও বাঁচে, কারণ test-গুলো তাতে থাকে না। আপনি দেখবেন, integration test যেহেতু আলাদা directory-তে থাকে, তাই সেগুলোর
`#[cfg(test)]` annotation লাগে না। কিন্তু unit test কোডের সঙ্গে একই ফাইলে থাকে বলে `#[cfg(test)]` দিয়ে বলে দিতে হয় যে
এগুলো compile করা ফলাফলে যুক্ত হবে না।

মনে করুন, এই অধ্যায়ের প্রথম অংশে নতুন `adder` project তৈরি করার সময় Cargo আমাদের জন্য এই কোড তৈরি করেছিল:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs}}
```

স্বয়ংক্রিয়ভাবে তৈরি `tests` module-এর ওপর থাকা `cfg` attribute-এর পূর্ণরূপ _configuration_ (কনফিগারেশন), আর এটি Rust-কে
বলে যে পরের item-টি শুধু নির্দিষ্ট একটি configuration বিকল্প থাকলেই যুক্ত হবে। এই ক্ষেত্রে configuration বিকল্পটি হলো `test`,
যা Rust test compile ও চালানোর জন্য দেয়। `cfg` attribute ব্যবহার করায় Cargo আমাদের test কোড compile করে কেবল তখনই, যখন আমরা
সক্রিয়ভাবে `cargo test` দিয়ে test চালাই। এর মধ্যে `#[test]` দিয়ে annotate করা function ছাড়াও এই module-এ থাকা যেকোনো
সহায়ক function পড়ে।

<!-- Old headings. Do not remove or links may break. -->

<a id="testing-private-functions"></a>

#### private function-এর test {#private-function-tests}

private function সরাসরি test করা উচিত কি না তা নিয়ে testing community-তে বিতর্ক আছে, আর অন্য কিছু ভাষায় private function
test করা কঠিন বা অসম্ভব। আপনি testing-এর যে মতাদর্শই মানুন না কেন, Rust-এর privacy-র নিয়ম আপনাকে private function test
করতে দেয়। Listing 11-12-এর কোডটি দেখুন, যেখানে `internal_adder` নামের একটি private function আছে।

<Listing number="11-12" file-name="src/lib.rs" caption="একটি private function test করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-12/src/lib.rs}}
```

</Listing>

খেয়াল করুন, `internal_adder` function-কে `pub` হিসেবে চিহ্নিত করা হয়নি। test শুধুই Rust কোড, আর `tests` module আরেকটি
module মাত্র। [“module tree-র কোনো item উল্লেখ করার path”][paths]<!-- ignore --> অংশে যেমন আলোচনা করেছি, child module-এর
item-গুলো তাদের পূর্বপুরুষ module-এর item ব্যবহার করতে পারে। এই test-এ আমরা `use super::*` দিয়ে `tests` module-এর parent-এর
সব item scope-এ আনি, তারপর test-টি `internal_adder` call করতে পারে। আপনি যদি মনে করেন private function test করা উচিত নয়, Rust-এ
এমন কিছু নেই যা আপনাকে তা করতে বাধ্য করবে।

### integration test {#integration-tests}

Rust-এ integration test পুরোপুরি আপনার library-র বাইরের। এগুলো অন্য যেকোনো কোডের মতোই আপনার library ব্যবহার করে, অর্থাৎ
শুধু সেই function-গুলোই call করতে পারে যা আপনার library-র public API-র অংশ। এদের উদ্দেশ্য হলো আপনার library-র অনেক অংশ
একসঙ্গে ঠিকমতো কাজ করে কি না তা test করা। কোডের যে এককগুলো আলাদাভাবে ঠিকঠাক কাজ করে, একসঙ্গে জোড়া লাগালে সেগুলোতে
সমস্যা দেখা দিতে পারে, তাই জোড়া লাগানো কোডের test coverage-ও গুরুত্বপূর্ণ। integration test তৈরি করতে প্রথমে একটি _tests_
directory লাগবে।

#### _tests_ directory {#the-tests-directory}

আমরা project directory-র শীর্ষ স্তরে, _src_-এর পাশে একটি _tests_ directory তৈরি করি। Cargo জানে যে এই directory-তে
integration test ফাইল খুঁজতে হবে। তারপর যত খুশি test ফাইল বানাতে পারি, আর Cargo প্রতিটি ফাইলকে একটি আলাদা crate হিসেবে
compile করবে।

চলুন একটি integration test তৈরি করি। Listing 11-12-এর কোড _src/lib.rs_ ফাইলে রেখেই একটি _tests_ directory বানান, আর
_tests/integration_test.rs_ নামে একটি নতুন ফাইল তৈরি করুন। আপনার directory-র গঠন দেখতে এমন হওয়ার কথা:

```text
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```

Listing 11-13-এর কোডটি _tests/integration_test.rs_ ফাইলে লিখুন।

<Listing number="11-13" file-name="tests/integration_test.rs" caption="`adder` crate-এর একটি function-এর integration test">

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-13/tests/integration_test.rs}}
```

</Listing>

_tests_ directory-র প্রতিটি ফাইল একটি আলাদা crate, তাই প্রতিটি test crate-এর scope-এ আমাদের library আনতে হবে। এ কারণে কোডের
শুরুতে আমরা `use
adder::add_two;` যোগ করি, যা unit test-এ দরকার হয়নি।

_tests/integration_test.rs_-এর কোনো কোডকে `#[cfg(test)]` দিয়ে annotate করার দরকার নেই। Cargo _tests_ directory-কে বিশেষভাবে
গণ্য করে এবং এই directory-র ফাইলগুলো compile করে কেবল তখনই, যখন আমরা `cargo test` চালাই। এখন `cargo test` চালান:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-13/output.txt}}
```

output-এর তিনটি অংশে আছে unit test, integration test, আর doc test। খেয়াল করুন, কোনো অংশের কোনো test fail করলে পরের অংশগুলো
চলবে না। যেমন, একটি unit test fail করলে integration ও doc test-এর কোনো output থাকবে না, কারণ ওই test-গুলো চলে কেবল তখনই,
যখন সব unit test pass করে।

unit test-এর প্রথম অংশটি আগে যেমন দেখেছি তেমনই: প্রতিটি unit test-এর জন্য একটি লাইন (Listing 11-12-এ যোগ করা `internal`
নামের একটি), তারপর unit test-গুলোর একটি সারসংক্ষেপ লাইন।

integration test-এর অংশটি শুরু হয় `Running
tests/integration_test.rs` লাইন দিয়ে। এরপর সেই integration test-এর প্রতিটি test function-এর জন্য একটি লাইন, আর `Doc-tests
adder` অংশ শুরু হওয়ার ঠিক আগে integration test-এর ফলের একটি সারসংক্ষেপ লাইন থাকে।

প্রতিটি integration test ফাইলের নিজস্ব অংশ থাকে, তাই _tests_ directory-তে আরও ফাইল যোগ করলে আরও integration test অংশ দেখা
যাবে।

test function-এর নাম `cargo test`-এ argument হিসেবে দিয়ে এখনো নির্দিষ্ট একটি integration test function চালানো যায়। নির্দিষ্ট
একটি integration test ফাইলের সব test চালাতে `cargo test`-এর `--test` argument-এর পরে ফাইলের নাম দিন:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-05-single-integration/output.txt}}
```

এই command শুধু _tests/integration_test.rs_ ফাইলের test-গুলো চালায়।

#### integration test-এ submodule {#submodules-in-integration-tests}

আরও integration test যোগ করতে থাকলে সেগুলো গুছিয়ে রাখতে হয়তো _tests_ directory-তে আরও ফাইল বানাতে চাইবেন; যেমন, test
function-গুলোকে তারা কোন কার্যক্ষমতা test করছে সেই অনুযায়ী ভাগ করতে পারেন। আগেই বলা হয়েছে, _tests_ directory-র প্রতিটি
ফাইল নিজস্ব আলাদা crate হিসেবে compile হয়, যা আলাদা scope তৈরি করে শেষ ব্যবহারকারীরা আপনার crate যেভাবে ব্যবহার করবে তা
আরও কাছ থেকে অনুকরণ করতে কাজে লাগে। তবে এর মানে হলো, অধ্যায় 7-এ কোডকে module ও ফাইলে ভাগ করা নিয়ে যা শিখেছেন, _tests_
directory-র ফাইলগুলো _src_-এর ফাইলগুলোর মতো আচরণ করে না।

_tests_ directory-র ফাইলগুলোর এই ভিন্ন আচরণ সবচেয়ে বেশি চোখে পড়ে যখন একাধিক integration test ফাইলে ব্যবহারের জন্য কিছু
সহায়ক function থাকে, আর আপনি অধ্যায় 7-এর [“module-গুলোকে আলাদা ফাইলে ভাগ
করা”][separating-modules-into-files]<!-- ignore --> অংশের ধাপগুলো মেনে সেগুলোকে একটি সাধারণ module-এ বের করে আনার চেষ্টা করেন।
যেমন, যদি _tests/common.rs_ তৈরি করে তাতে `setup` নামের একটি function রাখি, তাহলে `setup`-এ এমন কিছু কোড যোগ করতে পারি যা
একাধিক test ফাইলের একাধিক test function থেকে call করতে চাই:

<span class="filename">Filename: tests/common.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-12-shared-test-code-problem/tests/common.rs}}
```

আবার test চালালে test output-এ _common.rs_ ফাইলের জন্য একটি নতুন অংশ দেখব, যদিও এই ফাইলে কোনো test function নেই এবং আমরা
কোথাও থেকে `setup` function call-ও করিনি:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-12-shared-test-code-problem/output.txt}}
```

test-এর ফলে `common`-এর পাশে `running 0 tests` দেখানো আমরা চাইনি। আমরা শুধু অন্য integration test ফাইলগুলোর সঙ্গে কিছু কোড
ভাগ করতে চেয়েছিলাম। test output-এ `common` যাতে না দেখায়, সেজন্য _tests/common.rs_ তৈরি না করে আমরা _tests/common/mod.rs_
তৈরি করব। project directory এখন দেখতে এমন:

```text
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
```

এটি পুরোনো নামকরণের রীতি, যা Rust-ও বোঝে এবং যার কথা আমরা অধ্যায় 7-এর [“বিকল্প ফাইল path”][alt-paths]<!-- ignore --> অংশে
বলেছিলাম। এভাবে ফাইলের নাম দিলে Rust `common` module-কে integration test ফাইল হিসেবে গণ্য করে না। `setup` function-এর কোড
_tests/common/mod.rs_-এ সরিয়ে _tests/common.rs_ ফাইলটি মুছে দিলে test output-এর সেই অংশটি আর দেখা যাবে না। _tests_
directory-র subdirectory-তে থাকা ফাইলগুলো আলাদা crate হিসেবে compile হয় না, আর test output-এ তাদের কোনো অংশও থাকে না।

_tests/common/mod.rs_ তৈরি করার পর যেকোনো integration test ফাইল থেকে এটিকে একটি module হিসেবে ব্যবহার করা যায়। এখানে
_tests/integration_test.rs_-এর `it_adds_two` test থেকে `setup` function call করার একটি উদাহরণ:

<span class="filename">Filename: tests/integration_test.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-13-fix-shared-test-code-problem/tests/integration_test.rs}}
```

খেয়াল করুন, `mod common;` ঘোষণাটি Listing 7-21-এ দেখানো module ঘোষণার মতোই। তারপর test function-এ আমরা `common::setup()`
function call করতে পারি।

#### binary crate-এর integration test {#integration-tests-for-binary-crates}

আমাদের project যদি এমন একটি binary crate হয় যাতে শুধু একটি _src/main.rs_ ফাইল আছে এবং কোনো _src/lib.rs_ ফাইল নেই, তাহলে
_tests_ directory-তে integration test তৈরি করে `use` statement দিয়ে _src/main.rs_ ফাইলে সংজ্ঞায়িত function scope-এ আনা যায়
না। শুধু library crate-ই এমন function প্রকাশ করে যা অন্য crate ব্যবহার করতে পারে; binary crate তৈরি হয় নিজে নিজে চলার জন্য।

এ কারণেই binary দেয় এমন Rust project-গুলোতে একটি সোজাসাপ্টা _src/main.rs_ ফাইল থাকে, যা _src/lib.rs_ ফাইলে থাকা logic call
করে। এই গঠন ব্যবহার করলে integration test `use` দিয়ে গুরুত্বপূর্ণ কার্যক্ষমতা হাতের নাগালে এনে library crate test _করতে পারে_।
গুরুত্বপূর্ণ কার্যক্ষমতা কাজ করলে _src/main.rs_ ফাইলের অল্প কোডটুকুও কাজ করবে, আর সেই অল্প কোড test করার দরকার নেই।

## সারসংক্ষেপ {#summary}

Rust-এর testing বৈশিষ্ট্যগুলো কোড কীভাবে কাজ করা উচিত তা নির্দিষ্ট করার একটি উপায় দেয়, যাতে পরিবর্তন আনার পরও কোড আপনার
প্রত্যাশামতো কাজ করে যায়। unit test একটি library-র বিভিন্ন অংশ আলাদাভাবে পরীক্ষা করে এবং private implementation-এর খুঁটিনাটিও
test করতে পারে। integration test যাচাই করে library-র অনেক অংশ একসঙ্গে ঠিকমতো কাজ করে কি না, আর বাইরের কোড যেভাবে library
ব্যবহার করবে সেভাবেই library-র public API দিয়ে কোড test করে। Rust-এর type system ও ownership-এর নিয়ম কিছু ধরনের বাগ ঠেকাতে
সাহায্য করলেও, আপনার কোড কেমন আচরণ করবে বলে আশা করা হয় সে সংক্রান্ত logic-এর বাগ কমাতে test এখনো গুরুত্বপূর্ণ।

চলুন এই অধ্যায়ে ও আগের অধ্যায়গুলোতে শেখা জ্ঞান একসঙ্গে কাজে লাগিয়ে একটি project-এ কাজ করি!

{{#quiz ../quizzes/ch11-03-test-organization.toml}}

[paths]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
[separating-modules-into-files]: ch07-05-separating-modules-into-different-files.html
[alt-paths]: ch07-05-separating-modules-into-different-files.html#alternate-file-paths
