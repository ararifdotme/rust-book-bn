<!-- Old headings. Do not remove or links may break. -->

<a id="defining-modules-to-control-scope-and-privacy"></a>

## module দিয়ে scope আর privacy নিয়ন্ত্রণ {#control-scope-and-privacy-with-modules}

এই অংশে আমরা module আর module system-এর অন্যান্য অংশ নিয়ে কথা বলব, যেমন _path_ (কোনো item-এর
নাম লেখার পথ, যেমন `a::b::c`), যা দিয়ে item-এর নাম দেওয়া যায়; `use` keyword, যা একটি path-কে
scope-এ আনে; আর `pub` keyword, যা item-কে public করে। আমরা `as` keyword, বাইরের package আর glob
operator নিয়েও আলোচনা করব।

### module-এর এক নজরের সারাংশ {#modules-cheat-sheet}

module আর path-এর খুঁটিনাটিতে যাওয়ার আগে এখানে একটি দ্রুত রেফারেন্স দেওয়া হলো: compiler-এ module,
path, `use` keyword আর `pub` keyword কীভাবে কাজ করে, আর বেশিরভাগ ডেভেলপার কীভাবে তাদের কোড গুছিয়ে
রাখেন। এই অধ্যায়জুড়ে আমরা এই প্রতিটি নিয়মের উদাহরণ দেখব, তবে module কীভাবে কাজ করে তা মনে করার
জন্য এটি ফিরে দেখার চমৎকার জায়গা।

- **crate root থেকে শুরু**: একটি crate compile করার সময় compiler প্রথমে compile করার মতো কোড
  খোঁজে crate root ফাইলে (সাধারণত library crate-এর জন্য _src/lib.rs_ আর binary crate-এর জন্য
  _src/main.rs_)।
- **module ঘোষণা করা**: crate root ফাইলে আপনি নতুন module ঘোষণা করতে পারেন; ধরুন আপনি `mod garden;`
  দিয়ে একটি “garden” module ঘোষণা করলেন। compiler module-টির কোড খুঁজবে এই জায়গাগুলোতে:
  - সেখানেই (inline), কার্লি ব্র্যাকেটের ভেতরে, যা `mod
    garden`-এর পরের সেমিকোলনের জায়গা নেয়
  - _src/garden.rs_ ফাইলে
  - _src/garden/mod.rs_ ফাইলে
- **submodule ঘোষণা করা**: crate root ছাড়া অন্য যেকোনো ফাইলে আপনি submodule (module-এর ভেতরের
  module) ঘোষণা করতে পারেন। যেমন, আপনি _src/garden.rs_-এ `mod vegetables;` ঘোষণা করতে পারেন।
  compiler submodule-টির কোড খুঁজবে parent module-এর নামের ডিরেক্টরির ভেতরে, এই জায়গাগুলোতে:
  - সেখানেই (inline), ঠিক `mod vegetables`-এর পরে, সেমিকোলনের বদলে কার্লি ব্র্যাকেটের
    ভেতরে
  - _src/garden/vegetables.rs_ ফাইলে
  - _src/garden/vegetables/mod.rs_ ফাইলে
- **module-এর কোডের path**: একবার একটি module আপনার crate-এর অংশ হয়ে গেলে, privacy-র নিয়ম অনুমতি
  দিলে, কোডের path ব্যবহার করে একই crate-এর যেকোনো জায়গা থেকে সেই module-এর কোড উল্লেখ করতে পারেন।
  যেমন, garden vegetables module-এর একটি `Asparagus` type পাওয়া যাবে
  `crate::garden::vegetables::Asparagus`-এ।
- **private বনাম public**: একটি module-এর ভেতরের কোড ডিফল্টভাবে তার parent module-গুলোর কাছে
  private। একটি module-কে public করতে `mod`-এর বদলে `pub mod` দিয়ে ঘোষণা করুন। একটি public module-এর
  ভেতরের item-গুলোকেও public করতে তাদের ঘোষণার আগে `pub` ব্যবহার করুন।
- **`use` keyword**: একটি scope-এর ভেতরে `use` keyword item-গুলোর শর্টকাট তৈরি করে, যাতে লম্বা
  path বারবার লিখতে না হয়। যে scope `crate::garden::vegetables::Asparagus` উল্লেখ করতে পারে, সেখানে
  আপনি `use
  crate::garden::vegetables::Asparagus;` দিয়ে একটি শর্টকাট তৈরি করতে পারেন, আর তারপর থেকে ওই scope-এ
  সেই type ব্যবহার করতে শুধু `Asparagus` লিখলেই চলবে।

এখানে আমরা `backyard` নামে একটি binary crate তৈরি করছি, যা এই নিয়মগুলো দেখায়। crate-টির ডিরেক্টরির
নামও _backyard_, আর এতে আছে এই ফাইল আর ডিরেক্টরিগুলো:

```text
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

এই ক্ষেত্রে crate root ফাইল হলো _src/main.rs_, আর এতে আছে:

<Listing file-name="src/main.rs">

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/main.rs}}
```

</Listing>

`pub mod garden;` লাইনটি compiler-কে বলে _src/garden.rs_-এ যে কোড পাবে, তা যোগ করতে, যা
হলো:

<Listing file-name="src/garden.rs">

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/garden.rs}}
```

</Listing>

এখানে `pub mod vegetables;` মানে _src/garden/vegetables.rs_-এর কোডও যোগ করা হবে। সেই কোডটি
হলো:

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/garden/vegetables.rs}}
```

এবার চলুন এই নিয়মগুলোর খুঁটিনাটিতে ঢুকি এবং কাজের মধ্যে সেগুলো দেখি!

### সম্পর্কিত কোড module-এ একত্র করা {#grouping-related-code-in-modules}

_module_ দিয়ে আমরা একটি crate-এর ভেতরের কোড পাঠযোগ্যতা আর সহজে আবার ব্যবহারের জন্য গুছিয়ে রাখতে
পারি। module দিয়ে item-গুলোর _privacy_ (গোপনীয়তা, কে কী দেখতে পারবে)-ও নিয়ন্ত্রণ করা যায়, কারণ একটি
module-এর ভেতরের কোড ডিফল্টভাবে private (বাইরে থেকে ব্যবহার করা যায় না)। private item হলো ভেতরের
implementation-এর খুঁটিনাটি, যা বাইরের ব্যবহারের জন্য পাওয়া যায় না। আমরা module আর তাদের ভেতরের
item-গুলোকে public করতে বেছে নিতে পারি, যা সেগুলোকে প্রকাশ করে, যাতে বাইরের কোড সেগুলো ব্যবহার করতে
ও তাদের ওপর নির্ভর করতে পারে।

উদাহরণ হিসেবে, চলুন একটি library crate লিখি, যা একটি রেস্তোরাঁর কার্যকারিতা দেয়। আমরা function-এর
signature সংজ্ঞায়িত করব, কিন্তু body ফাঁকা রাখব, যাতে রেস্তোরাঁর implementation-এর বদলে কোডের
সংগঠনে মনোযোগ দেওয়া যায়।

রেস্তোরাঁ শিল্পে রেস্তোরাঁর কিছু অংশকে বলা হয় front of house (সামনের অংশ), আর কিছু অংশকে back of
house (পেছনের অংশ)। _front of house_ হলো যেখানে খদ্দেররা থাকেন; এর মধ্যে আছে যেখানে host খদ্দেরদের
বসান, পরিবেশনকারীরা অর্ডার আর টাকা নেন, আর bartender পানীয় বানান। _back of house_ হলো যেখানে
রান্নাঘরে শেফ আর রাঁধুনিরা কাজ করেন, থালাবাসন ধোয়া হয়, আর ম্যানেজাররা প্রশাসনিক কাজ করেন।

আমাদের crate-কে এভাবে সাজাতে আমরা এর function-গুলোকে একটির ভেতরে আরেকটি, এমন module-এ গুছিয়ে
রাখতে পারি। `cargo new
restaurant --lib` চালিয়ে `restaurant` নামে একটি নতুন library তৈরি করুন। তারপর কিছু module আর function
signature সংজ্ঞায়িত করতে Listing 7-1-এর কোডটি _src/lib.rs_-এ লিখুন; এই কোডটি front of house অংশ।

<Listing number="7-1" file-name="src/lib.rs" caption="একটি `front_of_house` module, যার ভেতরে অন্য module আছে, আর সেগুলোর ভেতরে আছে function">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-01/src/lib.rs}}
```

</Listing>

module সংজ্ঞায়িত করতে আমরা `mod` keyword লিখি, তারপর module-এর নাম (এই ক্ষেত্রে `front_of_house`)।
তারপর module-এর body যায় কার্লি ব্র্যাকেটের ভেতরে। module-এর ভেতরে আমরা অন্য module রাখতে পারি,
যেমন এই ক্ষেত্রে `hosting` আর `serving` module। module-এ অন্যান্য item-এর সংজ্ঞাও রাখা যায়, যেমন
struct, enum, constant, trait, এবং Listing 7-1-এর মতো function।

module ব্যবহার করে আমরা সম্পর্কিত সংজ্ঞাগুলো একত্র করে রাখতে পারি এবং সেগুলো কেন সম্পর্কিত, তার নাম
দিতে পারি। এই কোড ব্যবহারকারী প্রোগ্রামাররা সব সংজ্ঞা পড়ে যাওয়ার বদলে দলগুলোর ভিত্তিতে কোডে ঘুরে
বেড়াতে পারেন, ফলে তাদের জন্য প্রাসঙ্গিক সংজ্ঞা খুঁজে পাওয়া সহজ হয়। এই কোডে নতুন কার্যকারিতা যোগ
করা প্রোগ্রামাররা জানবেন প্রোগ্রাম গোছানো রাখতে কোড কোথায় রাখতে হবে।

আগে আমরা বলেছিলাম, _src/main.rs_ আর _src/lib.rs_-কে বলা হয় _crate
root_। এদের এই নামের কারণ হলো, এই দুটি ফাইলের যেকোনোটির বিষয়বস্তু crate-এর module কাঠামোর মূলে
`crate` নামের একটি module গঠন করে, যাকে বলা হয় _module tree_ (module-গুলোর গাছের মতো কাঠামো)।

Listing 7-2-এ Listing 7-1-এর কাঠামোর module tree দেখানো হয়েছে।

<Listing number="7-2" caption="Listing 7-1-এর কোডের module tree">

```text
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

</Listing>

এই গাছটি দেখায় কিছু module কীভাবে অন্য module-এর ভেতরে থাকে; যেমন, `hosting` থাকে `front_of_house`-এর
ভেতরে। গাছটি এটাও দেখায় যে কিছু module হলো _sibling_ (একই স্তরের module), অর্থাৎ এরা একই module-এ
সংজ্ঞায়িত; `hosting` আর `serving` হলো `front_of_house`-এর ভেতরে সংজ্ঞায়িত sibling। module A যদি module
B-এর ভেতরে থাকে, তাহলে আমরা বলি module A হলো module B-এর _child_ (ভেতরের module), আর module B হলো
module A-এর _parent_ (ওপরের module)। লক্ষ করুন, পুরো module tree-র মূল হলো `crate` নামের অন্তর্নিহিত
module।

module tree হয়তো আপনাকে আপনার কম্পিউটারের ফাইল সিস্টেমের ডিরেক্টরি-গাছের কথা মনে করিয়ে দেয়; এটি
খুবই যথার্থ তুলনা! ফাইল সিস্টেমের ডিরেক্টরির মতোই আপনি কোড গুছিয়ে রাখতে module ব্যবহার করেন। আর
একটি ডিরেক্টরির ফাইলের মতোই আমাদের module-গুলো খুঁজে পাওয়ার একটি উপায় দরকার।

{{#quiz ../quizzes/ch07-02-modules.toml}}