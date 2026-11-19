## hash map-এ key-এর সঙ্গে যুক্ত মান রাখা {#storing-keys-with-associated-values-in-hash-maps}

আমাদের সাধারণ collection-গুলোর শেষটি হলো hash map। `HashMap<K, V>` type একটি _hashing function_ (key থেকে একটি
সংখ্যা, অর্থাৎ hash, হিসাব করার function) ব্যবহার করে `K` type-এর key থেকে `V` type-এর মানের একটি ম্যাপিং রাখে; এই
function নির্ধারণ করে key আর মানগুলো মেমোরিতে কীভাবে রাখা হবে। অনেক প্রোগ্রামিং ভাষা এ ধরনের data structure সমর্থন
করে, তবে প্রায়ই ভিন্ন নামে, যেমন _hash_, _map_, _object_, _hash table_, _dictionary_ বা _associative array_, কয়েকটির
নাম বললে।

hash map কাজে আসে যখন আপনি vector-এর মতো index দিয়ে নয়, বরং যেকোনো type-এর হতে পারে এমন একটি key দিয়ে ডেটা খুঁজতে
চান। যেমন, একটি খেলায় আপনি প্রতিটি দলের স্কোরের হিসাব একটি hash map-এ রাখতে পারেন, যেখানে প্রতিটি key হলো একটি দলের
নাম, আর মানগুলো হলো প্রতিটি দলের স্কোর। একটি দলের নাম দিলে আপনি তার স্কোর বের করতে পারেন।

এই অংশে আমরা hash map-এর মৌলিক API দেখব, তবে standard library `HashMap<K, V>`-এর ওপর যে function-গুলো সংজ্ঞায়িত
করেছে, সেগুলোতে আরও অনেক দারুণ জিনিস লুকিয়ে আছে। বরাবরের মতো, আরও তথ্যের জন্য standard library-র ডকুমেন্টেশন দেখুন।

### একটি নতুন hash map তৈরি করা {#creating-a-new-hash-map}

একটি ফাঁকা hash map তৈরির একটি উপায় হলো `new` ব্যবহার করা, আর `insert` দিয়ে উপাদান যোগ করা। Listing 8-20-এ আমরা
দুটি দলের স্কোরের হিসাব রাখছি, যাদের নাম _Blue_ আর _Yellow_। Blue দল শুরু করে 10 পয়েন্ট দিয়ে, আর Yellow দল শুরু করে
50 দিয়ে।

<Listing number="8-20" caption="একটি নতুন hash map তৈরি করে কিছু key আর মান ঢোকানো">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-20/src/main.rs:here}}
```

</Listing>

লক্ষ করুন, আমাদের প্রথমে standard library-র collections অংশ থেকে `HashMap` `use` করতে হবে। আমাদের তিনটি সাধারণ
collection-এর মধ্যে এটিই সবচেয়ে কম ব্যবহৃত, তাই prelude-এ আপনা-আপনি scope-এ আসা বৈশিষ্ট্যগুলোর মধ্যে এটি রাখা নেই।
standard library-তে hash map-এর সমর্থনও কম; যেমন, এগুলো তৈরির কোনো বিল্ট-ইন macro নেই।

vector-এর মতোই hash map তাদের ডেটা heap-এ রাখে। এই `HashMap`-এর key-গুলো `String` type-এর আর মানগুলো `i32`
type-এর। vector-এর মতোই hash map একজাতীয়: সব key-এর type একই হতে হবে, আর সব মানের type-ও একই হতে হবে।

### hash map-এর মানে পৌঁছানো {#accessing-values-in-a-hash-map}

`get` method-কে key দিয়ে আমরা hash map থেকে একটি মান বের করতে পারি, Listing 8-21-এ যেমন দেখানো হয়েছে।

<Listing number="8-21" caption="hash map-এ রাখা Blue দলের স্কোরে পৌঁছানো">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-21/src/main.rs:here}}
```

</Listing>

এখানে `score`-এ থাকবে Blue দলের সঙ্গে যুক্ত মান, আর ফলাফল হবে `10`। `get` method একটি `Option<&V>` return করে; hash
map-এ ওই key-এর জন্য কোনো মান না থাকলে `get` `None` return করবে। এই প্রোগ্রাম `Option`-টি সামলায় প্রথমে `copied`
call করে, যাতে `Option<&i32>`-এর বদলে একটি `Option<i32>` পাওয়া যায়, তারপর `unwrap_or` call করে, যাতে `scores`-এ ওই
key-এর কোনো entry না থাকলে `score` শূন্য হয়।

vector-এর মতো একই উপায়ে, একটি `for` loop ব্যবহার করে আমরা hash map-এর প্রতিটি key-value জোড়ার ওপর iterate করতে
পারি:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-03-iterate-over-hashmap/src/main.rs:here}}
```

এই কোড প্রতিটি জোড়া এলোমেলো ক্রমে প্রিন্ট করবে:

```text
Yellow: 50
Blue: 10
```

<!-- Old headings. Do not remove or links may break. -->

<a id="hash-maps-and-ownership"></a>

### hash map-এ ownership সামলানো {#managing-ownership-in-hash-maps}

`i32`-এর মতো যেসব type `Copy` trait implement করে, সেগুলোর মান hash map-এ কপি হয়। `String`-এর মতো owned মানের
ক্ষেত্রে মানগুলো move হবে, আর hash map সেই মানগুলোর owner হবে, Listing 8-22-এ যেমন দেখানো হয়েছে।

<Listing number="8-22" caption="দেখানো যে key আর মান ঢোকানোর পর সেগুলোর owner হয়ে যায় hash map">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-22/src/main.rs:here}}
```

</Listing>

`insert` call দিয়ে hash map-এ move হয়ে যাওয়ার পর আমরা `field_name` আর `field_value` variable আর ব্যবহার করতে পারি
না।

hash map-এ যদি মানের reference ঢোকাই, তাহলে মানগুলো hash map-এ move হবে না। reference-গুলো যে মানকে নির্দেশ করে,
সেগুলোকে অন্তত ততক্ষণ বৈধ থাকতে হবে, যতক্ষণ hash map বৈধ থাকে। এই বিষয়গুলো নিয়ে আমরা অধ্যায় 10-এর [“Validating
References with
Lifetimes”][validating-references-with-lifetimes]<!-- ignore --> অংশে আরও কথা বলব।

### একটি hash map আপডেট করা {#updating-a-hash-map}

key আর মানের জোড়ার সংখ্যা বাড়তে পারলেও প্রতিটি অনন্য key-এর সঙ্গে এক সময়ে শুধু একটি মান যুক্ত থাকতে পারে (তবে
উল্টোটা নয়: যেমন, Blue দল আর Yellow দল দুটোরই `scores` hash map-এ `10` মান রাখা থাকতে পারে)।

hash map-এর ডেটা বদলাতে চাইলে আপনাকে ঠিক করতে হবে, কোনো key-এর সঙ্গে আগে থেকেই একটি মান assign করা থাকলে সেই
ক্ষেত্রটি কীভাবে সামলাবেন। আপনি পুরোনো মান পুরোপুরি উপেক্ষা করে নতুন মান দিয়ে বদলে দিতে পারেন। আপনি পুরোনো মান রেখে
নতুন মান উপেক্ষা করতে পারেন, শুধু key-এর আগে থেকে কোনো মান _না_ থাকলে নতুন মানটি যোগ করে। অথবা পুরোনো আর নতুন মান
মিলিয়ে দিতে পারেন। চলুন দেখি এর প্রতিটি কীভাবে করতে হয়!

#### একটি মানের ওপর নতুন করে লেখা {#overwriting-a-value}

hash map-এ একটি key আর একটি মান ঢুকিয়ে তারপর একই key আলাদা মানসহ ঢোকালে ওই key-এর সঙ্গে যুক্ত মানটি বদলে যাবে।
Listing 8-23-এর কোড দুবার `insert` call করলেও hash map-এ শুধু একটি key-value জোড়া থাকবে, কারণ দুবারই আমরা Blue দলের
key-এর জন্য মান ঢোকাচ্ছি।

<Listing number="8-23" caption="একটি নির্দিষ্ট key-এর সঙ্গে রাখা মান বদলে দেওয়া">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-23/src/main.rs:here}}
```

</Listing>

এই কোড প্রিন্ট করবে `{"Blue": 25}`। মূল মান `10`-এর ওপর নতুন করে লেখা হয়েছে।

<!-- Old headings. Do not remove or links may break. -->

<a id="only-inserting-a-value-if-the-key-has-no-value"></a>

#### key না থাকলেই শুধু key আর মান যোগ করা {#adding-a-key-and-value-only-if-a-key-isnt-present}

hash map-এ একটি নির্দিষ্ট key আগে থেকেই কোনো মানসহ আছে কি না যাচাই করে তারপর এই কাজগুলো করা প্রচলিত: key hash map-এ
থাকলে বিদ্যমান মান যেমন আছে তেমনই থাকবে; key না থাকলে key আর তার একটি মান ঢোকানো হবে।

এর জন্য hash map-এর একটি বিশেষ API আছে, যার নাম `entry`, যা যাচাই করতে চাওয়া key-টিকে parameter হিসেবে নেয়। `entry`
method-এর return মান হলো `Entry` নামের একটি enum, যা এমন একটি মান প্রকাশ করে, যা থাকতেও পারে, না-ও থাকতে পারে। ধরুন
আমরা যাচাই করতে চাই Yellow দলের key-এর সঙ্গে কোনো মান যুক্ত আছে কি না। না থাকলে আমরা `50` মান ঢোকাতে চাই, আর Blue দলের
জন্যও একই কাজ। `entry` API ব্যবহার করলে কোডটি দেখতে Listing 8-24-এর মতো।

<Listing number="8-24" caption="key-এর আগে থেকে কোনো মান না থাকলে তবেই ঢোকাতে `entry` method ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-24/src/main.rs:here}}
```

</Listing>

`Entry`-এর `or_insert` method এমনভাবে সংজ্ঞায়িত যে সংশ্লিষ্ট `Entry` key থাকলে এটি সেই key-এর মানের একটি mutable
reference return করে, আর না থাকলে parameter-টিকে এই key-এর নতুন মান হিসেবে ঢুকিয়ে নতুন মানের একটি mutable reference
return করে। এই কৌশল নিজেরা যুক্তি লেখার চেয়ে অনেক পরিচ্ছন্ন, আর borrow checker-এর সঙ্গেও বেশি ভালোভাবে মানিয়ে চলে।

Listing 8-24-এর কোড চালালে প্রিন্ট হবে `{"Yellow": 50, "Blue": 10}`। `entry`-এর প্রথম call Yellow দলের key `50` মানসহ
ঢোকাবে, কারণ Yellow দলের আগে থেকে কোনো মান নেই। `entry`-এর দ্বিতীয় call hash map বদলাবে না, কারণ Blue দলের আগে থেকেই
`10` মান আছে।

#### পুরোনো মানের ভিত্তিতে একটি মান আপডেট করা {#updating-a-value-based-on-the-old-value}

hash map-এর আরেকটি প্রচলিত ব্যবহার হলো একটি key-এর মান খুঁজে তারপর পুরোনো মানের ভিত্তিতে তা আপডেট করা। যেমন,
Listing 8-25-এ এমন কোড দেখানো হয়েছে, যা কোনো লেখায় প্রতিটি শব্দ কতবার এসেছে তা গোনে। আমরা শব্দগুলোকে key হিসেবে
রেখে একটি hash map ব্যবহার করি, আর একটি শব্দ কতবার দেখেছি তার হিসাব রাখতে মান বাড়াই। কোনো শব্দ প্রথমবার দেখলে আমরা
প্রথমে `0` মান ঢোকাব।

<Listing number="8-25" caption="শব্দ আর তার গণনা রাখা একটি hash map ব্যবহার করে শব্দের উপস্থিতি গোনা">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-25/src/main.rs:here}}
```

</Listing>

এই কোড প্রিন্ট করবে `{"world": 2, "hello": 1, "wonderful": 1}`। একই key-value জোড়াগুলো হয়তো ভিন্ন ক্রমে প্রিন্ট হতে
দেখবেন: [“hash map-এর মানে
পৌঁছানো”][access]<!-- ignore --> থেকে মনে করুন, hash map-এর ওপর iterate করা এলোমেলো ক্রমে ঘটে।

`split_whitespace` method `text`-এর মানের ফাঁকা অক্ষর দিয়ে আলাদা করা উপ-slice-গুলোর ওপর একটি iterator return করে।
`or_insert` method নির্দিষ্ট key-এর মানের একটি mutable reference (`&mut V`) return করে। এখানে আমরা সেই mutable
reference `count` variable-এ রাখি, তাই সেই মানে assign করতে আমাদের প্রথমে তারকাচিহ্ন (`*`) দিয়ে `count`-কে dereference
করতে হয়। mutable reference-টি `for` loop-এর শেষে scope-এর বাইরে চলে যায়, তাই এই সব পরিবর্তনই নিরাপদ এবং borrowing-এর
নিয়মে অনুমোদিত।

### hashing function {#hashing-functions}

ডিফল্টভাবে `HashMap` _SipHash_ নামের একটি hashing function ব্যবহার করে, যা hash table জড়িত denial-of-service (DoS)
আক্রমণ ঠেকাতে পারে[^siphash]<!-- ignore -->। এটি পাওয়া যায় এমন সবচেয়ে দ্রুত hashing algorithm নয়, তবে পারফরম্যান্স
কমার বিনিময়ে বাড়তি নিরাপত্তার এই আপসটুকু সার্থক। আপনি যদি আপনার কোড profile (কোন অংশে কত সময় লাগে তা মাপা) করে দেখেন
যে ডিফল্ট hash function আপনার উদ্দেশ্যের জন্য খুব ধীর, তাহলে আলাদা একটি hasher নির্দিষ্ট করে অন্য function-এ যেতে
পারেন। _hasher_ (hash function তৈরি করে এমন type) হলো এমন একটি type, যা `BuildHasher` trait implement করে। trait আর সেগুলো
কীভাবে implement করতে হয়, তা নিয়ে আমরা [অধ্যায় 10][traits]<!-- ignore -->-এ কথা বলব। আপনাকে যে একেবারে শুরু থেকে নিজের
hasher implement করতে হবে, এমন নয়; [crates.io](https://crates.io/)<!-- ignore -->-তে অন্য Rust ব্যবহারকারীদের শেয়ার
করা library আছে, যেগুলো অনেক প্রচলিত hashing algorithm implement করা hasher দেয়।

[^siphash]: [https://en.wikipedia.org/wiki/SipHash](https://en.wikipedia.org/wiki/SipHash)

{{#quiz ../quizzes/ch08-03-hashmap.toml}}

## সারসংক্ষেপ {#summary}

ডেটা রাখা, তাতে পৌঁছানো আর বদলানোর দরকার হলে প্রোগ্রামে প্রয়োজনীয় কার্যকারিতার বড় একটি অংশ vector, string আর hash
map দেবে। এখন আপনি এই অনুশীলনীগুলো সমাধান করতে পারার মতো প্রস্তুত:

1. integer-এর একটি তালিকা দেওয়া থাকলে, একটি vector ব্যবহার করে তালিকাটির median (সাজানোর পর মাঝখানের অবস্থানের মান)
   আর mode (সবচেয়ে বেশিবার আসা মান; এখানে একটি hash map কাজে আসবে) return করুন।
1. string-কে Pig Latin-এ রূপান্তর করুন। প্রতিটি শব্দের প্রথম ব্যঞ্জনবর্ণ শব্দের শেষে সরে যায় এবং _ay_ যোগ হয়, ফলে
   _first_ হয়ে যায় _irst-fay_। স্বরবর্ণ দিয়ে শুরু হওয়া শব্দের শেষে এর বদলে _hay_ যোগ হয় (_apple_ হয়ে যায়
   _apple-hay_)। UTF-8 এনকোডিংয়ের খুঁটিনাটি মাথায় রাখবেন!
1. একটি hash map আর vector ব্যবহার করে এমন একটি লেখাভিত্তিক ইন্টারফেস তৈরি করুন, যাতে ব্যবহারকারী একটি কোম্পানির একটি
   বিভাগে কর্মীর নাম যোগ করতে পারেন; যেমন, “Add Sally to Engineering” বা “Add Amir to Sales”। তারপর ব্যবহারকারীকে
   একটি বিভাগের সব মানুষের, বা বিভাগ অনুযায়ী কোম্পানির সব মানুষের, বর্ণানুক্রমে সাজানো তালিকা বের করতে
   দিন।

standard library-র API ডকুমেন্টেশনে vector, string আর hash map-এর এমন method-গুলোর বর্ণনা আছে, যা এই অনুশীলনীগুলোতে
কাজে আসবে!

আমরা এমন আরও জটিল প্রোগ্রামের দিকে যাচ্ছি, যেখানে operation ব্যর্থ হতে পারে, তাই error handling নিয়ে আলোচনার এটাই
উপযুক্ত সময়। পরের অংশে আমরা সেটিই করব!

[validating-references-with-lifetimes]: ch10-03-lifetime-syntax.html#validating-references-with-lifetimes
[access]: #accessing-values-in-a-hash-map
[traits]: ch10-02-traits.html
