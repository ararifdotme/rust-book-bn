## string দিয়ে UTF-8 এনকোড করা লেখা রাখা {#storing-utf-8-encoded-text-with-strings}

অধ্যায় 4-এ আমরা string নিয়ে কথা বলেছি, তবে এবার আরও গভীরে দেখব। নতুন Rustacean-রা সাধারণত তিনটি কারণের
সমন্বয়ে string-এ আটকে যান: সম্ভাব্য error প্রকাশ করার দিকে Rust-এর ঝোঁক, অনেক প্রোগ্রামার যতটা ভাবেন তার চেয়ে
string বেশি জটিল একটি data structure হওয়া, আর UTF-8। অন্য প্রোগ্রামিং ভাষা থেকে এলে এই বিষয়গুলো মিলে কঠিন
মনে হতে পারে।

আমরা string নিয়ে collection-এর প্রেক্ষাপটে আলোচনা করি, কারণ string implement করা হয়েছে byte-এর একটি collection
হিসেবে, সঙ্গে কিছু method, যা সেই byte-গুলোকে লেখা হিসেবে ব্যাখ্যা করলে কাজের সুবিধা দেয়। এই অংশে আমরা `String`-এর
সেই operation-গুলো নিয়ে কথা বলব, যা প্রতিটি collection type-এর থাকে, যেমন তৈরি করা, আপডেট করা আর পড়া। `String`
অন্য collection থেকে কোথায় আলাদা, তা-ও আলোচনা করব, অর্থাৎ মানুষ আর কম্পিউটার `String`-এর ডেটা যেভাবে ব্যাখ্যা
করে, তার পার্থক্যের কারণে `String`-এ index দিয়ে পৌঁছানো কেন জটিল।

<!-- Old headings. Do not remove or links may break. -->

<a id="what-is-a-string"></a>

### string সংজ্ঞায়িত করা {#defining-strings}

প্রথমে আমরা _string_ শব্দ দিয়ে কী বোঝাই, তা সংজ্ঞায়িত করব। Rust-এর মূল ভাষায় string type আছে মাত্র একটি, আর তা
হলো string slice `str`, যা সাধারণত দেখা যায় এর borrow করা রূপে, `&str`। অধ্যায় 4-এ আমরা string slice নিয়ে কথা
বলেছি, যা অন্য কোথাও রাখা UTF-8 এনকোড করা string ডেটার reference। যেমন, string literal (উদ্ধৃতিচিহ্নের ভেতরে
সরাসরি লেখা string) রাখা থাকে প্রোগ্রামের binary-তে, আর তাই এরা string slice।

`String` type, যা মূল ভাষায় কোড করা নয়, বরং Rust-এর standard library-র দেওয়া, হলো একটি বাড়তে পারে এমন, mutable,
owned, UTF-8 এনকোড করা string type। Rustacean-রা Rust-এ “string” বললে `String` বা string slice `&str` যেকোনো type-ই
বোঝাতে পারেন, শুধু একটিকে নয়। এই অংশ মূলত `String` নিয়ে হলেও Rust-এর standard library-তে দুটি type-ই প্রচুর
ব্যবহার হয়, আর `String` ও string slice দুটোই UTF-8 এনকোড করা।

### একটি নতুন string তৈরি করা {#creating-a-new-string}

`Vec<T>`-এর অনেক operation `String`-এও পাওয়া যায়, কারণ `String` আসলে implement করা হয়েছে byte-এর একটি vector-এর
চারপাশে একটি wrapper (অন্য একটি type-কে মুড়ে রাখা type) হিসেবে, সঙ্গে কিছু বাড়তি নিশ্চয়তা, সীমাবদ্ধতা আর
সক্ষমতা। `Vec<T>` আর `String`-এর সঙ্গে একইভাবে কাজ করে এমন function-এর একটি উদাহরণ হলো instance তৈরির `new`
function, যা Listing 8-11-এ দেখানো হয়েছে।

<Listing number="8-11" caption="একটি নতুন, ফাঁকা `String` তৈরি করা">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-11/src/main.rs:here}}
```

</Listing>

এই লাইনটি `s` নামে একটি নতুন, ফাঁকা string তৈরি করে, যাতে আমরা পরে ডেটা রাখতে পারি। প্রায়ই আমাদের কাছে কিছু
শুরুর ডেটা থাকে, যা দিয়ে string শুরু করতে চাই। এর জন্য আমরা `to_string` method ব্যবহার করি, যা `Display` trait
implement করে এমন যেকোনো type-এ পাওয়া যায়, যেমন string literal। Listing 8-12-এ দুটি উদাহরণ দেখানো হয়েছে।

<Listing number="8-12" caption="একটি string literal থেকে একটি `String` তৈরি করতে `to_string` method ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-12/src/main.rs:here}}
```

</Listing>

এই কোড `initial contents` থাকা একটি string তৈরি করে।

একটি string literal থেকে `String` তৈরি করতে আমরা `String::from` function-ও ব্যবহার করতে পারি। Listing 8-13-এর
কোডটি Listing 8-12-এর `to_string` ব্যবহার করা কোডের সমতুল্য।

<Listing number="8-13" caption="একটি string literal থেকে একটি `String` তৈরি করতে `String::from` function ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-13/src/main.rs:here}}
```

</Listing>

string যেহেতু অনেক কাজে ব্যবহার হয়, তাই string-এর জন্য আমরা অনেক আলাদা generic API ব্যবহার করতে পারি, যা আমাদের
অনেক বিকল্প দেয়। এর কিছু বাহুল্য মনে হতে পারে, কিন্তু প্রতিটিরই নিজস্ব জায়গা আছে! এই ক্ষেত্রে `String::from` আর
`to_string` একই কাজ করে, তাই কোনটি বেছে নেবেন, তা লেখার ধরন আর পাঠযোগ্যতার ব্যাপার।

মনে রাখবেন, string UTF-8 এনকোড করা, তাই সঠিকভাবে এনকোড করা যেকোনো ডেটা এতে রাখা যায়, Listing 8-14-এ যেমন
দেখানো হয়েছে।

<Listing number="8-14" caption="বিভিন্ন ভাষার শুভেচ্ছা-বার্তা string-এ রাখা">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:here}}
```

</Listing>

এগুলোর সবই বৈধ `String` মান।

### একটি string আপডেট করা {#updating-a-string}

`Vec<T>`-এর বিষয়বস্তুর মতোই একটি `String`-এ আরও ডেটা push করলে এর আকার বাড়তে পারে এবং এর বিষয়বস্তু বদলাতে
পারে। এছাড়া `String` মানগুলো জোড়া লাগাতে আপনি সুবিধাজনকভাবে `+` operator বা `format!` macro ব্যবহার করতে পারেন।

<!-- Old headings. Do not remove or links may break. -->

<a id="appending-to-a-string-with-push_str-and-push"></a>

#### `push_str` বা `push` দিয়ে শেষে জুড়ে দেওয়া {#appending-with-push_str-or-push}

একটি string slice শেষে জুড়ে দিতে `push_str` method ব্যবহার করে আমরা একটি `String` বড় করতে পারি, Listing 8-15-এ
যেমন দেখানো হয়েছে।

<Listing number="8-15" caption="`push_str` method ব্যবহার করে একটি `String`-এর শেষে একটি string slice জুড়ে দেওয়া">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-15/src/main.rs:here}}
```

</Listing>

এই দুটি লাইনের পরে `s`-এ থাকবে `foobar`। `push_str` method একটি string slice নেয়, কারণ আমরা parameter-টির
ownership নিতে চাই, এমন নয়। যেমন, Listing 8-16-এর কোডে আমরা চাই `s1`-এর শেষে `s2`-এর বিষয়বস্তু জুড়ে দেওয়ার পরও
এটি ব্যবহার করতে পারতে।

<Listing number="8-16" caption="একটি string slice-এর বিষয়বস্তু একটি `String`-এর শেষে জুড়ে দেওয়ার পরও সেটি ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-16/src/main.rs:here}}
```

</Listing>

`push_str` method যদি `s2`-এর ownership নিত, তাহলে শেষ লাইনে আমরা এর মান প্রিন্ট করতে পারতাম না। তবে এই কোড
আমাদের প্রত্যাশামতোই কাজ করে!

`push` method parameter হিসেবে একটিমাত্র অক্ষর নেয় এবং সেটি `String`-এ যোগ করে। Listing 8-17 `push` method
ব্যবহার করে একটি `String`-এ _l_ অক্ষরটি যোগ করে।

<Listing number="8-17" caption="`push` ব্যবহার করে একটি `String` মানে একটি অক্ষর যোগ করা">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-17/src/main.rs:here}}
```

</Listing>

ফলে `s`-এ থাকবে `lol`।

<!-- Old headings. Do not remove or links may break. -->

<a id="concatenation-with-the--operator-or-the-format-macro"></a>

#### `+` বা `format!` দিয়ে জোড়া লাগানো {#concatenating-with--or-format}

প্রায়ই আপনি বিদ্যমান দুটি string একত্র করতে চাইবেন। এটি করার একটি উপায় হলো `+` operator ব্যবহার করা, Listing
8-18-এ যেমন দেখানো হয়েছে।

<Listing number="8-18" caption="দুটি `String` মান একত্র করে একটি নতুন `String` মান তৈরি করতে `+` operator ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-18/src/main.rs:here}}
```

</Listing>

`s3` string-এ থাকবে `Hello, world!`। যোগের পরে `s1` কেন আর বৈধ নয়, আর `s2`-এর জন্য আমরা কেন একটি reference ব্যবহার
করেছি, তার কারণ `+` operator ব্যবহার করলে যে method call হয়, তার signature-এর সঙ্গে জড়িত। `+` operator `add` method
ব্যবহার করে, যার signature দেখতে মোটামুটি এরকম:

```rust,ignore
fn add(self, s: &str) -> String {
```

standard library-তে দেখবেন `add` সংজ্ঞায়িত হয়েছে generics আর associated type (trait-এর সঙ্গে যুক্ত type) ব্যবহার
করে। এখানে আমরা নির্দিষ্ট type বসিয়ে দিয়েছি, যা ঘটে যখন আমরা `String` মান দিয়ে এই method call করি। generics নিয়ে
আমরা অধ্যায় 10-এ আলোচনা করব। এই signature আমাদের সেই সূত্রগুলো দেয়, যা `+` operator-এর জটিল দিকগুলো বুঝতে দরকার।

প্রথমত, `s2`-এর সঙ্গে একটি `&` আছে, অর্থাৎ আমরা প্রথম string-এর সঙ্গে দ্বিতীয় string-এর একটি reference যোগ করছি।
এর কারণ `add` function-এর `s` parameter: আমরা একটি `String`-এর সঙ্গে শুধু একটি string slice যোগ করতে পারি; দুটি
`String` মান একসঙ্গে যোগ করতে পারি না। কিন্তু দাঁড়ান—`&s2`-এর type হলো `&String`, `&str` নয়, যা `add`-এর দ্বিতীয়
parameter-এ নির্দিষ্ট করা আছে। তাহলে Listing 8-18 compile হয় কেন?

`add` call-এ আমরা `&s2` ব্যবহার করতে পারার কারণ হলো, compiler `&String` argument-টিকে একটি `&str`-এ coerce
(এক type-কে স্বয়ংক্রিয়ভাবে আরেক type-এ রূপান্তর) করতে পারে। আমরা যখন `add` method call করি, Rust একটি deref
coercion (স্বয়ংক্রিয় dereference-রূপান্তর) ব্যবহার করে, যা এখানে `&s2`-কে `&s2[..]`-এ পরিণত করে। deref coercion
নিয়ে আমরা অধ্যায় 15-এ আরও গভীরভাবে আলোচনা করব। `add` যেহেতু `s` parameter-এর ownership নেয় না, তাই এই
operation-এর পরও `s2` একটি বৈধ `String` থাকবে।

<!-- BEGIN INTERVENTION: f1ab2171-96f0-4380-b16d-9055a9a00415 -->
দ্বিতীয়ত, signature-এ আমরা দেখতে পাই যে `add` `self`-এর ownership নেয়,
কারণ `self`-এর সঙ্গে কোনো `&` *নেই*। এর মানে Listing 8-18-এর `s1` `add` call-এ
move হয়ে যাবে এবং তারপর আর বৈধ থাকবে না। তাই `let s3 = s1 + &s2;` দেখে মনে হলেও যে এটি
দুটি string কপি করে একটি নতুন string তৈরি করবে, এই statement আসলে এরকম করে:
1. `add` `s1`-এর ownership নেয়,
2. এটি `s2`-এর বিষয়বস্তুর একটি কপি `s1`-এর শেষে জুড়ে দেয়, 
3. তারপর এটি `s1`-এর ownership ফেরত দেয়।

`s1`-এর যদি `s2`-এর জন্য যথেষ্ট ধারণক্ষমতা থাকে, তাহলে কোনো মেমোরি বরাদ্দ ঘটে না। তবে `s1`-এর যদি `s2`-এর জন্য যথেষ্ট ধারণক্ষমতা না থাকে, তাহলে দুটি string ধরাতে `s1` ভেতরে ভেতরে একটি বড় মেমোরি বরাদ্দ করবে।
<!-- END INTERVENTION -->

একাধিক string জোড়া লাগাতে হলে `+` operator-এর আচরণ অগোছালো হয়ে
যায়:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-01-concat-multiple-strings/src/main.rs:here}}
```

এই পর্যায়ে `s` হবে `tic-tac-toe`। এত `+` আর `"` অক্ষরের ভিড়ে কী ঘটছে তা বোঝা কঠিন। আরও জটিলভাবে string একত্র
করতে আমরা এর বদলে `format!` macro ব্যবহার করতে পারি:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-02-format/src/main.rs:here}}
```

এই কোডও `s`-এ `tic-tac-toe` সেট করে। `format!` macro `println!`-এর মতো কাজ করে, কিন্তু আউটপুট স্ক্রিনে প্রিন্ট
করার বদলে বিষয়বস্তুসহ একটি `String` return করে। `format!` ব্যবহার করা কোডের সংস্করণটি পড়া অনেক সহজ, আর
`format!` macro-র তৈরি কোড reference ব্যবহার করে, তাই এই call এর কোনো parameter-এর ownership নেয় না।

{{#quiz ../quizzes/ch08-02-string-sec1.toml}}

### string-এ index দিয়ে পৌঁছানো {#indexing-into-strings}

অন্য অনেক প্রোগ্রামিং ভাষায় index দিয়ে উল্লেখ করে string-এর আলাদা আলাদা অক্ষরে পৌঁছানো একটি বৈধ আর প্রচলিত
operation। কিন্তু Rust-এ index syntax ব্যবহার করে একটি `String`-এর অংশে পৌঁছানোর চেষ্টা করলে error পাবেন। Listing
8-19-এর অবৈধ কোডটি দেখুন।

<Listing number="8-19" caption="একটি `String`-এর সঙ্গে index syntax ব্যবহারের চেষ্টা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-19/src/main.rs:here}}
```

</Listing>

এই কোড নিচের error ঘটাবে:

```console
{{#include ../listings/ch08-common-collections/listing-08-19/output.txt}}
```

error-টিই পুরো গল্প বলে: Rust-এর string index সমর্থন করে না। কিন্তু কেন নয়? এই প্রশ্নের উত্তর দিতে আমাদের
আলোচনা করতে হবে Rust মেমোরিতে string কীভাবে রাখে।

#### ভেতরের প্রকাশরূপ {#internal-representation}

একটি `String` হলো একটি `Vec<u8>`-এর ওপর একটি wrapper। চলুন Listing 8-14-এর সঠিকভাবে এনকোড করা UTF-8 উদাহরণ
string-গুলোর কয়েকটি দেখি। প্রথমে এটি:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:spanish}}
```

এই ক্ষেত্রে `len` হবে `4`, অর্থাৎ `"Hola"` string রাখা vector-টি 4 byte লম্বা। UTF-8-এ এনকোড করলে এই অক্ষরগুলোর
প্রতিটি 1 byte নেয়। তবে নিচের লাইনটি আপনাকে অবাক করতে পারে (লক্ষ করুন, এই string শুরু হয়েছে বড় হাতের সিরিলিক
অক্ষর _Ze_ দিয়ে, সংখ্যা 3 দিয়ে নয়):

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:russian}}
```

string-টি কত লম্বা জিজ্ঞেস করা হলে আপনি হয়তো বলবেন 12। আসলে Rust-এর উত্তর 24: “Здравствуйте” UTF-8-এ এনকোড
করতে এতগুলো byte লাগে, কারণ ওই string-এর প্রতিটি Unicode scalar value (Unicode-এর একটি একক অক্ষর-মান) রাখতে 2 byte
জায়গা লাগে। তাই string-এর byte-গুলোর একটি index সবসময় একটি বৈধ Unicode scalar value-র সঙ্গে মিলবে না। দেখাতে
এই অবৈধ Rust কোডটি ভাবুন:

```rust,ignore,does_not_compile
let hello = "Здравствуйте";
let answer = &hello[0];
```

আপনি আগেই জানেন যে `answer` প্রথম অক্ষর `З` হবে না। UTF-8-এ এনকোড করলে `З`-এর প্রথম byte হলো `208` আর দ্বিতীয়টি
`151`, তাই মনে হতে পারে `answer` আসলে `208` হওয়া উচিত, কিন্তু `208` নিজে একা কোনো বৈধ অক্ষর নয়। কোনো ব্যবহারকারী
এই string-এর প্রথম অক্ষর চাইলে `208` return করা সম্ভবত তিনি চাইতেন না; তবে byte index 0-তে Rust-এর কাছে শুধু এই
ডেটাই আছে। string-এ শুধু লাতিন অক্ষর থাকলেও ব্যবহারকারীরা সাধারণত byte মান return হওয়া চান না: `&"hi"[0]` যদি
byte মান return করা বৈধ কোড হতো, তাহলে এটি `h` নয়, `104` return করত।

তাহলে উত্তর হলো, অপ্রত্যাশিত মান return করে এমন বাগ ঘটানো এড়াতে, যা হয়তো সঙ্গে সঙ্গে ধরা পড়ত না, Rust এই কোড
compile-ই করে না, আর ডেভেলপমেন্ট প্রক্রিয়ার শুরুতেই ভুল বোঝাবুঝি আটকে দেয়।

<!-- Old headings. Do not remove or links may break. -->

<a id="bytes-and-scalar-values-and-grapheme-clusters-oh-my"></a>

#### byte, scalar value আর grapheme cluster {#bytes-scalar-values-and-grapheme-clusters}

UTF-8 নিয়ে আরেকটি কথা হলো, Rust-এর দৃষ্টিকোণ থেকে string দেখার আসলে তিনটি প্রাসঙ্গিক উপায় আছে: byte হিসেবে,
scalar value হিসেবে, আর grapheme cluster (মানুষের চোখে একটি অক্ষর, যা একাধিক Unicode মান মিলে গঠিত) হিসেবে, যা আমরা
যাকে _বর্ণ_ বলি, তার সবচেয়ে কাছাকাছি।

দেবনাগরী লিপিতে লেখা হিন্দি শব্দ “नमस्ते” দেখলে, এটি রাখা হয় `u8` মানের একটি vector হিসেবে, যা দেখতে
এরকম:

```text
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```

এটি 18 byte, আর কম্পিউটার শেষ পর্যন্ত এভাবেই এই ডেটা রাখে। এগুলোকে Unicode scalar value হিসেবে দেখলে, যা Rust-এর
`char` type, সেই byte-গুলো দেখতে এরকম:

```text
['न', 'म', 'स', '्', 'त', 'े']
```

এখানে ছয়টি `char` মান আছে, কিন্তু চতুর্থ আর ষষ্ঠটি বর্ণ নয়: এগুলো এমন চিহ্ন (diacritic), যা একা অর্থবহ নয়। সবশেষে,
এগুলোকে grapheme cluster হিসেবে দেখলে আমরা পাব সেই চারটি বর্ণ, যা দিয়ে একজন মানুষ হিন্দি শব্দটি গঠিত বলবেন:

```text
["न", "म", "स्", "ते"]
```

Rust কম্পিউটারের রাখা কাঁচা string ডেটা ব্যাখ্যা করার বিভিন্ন উপায় দেয়, যাতে প্রতিটি প্রোগ্রাম তার দরকারি ব্যাখ্যাটি
বেছে নিতে পারে, ডেটা যে মানব-ভাষাতেই থাকুক না কেন।

একটি অক্ষর পেতে Rust আমাদের একটি `String`-এ index ব্যবহার করতে না দেওয়ার শেষ কারণ হলো, index operation সবসময়
constant time (ইনপুটের আকার যা-ই হোক, একই সময়) (O(1)) নেবে বলে আশা করা হয়। কিন্তু `String`-এর ক্ষেত্রে সেই
পারফরম্যান্সের নিশ্চয়তা দেওয়া সম্ভব নয়, কারণ কয়টি বৈধ অক্ষর আছে তা নির্ধারণ করতে Rust-কে শুরু থেকে index পর্যন্ত
বিষয়বস্তু হেঁটে যেতে হতো।

### string-এর slice নেওয়া {#slicing-strings}

string-এ index দিয়ে পৌঁছানো প্রায়ই খারাপ ধারণা, কারণ string-index operation-এর return type কী হওয়া উচিত, তা
স্পষ্ট নয়: একটি byte মান, একটি অক্ষর, একটি grapheme cluster, নাকি একটি string slice। তাই string slice তৈরি করতে
সত্যিই যদি index ব্যবহার করতে হয়, Rust আপনাকে আরও নির্দিষ্ট হতে বলে।

একটিমাত্র সংখ্যাসহ `[]` দিয়ে index করার বদলে আপনি একটি range-সহ `[]` ব্যবহার করে নির্দিষ্ট কিছু byte থাকা একটি
string slice তৈরি করতে পারেন:

```rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```

এখানে `s` হবে একটি `&str`, যাতে string-এর প্রথম 4 byte থাকবে। আগে আমরা বলেছিলাম, এই অক্ষরগুলোর প্রতিটি 2 byte,
অর্থাৎ `s` হবে `Зд`।

`&hello[0..1]`-এর মতো কিছু দিয়ে যদি একটি অক্ষরের byte-গুলোর শুধু একটি অংশের slice নেওয়ার চেষ্টা করতাম, তাহলে
vector-এ অবৈধ index-এ পৌঁছানোর মতোই Rust runtime-এ panic করত:

```console
{{#include ../listings/ch08-common-collections/output-only-01-not-char-boundary/output.txt}}
```

range দিয়ে string slice তৈরি করার সময় সতর্ক থাকা উচিত, কারণ এতে আপনার প্রোগ্রাম ক্র্যাশ করতে পারে।

<!-- Old headings. Do not remove or links may break. -->

<a id="methods-for-iterating-over-strings"></a>

### string-এর ওপর iterate করা {#iterating-over-strings}

string-এর অংশ নিয়ে কাজ করার সবচেয়ে ভালো উপায় হলো স্পষ্ট করে বলা যে আপনি অক্ষর চান নাকি byte। আলাদা আলাদা
Unicode scalar value-র জন্য `chars` method ব্যবহার করুন। “Зд”-এর ওপর `chars` call করলে এটি `char` type-এর দুটি মান
আলাদা করে return করে, আর প্রতিটি উপাদানে পৌঁছাতে আপনি ফলাফলের ওপর iterate করতে পারেন:

```rust
for c in "Зд".chars() {
    println!("{c}");
}
```

এই কোড নিচের আউটপুট প্রিন্ট করবে:

```text
З
д
```

বিকল্প হিসেবে, `bytes` method প্রতিটি কাঁচা byte return করে, যা আপনার কাজের ক্ষেত্রে উপযুক্ত হতে পারে:

```rust
for b in "Зд".bytes() {
    println!("{b}");
}
```

এই কোড এই string গঠনকারী 4টি byte প্রিন্ট করবে:

```text
208
151
208
180
```

তবে অবশ্যই মনে রাখবেন, বৈধ Unicode scalar value 1 byte-এর বেশি দিয়ে গঠিত হতে পারে।

দেবনাগরী লিপির মতো ক্ষেত্রে string থেকে grapheme cluster বের করা জটিল, তাই এই কার্যকারিতা standard library দেয়
না। এই কার্যকারিতা দরকার হলে [crates.io](https://crates.io/)<!-- ignore -->-তে crate পাওয়া যায়।

<!-- Old headings. Do not remove or links may break. -->

<a id="strings-are-not-so-simple"></a>

### string-এর জটিলতা সামলানো {#handling-the-complexities-of-strings}

সংক্ষেপে, string জটিল। বিভিন্ন প্রোগ্রামিং ভাষা প্রোগ্রামারের কাছে এই জটিলতা কীভাবে উপস্থাপন করবে, তা নিয়ে ভিন্ন
ভিন্ন সিদ্ধান্ত নেয়। Rust সব Rust প্রোগ্রামের ডিফল্ট আচরণ হিসেবে `String` ডেটা সঠিকভাবে সামলানোর পথ বেছে নিয়েছে,
অর্থাৎ প্রোগ্রামারদের শুরুতেই UTF-8 ডেটা সামলানো নিয়ে বেশি ভাবতে হয়। এই আপসের ফলে অন্য প্রোগ্রামিং ভাষায় যতটা
দেখা যায়, তার চেয়ে string-এর জটিলতা বেশি প্রকাশ পায়, কিন্তু এতে আপনার ডেভেলপমেন্টের পরের ধাপে ASCII নয় এমন অক্ষর
জড়িত error সামলানোর ঝামেলা থেকে বাঁচা যায়।

ভালো খবর হলো, এই জটিল পরিস্থিতিগুলো সঠিকভাবে সামলাতে standard library `String` আর `&str` type-এর ওপর ভিত্তি করে
অনেক কার্যকারিতা দেয়। string-এ খোঁজার জন্য `contains` আর string-এর অংশ অন্য string দিয়ে বদলানোর জন্য `replace`-এর
মতো কাজের method-গুলোর জন্য অবশ্যই ডকুমেন্টেশন দেখে নিন।

চলুন একটু কম জটিল কিছুতে যাই: hash map!

{{#quiz ../quizzes/ch08-02-string-sec2.toml}}