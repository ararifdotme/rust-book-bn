<!-- Old headings. Do not remove or links may break. -->

<a id="closures-anonymous-functions-that-can-capture-their-environment"></a>
<a id="closures-anonymous-functions-that-capture-their-environment"></a>

## closure {#closures}

Rust-এর closure (পারিপার্শ্বিক মান ধরে রাখতে পারে এমন নামহীন function) হলো নামহীন function, যা variable-এ রাখা যায় বা অন্য
function-এ argument হিসেবে পাঠানো যায়। এক জায়গায় closure তৈরি করে অন্য জায়গায় call করে ভিন্ন প্রেক্ষাপটে একে মূল্যায়ন করা যায়।
function-এর মতো নয়, closure যে scope-এ সংজ্ঞায়িত হয় সেখান থেকে মান capture করতে (ধরে রাখতে) পারে। closure-এর এই বৈশিষ্ট্যগুলো
কীভাবে কোডের পুনর্ব্যবহার আর আচরণ নিজের মতো সাজানো সম্ভব করে, তা আমরা দেখাব।

<!-- Old headings. Do not remove or links may break. -->

<a id="creating-an-abstraction-of-behavior-with-closures"></a>
<a id="refactoring-using-functions"></a>
<a id="refactoring-with-closures-to-store-code"></a>
<a id="capturing-the-environment-with-closures"></a>

### environment capture করা {#capturing-the-environment}

প্রথমে দেখব কীভাবে closure ব্যবহার করে যে environment (পারিপার্শ্বিক পরিবেশ)-এ এটি সংজ্ঞায়িত হয়েছে সেখান থেকে পরে ব্যবহারের জন্য মান
capture করা যায়। পরিস্থিতিটা এমন: প্রচারণার অংশ হিসেবে আমাদের T-shirt কোম্পানি মাঝে মাঝে mailing list-এর কাউকে একটি বিশেষ, সীমিত
সংস্করণের shirt উপহার দেয়। mailing list-এর মানুষেরা চাইলে তাদের profile-এ প্রিয় রং যোগ করতে পারে। বিনামূল্যের shirt-এর জন্য বাছাই
করা ব্যক্তির প্রিয় রং সেট করা থাকলে সে সেই রঙের shirt পায়। আর প্রিয় রং উল্লেখ না করলে কোম্পানির কাছে এই মুহূর্তে যে রঙের shirt সবচেয়ে
বেশি আছে, সেটি পায়।

এটি implement করার অনেক উপায় আছে। এই উদাহরণে আমরা `ShirtColor` নামের একটি enum ব্যবহার করব, যার variant হলো `Red` আর `Blue` (সরলতার
জন্য রঙের সংখ্যা সীমিত রাখা হয়েছে)। কোম্পানির মজুত বোঝাতে আমরা একটি `Inventory` struct ব্যবহার করি, যার `shirts` নামের একটি field-এ
একটি `Vec<ShirtColor>` থাকে, যা এই মুহূর্তে মজুত থাকা shirt-এর রংগুলো বোঝায়। `Inventory`-এর ওপর সংজ্ঞায়িত `giveaway` method
বিনামূল্যের shirt বিজয়ীর ঐচ্ছিক রঙের পছন্দ নেয়, এবং ব্যক্তিটি যে রঙের shirt পাবে তা return করে। এই ব্যবস্থাটি Listing 13-1-এ দেখানো
হয়েছে।

<Listing number="13-1" file-name="src/main.rs" caption="shirt কোম্পানির উপহার দেওয়ার পরিস্থিতি">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-01/src/main.rs}}
```

</Listing>

`main`-এ সংজ্ঞায়িত `store`-এ এই সীমিত সংস্করণের প্রচারণায় বিতরণের জন্য দুটি নীল আর একটি লাল shirt বাকি আছে। আমরা লাল shirt পছন্দ করা
একজন ব্যবহারকারী আর কোনো পছন্দ নেই এমন একজন ব্যবহারকারীর জন্য `giveaway` method call করি।

আবারও বলি, এই কোড অনেকভাবে implement করা যেত, আর এখানে closure-এর দিকে মনোযোগ রাখতে আমরা আপনার আগে শেখা ধারণাগুলোতেই সীমাবদ্ধ থেকেছি,
শুধু `giveaway` method-এর body ছাড়া, যেখানে একটি closure ব্যবহার করা হয়েছে। `giveaway` method-এ আমরা ব্যবহারকারীর পছন্দ
`Option<ShirtColor>` type-এর parameter হিসেবে পাই এবং `user_preference`-এর ওপর `unwrap_or_else` method call করি। [`Option<T>`-এর ওপর
`unwrap_or_else` method][unwrap-or-else]<!-- ignore --> standard library-তে সংজ্ঞায়িত। এটি একটি argument নেয়: কোনো argument ছাড়া একটি
closure, যা একটি `T` মান return করে (`Option<T>`-এর `Some` variant-এ যে type রাখা থাকে সেই একই type, এই ক্ষেত্রে `ShirtColor`)।
`Option<T>` যদি `Some` variant হয়, তাহলে `unwrap_or_else` `Some`-এর ভেতরের মানটি return করে। `Option<T>` যদি `None` variant হয়, তাহলে
`unwrap_or_else` closure-টি call করে এবং closure-এর return করা মানটি return করে।

`unwrap_or_else`-এর argument হিসেবে আমরা closure expression `|| self.most_stocked()` দিই। এটি এমন একটি closure যা নিজে কোনো parameter
নেয় না (closure-এর parameter থাকলে সেগুলো দুটি খাড়া দাগের মাঝে থাকত)। closure-এর body `self.most_stocked()` call করে। closure-টি আমরা
এখানে সংজ্ঞায়িত করছি, আর ফলাফল দরকার হলে `unwrap_or_else`-এর implementation পরে closure-টি মূল্যায়ন করবে।

এই কোড চালালে এই output আসে:

```console
{{#include ../listings/ch13-functional-features/listing-13-01/output.txt}}
```

এখানে একটি মজার দিক হলো, আমরা এমন একটি closure পাঠিয়েছি যা বর্তমান `Inventory` instance-এর ওপর `self.most_stocked()` call করে।
আমাদের সংজ্ঞায়িত `Inventory` বা `ShirtColor` type, বা এই পরিস্থিতিতে আমরা যে logic ব্যবহার করতে চাই, সে সম্পর্কে standard library-র কিছু
জানার দরকার হয়নি। closure-টি `self` `Inventory` instance-এর একটি immutable reference capture করে এবং আমাদের দেওয়া কোডের সঙ্গে সেটি
`unwrap_or_else` method-এ পাঠায়। অন্যদিকে function এভাবে তার environment capture করতে পারে না।

<!-- Old headings. Do not remove or links may break. -->

<a id="closure-type-inference-and-annotation"></a>

### closure-এর type অনুমান ও annotate করা {#inferring-and-annotating-closure-types}

function আর closure-এর মধ্যে আরও পার্থক্য আছে। `fn` function-এর মতো closure-এ সাধারণত parameter বা return মানের type annotate করতে হয়
না। function-এ type annotation বাধ্যতামূলক, কারণ type-গুলো আপনার ব্যবহারকারীদের কাছে প্রকাশিত একটি স্পষ্ট interface-এর অংশ। একটি
function কোন type-এর মান ব্যবহার ও return করে সে বিষয়ে সবাই যাতে একমত থাকে, তা নিশ্চিত করতে এই interface কঠোরভাবে সংজ্ঞায়িত করা
গুরুত্বপূর্ণ। অন্যদিকে closure এভাবে কোনো প্রকাশিত interface-এ ব্যবহৃত হয় না: এগুলো variable-এ রাখা হয়, এবং নাম না দিয়ে ও আমাদের
library-র ব্যবহারকারীদের কাছে প্রকাশ না করেই ব্যবহার করা হয়।

closure সাধারণত ছোট হয় এবং যেকোনো পরিস্থিতিতে নয়, বরং একটি সংকীর্ণ প্রেক্ষাপটেই প্রাসঙ্গিক থাকে। এই সীমিত প্রেক্ষাপটে compiler
parameter আর return মানের type অনুমান করতে পারে, ঠিক যেভাবে বেশিরভাগ variable-এর type অনুমান করতে পারে (তবে বিরল কিছু ক্ষেত্রে
compiler-এরও closure-এর type annotation দরকার হয়)।

variable-এর মতোই, প্রয়োজনের চেয়ে বেশি শব্দবহুল হওয়ার বিনিময়ে স্পষ্টতা বাড়াতে চাইলে আমরা type annotation যোগ করতে পারি। একটি
closure-এর type annotate করলে দেখতে Listing 13-2-এর সংজ্ঞার মতো হবে। এই উদাহরণে Listing 13-1-এর মতো যেখানে argument হিসেবে পাঠাচ্ছি
সেখানেই closure সংজ্ঞায়িত না করে আমরা একটি closure সংজ্ঞায়িত করে variable-এ রাখছি।

<Listing number="13-2" file-name="src/main.rs" caption="closure-এ parameter ও return মানের type-এর ঐচ্ছিক annotation যোগ করা">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-02/src/main.rs:here}}
```

</Listing>

type annotation যোগ করলে closure-এর syntax function-এর syntax-এর আরও কাছাকাছি দেখায়। তুলনার জন্য এখানে আমরা একটি function সংজ্ঞায়িত
করেছি যা তার parameter-এর সঙ্গে 1 যোগ করে, আর একই আচরণের একটি closure। প্রাসঙ্গিক অংশগুলো এক সারিতে আনতে কিছু space যোগ করেছি। এতে
বোঝা যায় যে closure-এর syntax function-এর syntax-এর মতোই, শুধু খাড়া দাগের ব্যবহার আর কতটা syntax ঐচ্ছিক, এই দুটি ছাড়া:

```rust,ignore
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

প্রথম লাইনে একটি function-এর সংজ্ঞা, আর দ্বিতীয় লাইনে পুরোপুরি annotate করা একটি closure-এর সংজ্ঞা। তৃতীয় লাইনে closure-এর সংজ্ঞা
থেকে type annotation সরিয়ে দিয়েছি। চতুর্থ লাইনে bracket সরিয়ে দিয়েছি, যা ঐচ্ছিক, কারণ closure-এর body-তে মাত্র একটি expression।
এগুলো সবই বৈধ সংজ্ঞা, যা call করলে একই আচরণ করবে। `add_one_v3` আর `add_one_v4` লাইন compile হতে হলে closure-গুলোকে মূল্যায়ন করতে হবে,
কারণ type অনুমান হবে তাদের ব্যবহার থেকে। এটা `let v = Vec::new();`-এর মতো, যেখানে Rust-কে type অনুমান করতে হলে হয় type annotation
লাগে, নয়তো `Vec`-এ কোনো type-এর মান ঢোকাতে হয়।

closure-এর সংজ্ঞার ক্ষেত্রে compiler এর প্রতিটি parameter আর return মানের জন্য একটি করে concrete type অনুমান করবে। যেমন, Listing 13-3-এ
একটি ছোট closure-এর সংজ্ঞা দেখানো হয়েছে, যা parameter হিসেবে পাওয়া মানটিই return করে। এই উদাহরণের উদ্দেশ্য ছাড়া closure-টি খুব একটা
কাজের নয়। খেয়াল করুন, সংজ্ঞায় আমরা কোনো type annotation যোগ করিনি। type annotation না থাকায় যেকোনো type দিয়ে closure-টি call করা যায়,
এখানে প্রথমবার আমরা `String` দিয়ে তা করেছি। তারপর integer দিয়ে `example_closure` call করার চেষ্টা করলে error পাব।

<Listing number="13-3" file-name="src/main.rs" caption="যে closure-এর type অনুমান করা হয়েছে তাকে দুটি ভিন্ন type দিয়ে call করার চেষ্টা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-03/src/main.rs:here}}
```

</Listing>

compiler আমাদের এই error দেয়:

```console
{{#include ../listings/ch13-functional-features/listing-13-03/output.txt}}
```

প্রথমবার `String` মান দিয়ে `example_closure` call করলে compiler `x`-এর type আর closure-এর return type `String` বলে অনুমান করে। তারপর
সেই type-গুলো `example_closure`-এর closure-এ আটকে যায়, আর পরের বার একই closure-এ ভিন্ন type ব্যবহার করার চেষ্টা করলে type error পাই।

{{#quiz ../quizzes/ch13-01-closures-sec1.toml}}

### reference capture করা বা ownership সরিয়ে নেওয়া {#capturing-references-or-moving-ownership}

closure তার environment থেকে তিনভাবে মান capture করতে পারে, যা সরাসরি function-এর parameter নেওয়ার তিনটি উপায়ের সঙ্গে মেলে: immutable-ভাবে
borrow করা, mutable-ভাবে borrow করা, আর ownership নেওয়া। capture করা মানগুলো নিয়ে function-এর body কী করে, তার ওপর ভিত্তি করে closure
ঠিক করবে এগুলোর কোনটি ব্যবহার করবে।

Listing 13-4-এ আমরা এমন একটি closure সংজ্ঞায়িত করি যা `list` নামের vector-এর একটি immutable reference capture করে, কারণ মান print করতে এর
শুধু একটি immutable reference দরকার।

<Listing number="13-4" file-name="src/main.rs" caption="immutable reference capture করে এমন একটি closure সংজ্ঞায়িত ও call করা">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-04/src/main.rs}}
```

</Listing>

এই উদাহরণ এটাও দেখায় যে একটি variable একটি closure-এর সংজ্ঞার সঙ্গে bind হতে পারে, আর পরে variable-এর নাম আর বন্ধনী ব্যবহার করে
closure-টি call করা যায়, যেন variable-এর নামটিই একটি function-এর নাম।

একই সময়ে `list`-এর একাধিক immutable reference থাকতে পারে বলে closure-এর সংজ্ঞার আগের কোড থেকে, closure-এর সংজ্ঞার পরে কিন্তু
closure call করার আগে, আর closure call করার পরে, সব জায়গা থেকেই `list` ব্যবহার করা যায়। এই কোড compile হয়, চলে, এবং print করে:

```console
{{#include ../listings/ch13-functional-features/listing-13-04/output.txt}}
```

এরপর Listing 13-5-এ আমরা closure-এর body বদলে দিই, যাতে এটি `list` vector-এ একটি উপাদান যোগ করে। closure-টি এখন একটি mutable reference
capture করে।

<Listing number="13-5" file-name="src/main.rs" caption="mutable reference capture করে এমন একটি closure সংজ্ঞায়িত ও call করা">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-05/src/main.rs}}
```

</Listing>

এই কোড compile হয়, চলে, এবং print করে:

```console
{{#include ../listings/ch13-functional-features/listing-13-05/output.txt}}
```

খেয়াল করুন, `borrows_mutably` closure-এর সংজ্ঞা আর call-এর মাঝে এখন আর কোনো `println!` নেই: `borrows_mutably` সংজ্ঞায়িত হওয়ার সময় এটি
`list`-এর একটি mutable reference capture করে। closure call করার পর আমরা আর closure-টি ব্যবহার করি না, তাই mutable borrow শেষ হয়ে
যায়। closure-এর সংজ্ঞা আর closure call-এর মাঝে print করার জন্য immutable borrow অনুমোদিত নয়, কারণ mutable borrow থাকলে অন্য কোনো
borrow অনুমোদিত নয়। সেখানে একটি `println!` যোগ করে দেখুন কোন error message পান!

closure-এর body-র আসলে ownership দরকার না হলেও আপনি যদি closure-কে environment-এ ব্যবহৃত মানগুলোর ownership নিতে বাধ্য করতে চান, তাহলে
parameter তালিকার আগে `move` keyword ব্যবহার করতে পারেন।

এই কৌশল মূলত কাজে লাগে যখন একটি নতুন thread-এ closure পাঠিয়ে ডেটা সরিয়ে নিতে হয়, যাতে ডেটার owner হয় নতুন thread। thread কী এবং কেন
ব্যবহার করবেন, তা বিস্তারিত আলোচনা করব অধ্যায় 16-এ concurrency (একসঙ্গে একাধিক কাজ সামলানো) নিয়ে কথা বলার সময়, তবে আপাতত চলুন সংক্ষেপে
দেখি কীভাবে `move` keyword লাগে এমন একটি closure দিয়ে নতুন thread চালু করা যায়। Listing 13-6-এ Listing 13-4-কে বদলে main thread-এর বদলে
একটি নতুন thread-এ vector print করানো হয়েছে।

<Listing number="13-6" file-name="src/main.rs" caption="thread-এর closure-কে `list`-এর ownership নিতে বাধ্য করতে `move` ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-06/src/main.rs}}
```

</Listing>

আমরা একটি নতুন thread চালু করি, আর argument হিসেবে thread-কে চালানোর জন্য একটি closure দিই। closure-এর body তালিকাটি print করে। Listing
13-4-এ closure শুধু একটি immutable reference দিয়ে `list` capture করেছিল, কারণ print করতে `list`-এর ওপর ওইটুকু প্রবেশাধিকারই যথেষ্ট। এই
উদাহরণে closure-এর body-র এখনো শুধু একটি immutable reference দরকার হলেও closure-এর সংজ্ঞার শুরুতে `move` keyword বসিয়ে আমাদের বলে দিতে
হবে যে `list`-কে closure-এর ভেতরে সরিয়ে নিতে হবে। নতুন thread-এ `join` call করার আগে main thread যদি আরও কাজ করত, তাহলে নতুন thread
main thread-এর বাকি অংশ শেষ হওয়ার আগেই শেষ হয়ে যেতে পারত, অথবা main thread আগে শেষ হতে পারত। main thread যদি `list`-এর ownership রেখে
দিত কিন্তু নতুন thread-এর আগে শেষ হয়ে `list` drop করত, তাহলে thread-এর immutable reference অবৈধ হয়ে যেত। তাই compiler চায় `list`-কে
নতুন thread-এ দেওয়া closure-এর ভেতরে সরিয়ে নেওয়া হোক, যাতে reference বৈধ থাকে। `move` keyword সরিয়ে দিয়ে, বা closure সংজ্ঞায়িত করার
পরে main thread-এ `list` ব্যবহার করে দেখুন কোন compiler error পান!

<!-- Old headings. Do not remove or links may break. -->

<a id="storing-closures-using-generic-parameters-and-the-fn-traits"></a>
<a id="limitations-of-the-cacher-implementation"></a>
<a id="moving-captured-values-out-of-the-closure-and-the-fn-traits"></a>
<a id="moving-captured-values-out-of-closures-and-the-fn-traits"></a>

### capture করা মান closure থেকে বের করে নেওয়া {#moving-captured-values-out-of-closures}

closure যে environment-এ সংজ্ঞায়িত হয়েছে সেখান থেকে একবার কোনো মানের reference বা ownership capture করে ফেললে (যা ঠিক করে কী, যদি
কিছু থাকে, closure-এর _ভেতরে_ সরানো হবে), closure-এর body-র কোড ঠিক করে পরে closure মূল্যায়নের সময় reference বা মানগুলোর কী হবে (যা ঠিক
করে কী, যদি কিছু থাকে, closure-এর _বাইরে_ সরানো হবে)।

একটি closure-এর body এর যেকোনোটি করতে পারে: capture করা মান closure থেকে বাইরে সরিয়ে নেওয়া, capture করা মান পরিবর্তন করা, মানটি
সরানো বা পরিবর্তন কোনোটিই না করা, অথবা শুরু থেকেই environment থেকে কিছুই capture না করা।

closure কীভাবে environment থেকে মান capture করে ও সামলায়, তা ঠিক করে closure কোন trait implement করবে, আর trait-এর মাধ্যমেই function
ও struct বলে দিতে পারে তারা কোন ধরনের closure ব্যবহার করতে পারে। closure-এর body মানগুলো কীভাবে সামলায় তার ওপর নির্ভর করে closure
স্বয়ংক্রিয়ভাবে এই `Fn` trait-গুলোর একটি, দুটি বা তিনটিই, যোগ হওয়ার ধাঁচে, implement করবে:

* `FnOnce` প্রযোজ্য সেই closure-এর ক্ষেত্রে যা একবার call করা যায়। সব closure অন্তত এই trait implement করে, কারণ সব closure call করা
  যায়। যে closure capture করা মান তার body থেকে বাইরে সরিয়ে নেয়, সেটি শুধু `FnOnce` implement করবে, অন্য কোনো `Fn` trait নয়, কারণ
  এটি মাত্র একবার call করা যায়।
* `FnMut` প্রযোজ্য সেই closure-এর ক্ষেত্রে যা capture করা মান তার body থেকে বাইরে সরায় না, কিন্তু capture করা মান পরিবর্তন করতে পারে।
  এই closure-গুলো একাধিকবার call করা যায়।
* `Fn` প্রযোজ্য সেই closure-এর ক্ষেত্রে যা capture করা মান তার body থেকে বাইরে সরায় না এবং capture করা মান পরিবর্তনও করে না, আর সেই
  closure-এর ক্ষেত্রেও যা environment থেকে কিছুই capture করে না। এই closure-গুলো environment পরিবর্তন না করেই একাধিকবার call করা যায়, যা গুরুত্বপূর্ণ, যেমন একই closure একসঙ্গে একাধিকবার call করার ক্ষেত্রে।

Listing 13-1-এ ব্যবহার করা `Option<T>`-এর `unwrap_or_else` method-এর সংজ্ঞাটি দেখা যাক:

```rust,ignore
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

মনে করুন, `T` হলো generic type, যা একটি `Option`-এর `Some` variant-এ থাকা মানের type বোঝায়। সেই `T` type-ই `unwrap_or_else` function-এর
return type: যেমন, একটি `Option<String>`-এর ওপর `unwrap_or_else` call করা কোড একটি `String` পাবে।

এরপর খেয়াল করুন, `unwrap_or_else` function-এর একটি বাড়তি generic type parameter `F` আছে। `F` type হলো `f` নামের parameter-এর type, যা
`unwrap_or_else` call করার সময় আমাদের দেওয়া closure।

generic type `F`-এর ওপর দেওয়া trait bound হলো `FnOnce() -> T`, অর্থাৎ `F`-কে একবার call করা যেতে হবে, কোনো argument নেবে না, এবং একটি `T`
return করবে। trait bound-এ `FnOnce` ব্যবহার করা এই শর্ত প্রকাশ করে যে `unwrap_or_else` `f`-কে একবারের বেশি call করবে না।
`unwrap_or_else`-এর body-তে দেখতে পাই, `Option` যদি `Some` হয়, তাহলে `f` call হবে না। `Option` যদি `None` হয়, তাহলে `f` একবার call হবে।
সব closure যেহেতু `FnOnce` implement করে, তাই `unwrap_or_else` তিন ধরনের closure-ই গ্রহণ করে এবং যতটা সম্ভব নমনীয়।

> Note: আমরা যা করতে চাই তাতে যদি environment থেকে কোনো মান capture করার দরকার না হয়, তাহলে যেখানে `Fn` trait-গুলোর কোনো একটি implement
> করে এমন কিছু লাগে, সেখানে closure-এর বদলে একটি function-এর নাম ব্যবহার করতে পারি। যেমন, একটি `Option<Vec<T>>` মানের ওপর
> `unwrap_or_else(Vec::new)` call করে মানটি `None` হলে একটি নতুন, খালি vector পেতে পারি। compiler একটি function-এর সংজ্ঞার জন্য `Fn`
> trait-গুলোর মধ্যে যেটি প্রযোজ্য, সেটি স্বয়ংক্রিয়ভাবে implement করে।

এবার slice-এর ওপর সংজ্ঞায়িত standard library-র method `sort_by_key` দেখা যাক, যাতে বোঝা যায় এটি `unwrap_or_else` থেকে কীভাবে আলাদা এবং
কেন `sort_by_key` trait bound-এ `FnOnce`-এর বদলে `FnMut` ব্যবহার করে। closure-টি একটি argument পায়, যা slice-এর বিবেচনাধীন বর্তমান
item-এর একটি reference, এবং এটি `K` type-এর এমন একটি মান return করে যা ক্রমানুসারে সাজানো যায়। প্রতিটি item-এর নির্দিষ্ট কোনো বৈশিষ্ট্য
অনুযায়ী একটি slice সাজাতে চাইলে এই function কাজে লাগে। Listing 13-7-এ আমাদের কাছে `Rectangle` instance-এর একটি তালিকা আছে, আর আমরা
`sort_by_key` ব্যবহার করে সেগুলোকে তাদের `width` বৈশিষ্ট্য অনুযায়ী কম থেকে বেশি ক্রমে সাজাই।

<Listing number="13-7" file-name="src/main.rs" caption="প্রস্থ অনুযায়ী আয়তক্ষেত্র সাজাতে `sort_by_key` ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-07/src/main.rs}}
```

</Listing>

এই কোড print করে:

```console
{{#include ../listings/ch13-functional-features/listing-13-07/output.txt}}
```

`sort_by_key`-কে একটি `FnMut` closure নেওয়ার মতো করে সংজ্ঞায়িত করার কারণ হলো এটি closure-টি একাধিকবার call করে: slice-এর প্রতিটি item-এর
জন্য একবার করে। `|r|
r.width` closure তার environment থেকে কিছু capture, পরিবর্তন বা বাইরে সরায় না, তাই এটি trait bound-এর শর্ত পূরণ করে।

বিপরীতে, Listing 13-8-এ এমন একটি closure-এর উদাহরণ দেখানো হয়েছে যা শুধু `FnOnce` trait implement করে, কারণ এটি environment থেকে একটি মান
বাইরে সরিয়ে নেয়। compiler আমাদের এই closure `sort_by_key`-এর সঙ্গে ব্যবহার করতে দেবে না।

<Listing number="13-8" file-name="src/main.rs" caption="`sort_by_key`-এর সঙ্গে একটি `FnOnce` closure ব্যবহারের চেষ্টা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-08/src/main.rs}}
```

</Listing>

`list` সাজানোর সময় `sort_by_key` কতবার closure call করে তা গোনার এটি একটি কৃত্রিম, জটিল (এবং কাজ করে না এমন) চেষ্টা। এই কোড গোনার কাজটি
করতে চায় `value`—closure-এর environment-এর একটি `String`—কে `sort_operations` vector-এ push করে। closure-টি `value` capture করে, তারপর
`value`-এর ownership `sort_operations` vector-এ হস্তান্তর করে `value`-কে closure থেকে বাইরে সরিয়ে নেয়। এই closure একবার call করা যায়;
দ্বিতীয়বার call করার চেষ্টা কাজ করবে না, কারণ তখন `sort_operations`-এ আবার push করার জন্য environment-এ আর `value` থাকবে না! তাই এই
closure শুধু `FnOnce` implement করে। এই কোড compile করার চেষ্টা করলে আমরা এই error পাই যে `value`-কে closure থেকে বাইরে সরানো যাবে না,
কারণ closure-কে `FnMut` implement করতে হবে:

```console
{{#include ../listings/ch13-functional-features/listing-13-08/output.txt}}
```

error-টি closure-এর body-র সেই লাইন দেখিয়ে দেয় যা `value`-কে environment থেকে বাইরে সরায়। এটি ঠিক করতে closure-এর body এমনভাবে বদলাতে
হবে যাতে এটি environment থেকে কোনো মান বাইরে না সরায়। environment-এ একটি counter রেখে closure-এর body-তে তার মান বাড়ানো closure কতবার call
হয়েছে তা গোনার আরও সোজাসাপ্টা উপায়। Listing 13-9-এর closure `sort_by_key`-এর সঙ্গে কাজ করে, কারণ এটি শুধু `num_sort_operations` counter-এর
একটি mutable reference capture করছে, তাই একাধিকবার call করা যায়।

<Listing number="13-9" file-name="src/main.rs" caption="`sort_by_key`-এর সঙ্গে একটি `FnMut` closure ব্যবহার করা অনুমোদিত।">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-09/src/main.rs}}
```

</Listing>

<!-- TODO: consider adding a section on the use<> operator -->

সংক্ষেপে, closure ব্যবহার করে এমন function বা type সংজ্ঞায়িত বা ব্যবহারের সময় `Fn` trait-গুলো গুরুত্বপূর্ণ। পরের অংশে আমরা iterator নিয়ে
আলোচনা করব। iterator-এর অনেক method closure argument নেয়, তাই এগোনোর সময় closure-এর এই খুঁটিনাটিগুলো মনে রাখবেন!

{{#quiz ../quizzes/ch13-01-closures-sec2.toml}}

[unwrap-or-else]: https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_else
[lifetime elision]: ch10-03-lifetime-syntax.html#lifetime-elision