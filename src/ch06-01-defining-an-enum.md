## enum সংজ্ঞায়িত করা {#defining-an-enum}

struct যেখানে সম্পর্কিত field আর ডেটা একত্র করার উপায় দেয়, যেমন `width` আর `height`-সহ
একটি `Rectangle`, সেখানে enum বলার উপায় দেয় যে একটি মান সম্ভাব্য কয়েকটি মানের একটি।
যেমন, আমরা হয়তো বলতে চাই যে `Rectangle` সম্ভাব্য কয়েকটি আকৃতির একটি, যার মধ্যে
`Circle` আর `Triangle`-ও আছে। এটি করতে Rust আমাদের এই সম্ভাবনাগুলোকে একটি enum হিসেবে
প্রকাশ করতে দেয়।

চলুন এমন একটি পরিস্থিতি দেখি, যা আমরা কোডে প্রকাশ করতে চাইতে পারি, আর দেখি এই
ক্ষেত্রে struct-এর চেয়ে enum কেন বেশি কাজের ও উপযুক্ত। ধরুন আমাদের IP address নিয়ে
কাজ করতে হবে। বর্তমানে IP address-এর জন্য দুটি প্রধান মানদণ্ড ব্যবহার হয়: version
four আর version six। আমাদের প্রোগ্রাম যেহেতু IP address-এর শুধু এই দুটি সম্ভাবনারই
মুখোমুখি হবে, তাই আমরা সব সম্ভাব্য variant একে একে তালিকাভুক্ত (_enumerate_) করতে পারি,
আর এখান থেকেই enumeration নামটি এসেছে।

যেকোনো IP address হয় version four নয়তো version six address হতে পারে, কিন্তু একই সঙ্গে
দুটোই নয়। IP address-এর এই বৈশিষ্ট্যের কারণে enum data structure (ডেটা সাজানোর কাঠামো)
এখানে উপযুক্ত, কারণ একটি enum মান তার variant-গুলোর শুধু একটিই হতে পারে। version four
আর version six দুই ধরনের address-ই মূলত IP address, তাই যেকোনো ধরনের IP address-এর
ক্ষেত্রে প্রযোজ্য পরিস্থিতি সামলানোর সময় কোডে এদের একই type হিসেবে দেখা উচিত।

এই ধারণাটি আমরা কোডে প্রকাশ করতে পারি একটি `IpAddrKind` enumeration সংজ্ঞায়িত করে
এবং একটি IP address যে সম্ভাব্য ধরনগুলো হতে পারে, অর্থাৎ `V4` আর `V6`, তালিকাভুক্ত করে।
এগুলোই enum-টির variant:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:def}}
```

`IpAddrKind` এখন একটি নিজস্ব data type (ডেটার ধরন), যা আমরা আমাদের কোডের অন্য জায়গায়
ব্যবহার করতে পারি।

### enum-এর মান {#enum-values}

`IpAddrKind`-এর দুটি variant-এর প্রতিটির instance (কোনো type-এর একটি নির্দিষ্ট মান) আমরা
এভাবে তৈরি করতে পারি:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:instance}}
```

লক্ষ করুন, enum-এর variant-গুলো তার নামের namespace (নামগুলোর আলাদা একটি পরিসর)-এর
ভেতরে থাকে, আর দুটিকে আলাদা করতে আমরা দুটি কোলন ব্যবহার করি। এটি কাজের, কারণ এখন
`IpAddrKind::V4` আর `IpAddrKind::V6` দুটি মানই একই type-এর: `IpAddrKind`। এরপর আমরা,
যেমন, এমন একটি function সংজ্ঞায়িত করতে পারি, যা যেকোনো `IpAddrKind` নেয়:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn}}
```

আর যেকোনো variant দিয়ে এই function call করতে পারি:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn_call}}
```

enum ব্যবহারের আরও সুবিধা আছে। আমাদের IP address type নিয়ে আরেকটু ভাবলে দেখা যায়,
এই মুহূর্তে প্রকৃত IP address-এর _ডেটা_ রাখার কোনো উপায় আমাদের নেই; আমরা শুধু জানি
এটি কোন _ধরনের_। অধ্যায় 5-এ আপনি সদ্য struct সম্পর্কে শিখেছেন, তাই হয়তো Listing 6-1-এর
মতো করে struct দিয়ে এই সমস্যা সমাধানের ইচ্ছা হতে পারে।

```aquascope,interpreter
#fn main() {
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};`[]`
#}
```

এখানে আমরা `IpAddr` নামে একটি struct সংজ্ঞায়িত করেছি, যার দুটি field: `IpAddrKind`
type-এর (আগে সংজ্ঞায়িত enum) একটি `kind` field, আর `String` type-এর একটি `address`
field। এই struct-এর দুটি instance আছে আমাদের। প্রথমটি `home`, এবং এর `kind` হলো
`IpAddrKind::V4` মান, সঙ্গে যুক্ত address ডেটা `127.0.0.1`। দ্বিতীয় instance হলো
`loopback`। এর `kind` মান হলো `IpAddrKind`-এর অন্য variant `V6`, আর এর সঙ্গে যুক্ত
address হলো `::1`। `kind` আর `address` মান দুটিকে একসঙ্গে বাঁধতে আমরা একটি struct
ব্যবহার করেছি, তাই এখন variant-টি মানের সঙ্গে যুক্ত।

তবে শুধু একটি enum দিয়ে একই ধারণা প্রকাশ করা আরও সংক্ষিপ্ত: struct-এর ভেতরে enum রাখার
বদলে আমরা ডেটা সরাসরি প্রতিটি enum variant-এ রাখতে পারি। `IpAddr` enum-এর এই নতুন সংজ্ঞা
বলে যে `V4` আর `V6` দুটি variant-এরই সঙ্গে যুক্ত `String` মান থাকবে:

```aquascope,interpreter
#fn main() {    
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));`[]`
#}
```

আমরা enum-এর প্রতিটি variant-এ সরাসরি ডেটা জুড়ে দিই, তাই বাড়তি struct-এর দরকার নেই।
এখানে enum কীভাবে কাজ করে তার আরেকটি খুঁটিনাটিও দেখা সহজ: আমরা যে প্রতিটি enum variant
সংজ্ঞায়িত করি, তার নামটিও এমন একটি function হয়ে যায়, যা enum-এর একটি instance তৈরি করে।
অর্থাৎ `IpAddr::V4()` হলো একটি function call, যা একটি `String` argument নিয়ে `IpAddr`
type-এর একটি instance return করে। enum সংজ্ঞায়িত করার ফলেই আমরা আপনা-আপনি এই constructor
(নতুন instance তৈরি করে এমন function) function সংজ্ঞায়িত অবস্থায় পেয়ে যাই।

struct-এর বদলে enum ব্যবহারের আরেকটি সুবিধা আছে: প্রতিটি variant-এর সঙ্গে যুক্ত ডেটার
type আর পরিমাণ আলাদা হতে পারে। version four IP address-এ সবসময় চারটি সংখ্যার অংশ থাকবে,
যাদের মান 0 থেকে 255-এর মধ্যে। আমরা যদি `V4` address-কে চারটি `u8` মান হিসেবে রাখতে চাই,
কিন্তু `V6` address-কে তবু একটি `String` মান হিসেবে প্রকাশ করতে চাই, তাহলে struct দিয়ে
তা পারতাম না। enum এই ক্ষেত্রটি সহজেই সামলায়:

```aquascope,interpreter
#fn main() {
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));`[]`
#}

```

version four আর version six IP address রাখার জন্য data structure সংজ্ঞায়িত করার কয়েকটি
আলাদা উপায় আমরা দেখালাম। তবে দেখা যাচ্ছে, IP address রাখা আর সেগুলো কোন ধরনের তা প্রকাশ
করার প্রয়োজন এতই প্রচলিত যে [standard library-তে আমাদের ব্যবহারের উপযোগী একটি সংজ্ঞা
আছে!][IpAddr]<!-- ignore --> চলুন দেখি standard library (আদর্শ লাইব্রেরি) `IpAddr` কীভাবে
সংজ্ঞায়িত করে। এতে হুবহু সেই enum আর variant আছে, যা আমরা সংজ্ঞায়িত ও ব্যবহার করেছি,
কিন্তু এটি address-এর ডেটা variant-গুলোর ভেতরে রাখে দুটি আলাদা struct-এর আকারে, যেগুলো
প্রতিটি variant-এর জন্য আলাদাভাবে সংজ্ঞায়িত:

```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

এই কোড দেখায় যে একটি enum variant-এর ভেতরে আপনি যেকোনো ধরনের ডেটা রাখতে পারেন: যেমন
string, সংখ্যার type, বা struct। এমনকি আরেকটি enum-ও রাখতে পারেন! আর standard library-র
type-গুলো প্রায়ই আপনি নিজে যা ভেবে বের করতেন, তার চেয়ে খুব বেশি জটিল নয়।

লক্ষ করুন, standard library-তে `IpAddr`-এর একটি সংজ্ঞা থাকলেও আমরা কোনো সংঘাত ছাড়াই
আমাদের নিজস্ব সংজ্ঞা তৈরি ও ব্যবহার করতে পারি, কারণ standard library-র সংজ্ঞাটি আমরা
আমাদের scope (কোডের যে অংশে একটি নাম ব্যবহার করা যায়)-এ আনিনি। type-কে scope-এ আনা নিয়ে
আমরা অধ্যায় 7-এ আরও কথা বলব।

Listing 6-2-এ চলুন enum-এর আরেকটি উদাহরণ দেখি: এর variant-গুলোতে নানা ধরনের type বসানো আছে।

<Listing number="6-2" caption="একটি `Message` enum, যার প্রতিটি variant আলাদা পরিমাণ ও type-এর মান রাখে">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-02/src/main.rs:here}}
```

</Listing>

এই enum-এ আলাদা type-এর চারটি variant আছে:

- `Quit`: এর সঙ্গে কোনো ডেটাই যুক্ত নেই
- `Move`: struct-এর মতো নামযুক্ত field আছে
- `Write`: একটিমাত্র `String` আছে
- `ChangeColor`: তিনটি `i32` মান আছে

Listing 6-2-এর মতো variant-সহ একটি enum সংজ্ঞায়িত করা আলাদা আলাদা ধরনের struct সংজ্ঞা
লেখার মতোই, শুধু enum `struct` keyword ব্যবহার করে না এবং সব variant একসঙ্গে `Message`
type-এর অধীনে থাকে। নিচের struct-গুলো আগের enum variant-গুলোর রাখা একই ডেটা রাখতে পারত:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-04-structs-similar-to-message-enum/src/main.rs:here}}
```

কিন্তু আমরা যদি আলাদা আলাদা struct ব্যবহার করতাম, যাদের প্রতিটির নিজস্ব type আছে, তাহলে
এই যেকোনো ধরনের বার্তা নিতে পারে এমন একটি function সংজ্ঞায়িত করা অত সহজ হতো না, যতটা
Listing 6-2-এ সংজ্ঞায়িত `Message` enum-এর ক্ষেত্রে সহজ, কারণ সেটি একটিমাত্র type।

enum আর struct-এর মধ্যে আরও একটি মিল আছে: যেমন `impl` ব্যবহার করে struct-এর ওপর method
(টাইপের সঙ্গে যুক্ত function) সংজ্ঞায়িত করা যায়, তেমনি enum-এর ওপরও method সংজ্ঞায়িত করা যায়।
এখানে `call` নামের একটি method আছে, যা আমরা আমাদের `Message` enum-এর ওপর সংজ্ঞায়িত করতে পারি:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-05-methods-on-enums/src/main.rs:here}}
```

যে মানের ওপর method-টি call করা হয়েছে, তা পেতে method-এর body `self` ব্যবহার করবে। এই
উদাহরণে আমরা `m` নামের একটি variable তৈরি করেছি, যার মান
`Message::Write(String::from("hello"))`, আর `m.call()` চলার সময় `call` method-এর body-তে
`self` হবে সেটিই।

চলুন standard library-র আরেকটি enum দেখি, যা খুবই প্রচলিত ও কাজের: `Option`।

<!-- Old headings. Do not remove or links may break. -->

<a id="the-option-enum-and-its-advantages-over-null-values"></a>

### `Option` enum {#the-option-enum}

এই অংশে `Option`-কে উদাহরণ হিসেবে বিশ্লেষণ করা হয়েছে, যা standard library-তে সংজ্ঞায়িত
আরেকটি enum। `Option` type খুবই প্রচলিত একটি পরিস্থিতি প্রকাশ করে, যেখানে একটি মান কিছু
একটা হতে পারে, আবার কিছুই না-ও হতে পারে।

যেমন, একটি খালি নয় এমন তালিকার প্রথম আইটেম চাইলে আপনি একটি মান পাবেন। একটি খালি তালিকার
প্রথম আইটেম চাইলে কিছুই পাবেন না। এই ধারণাটি type system (ভাষার type-সংক্রান্ত নিয়মের
ব্যবস্থা)-এর ভাষায় প্রকাশ করলে compiler যাচাই করতে পারে যে যেসব ক্ষেত্র সামলানো উচিত,
তার সবগুলো আপনি সামলেছেন কি না; এই সুবিধা এমন বাগ আটকাতে পারে, যা অন্যান্য প্রোগ্রামিং
ভাষায় অত্যন্ত প্রচলিত।

প্রোগ্রামিং ভাষার নকশাকে প্রায়ই দেখা হয় কোন কোন বৈশিষ্ট্য রাখা হলো তার ভিত্তিতে, কিন্তু
কোন বৈশিষ্ট্য বাদ দেওয়া হলো, সেটাও গুরুত্বপূর্ণ। অন্য অনেক ভাষায় থাকা null বৈশিষ্ট্যটি
Rust-এ নেই। _null_ ("কোনো মান নেই" বোঝানো বিশেষ মান) এমন একটি মান, যার মানে সেখানে কোনো মান
নেই। null-যুক্ত ভাষায় variable সবসময় দুটি অবস্থার যেকোনো একটিতে থাকতে পারে: null বা not-null।

null-এর উদ্ভাবক Tony Hoare তার 2009 সালের উপস্থাপনা “Null References: The Billion Dollar
Mistake”-এ বলেছিলেন:

> আমি এটিকে বলি আমার বিলিয়ন ডলারের ভুল। সে সময় আমি একটি object-oriented ভাষায় reference-এর
> জন্য প্রথম পূর্ণাঙ্গ type system নকশা করছিলাম। আমার লক্ষ্য ছিল নিশ্চিত করা যে reference-এর
> সব ব্যবহার সম্পূর্ণ নিরাপদ হবে, আর compiler নিজে থেকেই তা যাচাই করবে। কিন্তু একটি null
> reference রেখে দেওয়ার লোভ আমি সামলাতে পারিনি, শুধু এ কারণে যে এটি implement করা খুব
> সহজ ছিল। এর ফলে অসংখ্য error, নিরাপত্তা-দুর্বলতা আর সিস্টেম ক্র্যাশ হয়েছে, যা সম্ভবত
> গত চল্লিশ বছরে বিলিয়ন ডলারের কষ্ট আর ক্ষতির কারণ হয়েছে।

null মানের সমস্যা হলো, একটি null মানকে not-null মান হিসেবে ব্যবহারের চেষ্টা করলে কোনো না
কোনো ধরনের error পাবেন। এই null বা not-null বৈশিষ্ট্যটি সর্বত্র থাকে বলে এ ধরনের ভুল করা
অত্যন্ত সহজ।

তবে null যে ধারণাটি প্রকাশ করতে চায়, তা এখনো কাজের: null হলো এমন একটি মান, যা কোনো
কারণে বর্তমানে অবৈধ বা অনুপস্থিত।

সমস্যাটি আসলে ধারণায় নয়, বরং নির্দিষ্ট implementation-এ। তাই Rust-এ null নেই, কিন্তু
এমন একটি enum আছে, যা একটি মান উপস্থিত না অনুপস্থিত, সেই ধারণা প্রকাশ করতে পারে। এই
enum হলো `Option<T>`, আর এটি [standard library-তে সংজ্ঞায়িত][option]<!-- ignore -->
এভাবে:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

`Option<T>` enum এতই কাজের যে এটি এমনকি prelude (প্রতিটি প্রোগ্রামে আপনা-আপনি আসা জিনিসের
সেট)-এও রাখা আছে; একে স্পষ্টভাবে scope-এ আনতে হয় না। এর variant-গুলোও prelude-এ আছে:
`Option::` উপসর্গ ছাড়াই আপনি সরাসরি `Some` আর `None` ব্যবহার করতে পারেন। `Option<T>`
enum তবু একটি সাধারণ enum-ই, আর `Some(T)` ও `None` তবু `Option<T>` type-এরই variant।

`<T>` syntax Rust-এর এমন একটি বৈশিষ্ট্য, যা নিয়ে আমরা এখনো কথা বলিনি। এটি একটি generic
type parameter (যেকোনো type বসানো যায় এমন জায়গা-রাখা নাম), আর generics (একাধিক টাইপের
জন্য একই কোড) নিয়ে আমরা অধ্যায় 10-এ আরও বিস্তারিত আলোচনা করব। আপাতত শুধু এটুকু জানলেই
চলবে যে `<T>` মানে `Option` enum-এর `Some` variant যেকোনো type-এর একটি ডেটা রাখতে পারে,
আর `T`-এর জায়গায় ব্যবহৃত প্রতিটি নির্দিষ্ট type পুরো `Option<T>` type-টিকে একটি আলাদা
type বানিয়ে দেয়। সংখ্যার type আর char type রাখতে `Option` মান ব্যবহারের কয়েকটি উদাহরণ:

```aquascope,interpreter
#fn main() {
let some_number = Some(5);
let some_char = Some('e');

let absent_number: Option<i32> = None;`[]`
#}
```

`some_number`-এর type হলো `Option<i32>`। `some_char`-এর type হলো `Option<char>`, যা একটি
আলাদা type। Rust এই type-গুলো infer (type অনুমান করে নেওয়া) করতে পারে, কারণ আমরা `Some`
variant-এর ভেতরে একটি মান দিয়েছি। `absent_number`-এর জন্য Rust চায় আমরা পুরো `Option`
type-টি annotate (type স্পষ্ট করে লিখে দেওয়া) করি: শুধু একটি `None` মান দেখে compiler
বুঝতে পারে না সংশ্লিষ্ট `Some` variant কোন type রাখবে। এখানে আমরা Rust-কে বলছি যে
`absent_number`-কে আমরা `Option<i32>` type-এর হিসেবে চাই।

আমাদের কাছে একটি `Some` মান থাকলে আমরা জানি একটি মান উপস্থিত আছে, আর মানটি `Some`-এর
ভেতরে রাখা আছে। একটি `None` মান থাকলে এক অর্থে এটি null-এর মতোই: আমাদের কাছে কোনো বৈধ মান
নেই। তাহলে null-এর চেয়ে `Option<T>` থাকা ভালো কেন?

সংক্ষেপে, যেহেতু `Option<T>` আর `T` (যেখানে `T` যেকোনো type হতে পারে) আলাদা type, তাই
compiler আমাদের একটি `Option<T>` মানকে নিশ্চিতভাবে বৈধ মানের মতো করে ব্যবহার করতে দেবে না।
যেমন, এই কোড compile হবে না, কারণ এটি একটি `Option<i8>`-এর সঙ্গে একটি `i8` যোগ করার চেষ্টা
করছে:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-07-cant-use-option-directly/src/main.rs:here}}
```

এই কোড চালালে এরকম একটি error message পাই:

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-07-cant-use-option-directly/output.txt}}
```

ভয়ংকর! কার্যত এই error message-এর মানে হলো, একটি `i8` আর একটি `Option<i8>` কীভাবে যোগ
করতে হয়, Rust তা বোঝে না, কারণ এরা আলাদা type। Rust-এ যখন আমাদের কাছে `i8`-এর মতো কোনো
type-এর মান থাকে, তখন compiler নিশ্চিত করবে যে আমাদের কাছে সবসময় একটি বৈধ মান আছে। ওই মান
ব্যবহারের আগে null যাচাই না করেই আমরা আত্মবিশ্বাসের সঙ্গে এগোতে পারি। শুধু যখন আমাদের কাছে
একটি `Option<i8>` (বা আমরা যে type-এর মান নিয়েই কাজ করি না কেন) থাকে, তখনই মান না থাকার
সম্ভাবনা নিয়ে চিন্তা করতে হয়, আর মানটি ব্যবহারের আগে compiler নিশ্চিত করবে যে আমরা সেই
ক্ষেত্রটি সামলেছি।

অন্যভাবে বললে, একটি `Option<T>`-এর ওপর `T`-এর operation চালানোর আগে আপনাকে সেটিকে একটি
`T`-তে রূপান্তর করতে হবে। সাধারণত এটি null-এর সবচেয়ে প্রচলিত সমস্যাগুলোর একটি ধরতে সাহায্য
করে: কোনো কিছু আসলে null হলেও সেটি null নয় বলে ধরে নেওয়া।

কোনো মানকে ভুল করে not-null ধরে নেওয়ার ঝুঁকি দূর করলে আপনি আপনার কোড নিয়ে আরও
আত্মবিশ্বাসী হতে পারেন। null হতে পারে এমন একটি মান রাখতে চাইলে আপনাকে সেই মানের type
`Option<T>` করে স্পষ্টভাবে তা বেছে নিতে হবে। তারপর সেই মান ব্যবহারের সময় মানটি null হলে
সেই ক্ষেত্রটি স্পষ্টভাবে সামলাতে হবে। যেখানেই কোনো মানের type `Option<T>` নয়, সেখানে আপনি
নিরাপদে ধরে নিতে _পারেন_ যে মানটি null নয়। null-এর সর্বব্যাপিতা সীমিত করতে এবং Rust কোডের
নিরাপত্তা বাড়াতে এটি ছিল Rust-এর একটি সচেতন নকশাগত সিদ্ধান্ত।

তাহলে `Option<T>` type-এর একটি মান থাকলে `Some` variant থেকে `T` মানটি কীভাবে বের করে
আনবেন, যাতে সেই মান ব্যবহার করতে পারেন? `Option<T>` enum-এর অনেকগুলো method আছে, যেগুলো
নানা পরিস্থিতিতে কাজে লাগে; সেগুলো দেখে নিতে পারেন [এর ডকুমেন্টেশনে][docs]<!-- ignore -->।
`Option<T>`-এর method-গুলোর সঙ্গে পরিচিত হওয়া Rust নিয়ে আপনার যাত্রায় অত্যন্ত কাজে আসবে।

সাধারণভাবে, একটি `Option<T>` মান ব্যবহার করতে আপনি এমন কোড চাইবেন, যা প্রতিটি variant
সামলাবে। কিছু কোড চাইবেন, যা শুধু তখনই চলবে, যখন আপনার কাছে একটি `Some(T)` মান থাকবে, আর
এই কোড ভেতরের `T` ব্যবহার করতে পারবে। অন্য কিছু কোড চাইবেন, যা শুধু তখনই চলবে, যখন আপনার
কাছে একটি `None` মান থাকবে, আর সেই কোডের হাতে কোনো `T` মান থাকবে না। `match` expression
হলো এমন একটি control flow (কোড চলার ক্রম নিয়ন্ত্রণ) গঠন, যা enum-এর সঙ্গে ব্যবহার করলে
ঠিক এই কাজটিই করে: enum-এর কোন variant আছে তার ওপর নির্ভর করে এটি আলাদা কোড চালায়, আর
সেই কোড মিলে যাওয়া মানের ভেতরের ডেটা ব্যবহার করতে পারে।

{{#quiz ../quizzes/ch06-01-defining-an-enum.toml}}

[IpAddr]: https://doc.rust-lang.org/std/net/enum.IpAddr.html
[option]: https://doc.rust-lang.org/std/option/enum.Option.html
[docs]: https://doc.rust-lang.org/std/option/enum.Option.html
