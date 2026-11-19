## object-oriented ভাষার বৈশিষ্ট্য {#characteristics-of-object-oriented-languages}

একটি ভাষাকে object-oriented বলে গণ্য করতে হলে তাতে কী কী বৈশিষ্ট্য থাকতে হবে, তা নিয়ে প্রোগ্রামিং community-তে কোনো ঐকমত্য নেই। Rust OOP-সহ অনেক প্রোগ্রামিং paradigm (ধারা) দ্বারা
প্রভাবিত; যেমন, functional programming থেকে আসা বৈশিষ্ট্যগুলো আমরা অধ্যায় 13-এ দেখেছি। বলা যায়, OOP ভাষাগুলো কিছু সাধারণ বৈশিষ্ট্য ভাগ করে—যেমন object, encapsulation আর inheritance।
চলুন দেখি এই বৈশিষ্ট্যগুলোর প্রতিটির মানে কী এবং Rust তা সমর্থন করে কি না।

### object ডেটা আর আচরণ ধারণ করে {#objects-contain-data-and-behavior}

Erich Gamma, Richard Helm, Ralph Johnson আর John Vlissides-এর লেখা বই _Design Patterns: Elements of Reusable Object-Oriented Software_ (Addison-Wesley,
1994), যাকে চলতি ভাষায় _The Gang of Four_ বই বলা হয়, object-oriented design pattern-এর একটি তালিকা। এটি OOP-কে এভাবে সংজ্ঞায়িত করে:

> object-oriented প্রোগ্রাম object দিয়ে তৈরি। একটি **object** ডেটা আর সেই ডেটার ওপর কাজ করা procedure, দুটিকেই একসঙ্গে মোড়ে। procedure-গুলোকে সাধারণত **methods** (method) বা
> **operations** (operation) বলা হয়।

এই সংজ্ঞা অনুযায়ী Rust object-oriented: struct আর enum-এ ডেটা থাকে, আর `impl` block struct ও enum-এর ওপর method দেয়। method-যুক্ত struct আর enum-কে object _বলা_ না হলেও Gang of Four-এর
object-এর সংজ্ঞা অনুযায়ী এরা একই কার্যক্ষমতা দেয়।

### implementation-এর খুঁটিনাটি লুকিয়ে রাখা encapsulation {#encapsulation-that-hides-implementation-details}

OOP-এর সঙ্গে সাধারণত যুক্ত আরেকটি দিক হলো _encapsulation_ (আবদ্ধকরণ)-এর ধারণা, যার মানে একটি object-এর implementation-এর খুঁটিনাটি সেই object ব্যবহার করা কোডের নাগালের বাইরে। তাই একটি
object-এর সঙ্গে কাজ করার একমাত্র উপায় হলো এর public API; object ব্যবহার করা কোড object-এর ভেতরে হাত দিয়ে সরাসরি ডেটা বা আচরণ বদলাতে পারা উচিত নয়। এতে প্রোগ্রামার object ব্যবহার করা কোড না
বদলেই object-এর ভেতরটা বদলাতে ও refactor করতে পারেন।

অধ্যায় 7-এ আমরা আলোচনা করেছি কীভাবে encapsulation নিয়ন্ত্রণ করতে হয়: আমাদের কোডের কোন module, type, function আর method public হবে তা ঠিক করতে `pub` keyword ব্যবহার করা যায়, আর default-ভাবে
বাকি সবকিছু private। যেমন, আমরা `AveragedCollection` নামের একটি struct সংজ্ঞায়িত করতে পারি, যার একটি field-এ `i32` মানের একটি vector থাকে। struct-টিতে vector-এর মানগুলোর গড় রাখা একটি field-ও
থাকতে পারে, অর্থাৎ কারও দরকার হলেই প্রতিবার গড় হিসাব করতে হয় না। অন্যভাবে বললে, `AveragedCollection` আমাদের জন্য হিসাব করা গড়টি cache করে (জমিয়ে রাখে)। Listing 18-1-এ `AveragedCollection`
struct-এর সংজ্ঞা আছে।

<Listing number="18-1" file-name="src/lib.rs" caption="integer-এর একটি তালিকা আর collection-এর item-গুলোর গড় রাখা একটি `AveragedCollection` struct">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-01/src/lib.rs}}
```

</Listing>

struct-টি `pub` হিসেবে চিহ্নিত, যাতে অন্য কোড এটি ব্যবহার করতে পারে, কিন্তু struct-এর ভেতরের field-গুলো private থাকে। এই ক্ষেত্রে এটি গুরুত্বপূর্ণ, কারণ আমরা নিশ্চিত করতে চাই যে তালিকায় কোনো মান যোগ
বা বাদ দিলে গড়ও হালনাগাদ হয়। এটা করি struct-এর ওপর `add`, `remove` আর `average` method implement করে, যেমনটি Listing 18-2-এ দেখানো হয়েছে।

<Listing number="18-2" file-name="src/lib.rs" caption="`AveragedCollection`-এর ওপর public method `add`, `remove` আর `average`-এর implementation">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-02/src/lib.rs:here}}
```

</Listing>

`AveragedCollection`-এর একটি instance-এর ডেটা পেতে বা পরিবর্তন করতে public method `add`, `remove` আর `average`-ই একমাত্র উপায়। `add` method দিয়ে `list`-এ একটি item যোগ করলে বা `remove`
method দিয়ে বাদ দিলে প্রতিটির implementation private `update_average` method call করে, যা `average` field হালনাগাদ করার কাজটিও সামলায়।

`list` আর `average` field-গুলো আমরা private রাখি, যাতে বাইরের কোড সরাসরি `list` field-এ item যোগ বা বাদ দিতে না পারে; নইলে `list` বদলালে `average` field অসংগত হয়ে যেতে পারত। `average` method
`average` field-এর মান return করে, ফলে বাইরের কোড `average` পড়তে পারে কিন্তু পরিবর্তন করতে পারে না।

`AveragedCollection` struct-এর implementation-এর খুঁটিনাটি encapsulate করেছি বলে ভবিষ্যতে data structure-এর মতো দিকগুলো সহজেই বদলাতে পারি। যেমন, `list` field-এর জন্য একটি `Vec<i32>`-এর
বদলে একটি `HashSet<i32>` ব্যবহার করতে পারতাম। `add`, `remove` আর `average` public method-গুলোর signature একই থাকলে `AveragedCollection` ব্যবহার করা কোড বদলাতে হতো না। এর বদলে `list` public করলে
তা নাও হতে পারত: `HashSet<i32>` আর `Vec<i32>`-এ item যোগ ও বাদ দেওয়ার method আলাদা, তাই বাইরের কোড সরাসরি `list` পরিবর্তন করলে সম্ভবত তাকে বদলাতে হতো।

একটি ভাষাকে object-oriented বলে গণ্য করতে encapsulation যদি বাধ্যতামূলক দিক হয়, তাহলে Rust সেই শর্ত পূরণ করে। কোডের বিভিন্ন অংশে `pub` ব্যবহার করা বা না করার সুযোগ implementation-এর
খুঁটিনাটি encapsulate করা সম্ভব করে।

### type system ও কোড ভাগ করার উপায় হিসেবে inheritance {#inheritance-as-a-type-system-and-as-code-sharing}

_Inheritance_ (উত্তরাধিকার) এমন একটি ব্যবস্থা, যার মাধ্যমে একটি object আরেকটি object-এর সংজ্ঞা থেকে উপাদান উত্তরাধিকার সূত্রে পেতে পারে, ফলে আপনাকে আবার সংজ্ঞায়িত না করেই এটি parent
object-এর ডেটা আর আচরণ পায়।

object-oriented হতে হলে যদি একটি ভাষায় inheritance থাকতেই হয়, তাহলে Rust তেমন ভাষা নয়। macro ব্যবহার না করে এমন struct সংজ্ঞায়িত করার কোনো উপায় নেই, যা parent struct-এর field আর method-এর
implementation উত্তরাধিকার সূত্রে পায়।

তবে আপনার প্রোগ্রামিং হাতিয়ারে inheritance রাখতে অভ্যস্ত হলে, শুরুতে কেন inheritance চাইছিলেন তার ওপর নির্ভর করে Rust-এ অন্য সমাধান ব্যবহার করতে পারেন।

inheritance বেছে নেওয়ার দুটি প্রধান কারণ থাকে। একটি হলো কোডের পুনর্ব্যবহার: আপনি এক type-এর জন্য নির্দিষ্ট আচরণ implement করতে পারেন, আর inheritance আপনাকে সেই implementation আরেকটি type-এর
জন্য পুনর্ব্যবহার করতে দেয়। Rust কোডে trait method-এর default implementation দিয়ে সীমিতভাবে এটা করা যায়, যা আপনি Listing 10-14-এ দেখেছেন, যখন `Summary` trait-এ `summarize` method-এর
একটি default implementation যোগ করেছিলাম। `Summary` trait implement করা যেকোনো type-এ আর কোনো কোড ছাড়াই `summarize` method পাওয়া যায়। এটি অনেকটা একটি parent class-এ একটি method-এর
implementation থাকা এবং উত্তরাধিকারী child class-এও সেই method-এর implementation থাকার মতো। `Summary` trait implement করার সময় `summarize` method-এর default implementation override-ও করতে
পারি, যা parent class থেকে উত্তরাধিকার সূত্রে পাওয়া একটি method-এর implementation child class-এর override করার মতো।

inheritance ব্যবহারের অন্য কারণটি type system-এর সঙ্গে সম্পর্কিত: parent type যেসব জায়গায় ব্যবহার হয়, সেখানে child type ব্যবহার করতে দেওয়া। একে _polymorphism_ (বহুরূপতা)-ও বলা হয়, যার মানে
একাধিক object-এর কিছু বৈশিষ্ট্য মিললে runtime-এ একটির জায়গায় আরেকটি বসানো যায়।

> ### polymorphism {#polymorphism}
>
> অনেকের কাছে polymorphism আর inheritance সমার্থক। কিন্তু আসলে এটি আরও সাধারণ একটি ধারণা, যা এমন কোড বোঝায় যা একাধিক type-এর ডেটা নিয়ে কাজ করতে পারে। inheritance-এর ক্ষেত্রে সেই
> type-গুলো সাধারণত subclass।
>
> Rust বরং বিভিন্ন সম্ভাব্য type-এর ওপর abstraction করতে generics ব্যবহার করে, আর সেই type-গুলোকে কী দিতে হবে তার ওপর বাধ্যবাধকতা আরোপ করতে trait bound ব্যবহার করে। একে কখনো কখনো বলা হয়
> _bounded parametric polymorphism_।

inheritance না দিয়ে Rust ভিন্ন কিছু আপস বেছে নিয়েছে। inheritance-এ প্রায়ই প্রয়োজনের চেয়ে বেশি কোড ভাগ করার ঝুঁকি থাকে। subclass-এর সবসময় parent class-এর সব বৈশিষ্ট্য ভাগ করা উচিত নয়, কিন্তু
inheritance-এ তা-ই হয়। এতে প্রোগ্রামের নকশা কম নমনীয় হতে পারে। এটি subclass-এর ওপর এমন method call করার সম্ভাবনাও তৈরি করে যা অর্থহীন বা error ঘটায়, কারণ method-গুলো subclass-এ প্রযোজ্য
নয়। তা ছাড়া কিছু ভাষা শুধু _single inheritance_ (একক উত্তরাধিকার, অর্থাৎ একটি subclass শুধু একটি class থেকে উত্তরাধিকার পায়) অনুমোদন করে, যা প্রোগ্রামের নকশার নমনীয়তা আরও সীমিত করে।

এসব কারণে runtime-এ polymorphism পেতে Rust inheritance-এর বদলে trait object ব্যবহারের ভিন্ন পদ্ধতি নেয়। চলুন দেখি trait object কীভাবে কাজ করে।

{{#quiz ../quizzes/ch17-01-what-is-oo.toml}}
