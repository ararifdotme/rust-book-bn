## পরিশিষ্ট C: derive করা যায় এমন trait {#appendix-c-derivable-traits}

বইয়ের নানা জায়গায় আমরা `derive` attribute নিয়ে আলোচনা করেছি, যা একটি struct বা enum-এর সংজ্ঞায় প্রয়োগ করা যায়। `derive` attribute এমন কোড তৈরি করে, যা `derive` syntax দিয়ে annotate করা type-এ নিজস্ব default implementation-সহ একটি trait implement করবে।

এই পরিশিষ্টে standard library-র সেই সব trait-এর একটি তথ্যসূত্র দেওয়া হলো, যেগুলো `derive`-এর সঙ্গে ব্যবহার করা যায়। প্রতিটি অংশে আলোচনা করা হয়েছে:

- এই trait derive করলে কোন operator আর method ব্যবহার করা যাবে
- `derive`-এর দেওয়া trait implementation কী করে
- trait implement করা type সম্পর্কে কী বোঝায়
- কোন পরিস্থিতিতে trait implement করার অনুমতি আছে বা নেই
- এমন operation-এর উদাহরণ, যেগুলোর জন্য trait-টি দরকার

`derive` attribute-এর দেওয়া আচরণ থেকে ভিন্ন আচরণ চাইলে প্রতিটি trait কীভাবে হাতে implement করতে হয় তার বিস্তারিত জানতে
[standard library documentation](../std/index.html)<!-- ignore --> দেখুন।

এখানে তালিকাভুক্ত trait-গুলোই standard library-তে সংজ্ঞায়িত একমাত্র trait, যেগুলো `derive` ব্যবহার করে আপনার type-এ implement করা যায়। standard library-তে সংজ্ঞায়িত অন্য trait-গুলোর কোনো যুক্তিসঙ্গত default আচরণ নেই, তাই আপনি যা অর্জন করতে চাইছেন সেই অনুযায়ী অর্থবহ উপায়ে সেগুলো implement করা আপনার দায়িত্ব।

derive করা যায় না এমন trait-এর একটি উদাহরণ হলো `Display`, যা শেষ ব্যবহারকারীর জন্য formatting সামলায়। একটি type শেষ ব্যবহারকারীকে দেখানোর উপযুক্ত উপায় আপনাকে সবসময় বিবেচনা করতে হবে। type-এর কোন অংশ শেষ ব্যবহারকারীকে দেখতে দেওয়া উচিত? কোন অংশ তার কাছে প্রাসঙ্গিক মনে হবে? ডেটার কোন format তার কাছে সবচেয়ে প্রাসঙ্গিক হবে? Rust compiler-এর এই অন্তর্দৃষ্টি নেই, তাই এটি আপনার জন্য উপযুক্ত default আচরণ দিতে পারে না।

এই পরিশিষ্টে দেওয়া derive করা যায় এমন trait-এর তালিকা সম্পূর্ণ নয়: library-গুলো নিজেদের trait-এর জন্য `derive` implement করতে পারে, ফলে `derive`-এর সঙ্গে ব্যবহারযোগ্য trait-এর তালিকা সত্যিই উন্মুক্ত। `derive` implement করতে একটি procedural macro ব্যবহার করতে হয়, যা অধ্যায় 20-এর [“নিজস্ব `derive`
macro”][custom-derive-macros]<!-- ignore --> অংশে আলোচনা করা হয়েছে।

### programmer-এর জন্য output: `Debug` {#debug-for-programmer-output}

`Debug` trait format string-এ debug formatting চালু করে, যা `{}` placeholder-এর ভেতরে `:?` যোগ করে বোঝানো হয়।

`Debug` trait debug করার উদ্দেশ্যে একটি type-এর instance print করতে দেয়, যাতে আপনি এবং আপনার type ব্যবহারকারী অন্য programmer-রা প্রোগ্রাম চলার নির্দিষ্ট মুহূর্তে একটি instance পরীক্ষা করে দেখতে পারেন।

যেমন, `assert_eq!` macro ব্যবহারে `Debug` trait দরকার। সমতার assertion ব্যর্থ হলে এই macro argument হিসেবে দেওয়া instance-গুলোর মান print করে, যাতে programmer-রা দেখতে পারেন দুটি instance কেন সমান হয়নি।

### সমতার তুলনার জন্য `PartialEq` ও `Eq` {#partialeq-and-eq-for-equality-comparisons}

`PartialEq` trait একটি type-এর instance-গুলো সমান কি না তা তুলনা করতে দেয়, এবং `==` আর `!=` operator ব্যবহার করা সম্ভব করে।

`PartialEq` derive করলে `eq` method implement হয়। struct-এ `PartialEq` derive করলে দুটি instance সমান হবে শুধু যদি _সব_ field সমান হয়, আর _কোনো একটি_ field সমান না হলেই instance-গুলো সমান নয়। enum-এ derive করলে প্রতিটি variant নিজের সমান এবং অন্য variant-গুলোর সমান নয়।

যেমন, `assert_eq!` macro ব্যবহারে `PartialEq` trait দরকার, কারণ এটিকে একটি type-এর দুটি instance সমান কি না তুলনা করতে পারতে হয়।

`Eq` trait-এর কোনো method নেই। এর উদ্দেশ্য হলো জানানো যে annotate করা type-এর প্রতিটি মান নিজের সমান। `Eq` trait শুধু এমন type-এ প্রয়োগ করা যায়, যেগুলো `PartialEq`-ও implement করে, যদিও `PartialEq` implement করা সব type `Eq` implement করতে পারে না। এর একটি উদাহরণ হলো floating-point সংখ্যার type: floating-point সংখ্যার implementation বলে যে not-a-number (`NaN`) মানের দুটি instance একে অপরের সমান নয়।

`Eq` কখন দরকার তার একটি উদাহরণ হলো `HashMap<K, V>`-এর key, যাতে `HashMap<K, V>` বলতে পারে দুটি key একই কি না।

### ক্রমের তুলনার জন্য `PartialOrd` ও `Ord` {#partialord-and-ord-for-ordering-comparisons}

`PartialOrd` trait sort করার উদ্দেশ্যে একটি type-এর instance-গুলো তুলনা করতে দেয়। `PartialOrd` implement করা type `<`, `>`, `<=` আর `>=` operator-এর সঙ্গে ব্যবহার করা যায়। `PartialOrd` trait শুধু এমন type-এ প্রয়োগ করা যায়, যেগুলো `PartialEq`-ও implement করে।

`PartialOrd` derive করলে `partial_cmp` method implement হয়, যা একটি `Option<Ordering>` return করে; দেওয়া মানগুলো থেকে কোনো ক্রম তৈরি না হলে এটি `None` হবে। এমন মানের একটি উদাহরণ, যা থেকে কোনো ক্রম তৈরি হয় না, যদিও সেই type-এর বেশিরভাগ মান তুলনা করা যায়, তা হলো `NaN` floating point মান। যেকোনো floating-point সংখ্যা আর `NaN` floating-point মান দিয়ে `partial_cmp` call করলে `None` return হবে।

struct-এ derive করলে `PartialOrd` দুটি instance তুলনা করে প্রতিটি field-এর মান তুলনা করে, struct-এর সংজ্ঞায় field-গুলো যে ক্রমে আছে সেই ক্রমে। enum-এ derive করলে enum-এর সংজ্ঞায় আগে ঘোষিত variant-গুলোকে পরে তালিকাভুক্ত variant-গুলোর চেয়ে ছোট ধরা হয়।

যেমন, `rand` crate-এর `gen_range` method-এর জন্য `PartialOrd` trait দরকার, যা একটি range expression দিয়ে নির্দিষ্ট করা range-এর মধ্যে একটি random মান তৈরি করে।

`Ord` trait আপনাকে জানতে দেয় যে annotate করা type-এর যেকোনো দুটি মানের জন্য একটি বৈধ ক্রম থাকবে। `Ord` trait `cmp` method implement করে, যা `Option<Ordering>`-এর বদলে একটি `Ordering` return করে, কারণ একটি বৈধ ক্রম সবসময় সম্ভব। `Ord` trait শুধু এমন type-এ প্রয়োগ করা যায়, যেগুলো `PartialOrd` আর `Eq`-ও implement করে (আর `Eq`-এর জন্য `PartialEq` লাগে)। struct আর enum-এ derive করলে `cmp` ঠিক সেভাবে আচরণ করে, যেভাবে `PartialOrd`-এর ক্ষেত্রে `partial_cmp`-এর derive করা implementation করে।

`Ord` কখন দরকার তার একটি উদাহরণ হলো `BTreeSet<T>`-এ মান রাখা, যা এমন একটি data structure যেটি মানগুলোর sort ক্রমের ভিত্তিতে ডেটা রাখে।

### মানের প্রতিলিপি তৈরির জন্য `Clone` ও `Copy` {#clone-and-copy-for-duplicating-values}

`Clone` trait স্পষ্টভাবে একটি মানের deep copy তৈরি করতে দেয়, আর প্রতিলিপি তৈরির প্রক্রিয়ায় যেকোনো কোড চালানো ও heap ডেটা copy করা থাকতে পারে।

`Clone` derive করলে `clone` method implement হয়, যা পুরো type-এর জন্য implement করা হলে type-এর প্রতিটি অংশের ওপর `clone` call করে। এর মানে `Clone` derive করতে type-এর সব field বা মানকেও `Clone` implement করতে হবে।

`Clone` কখন দরকার তার একটি উদাহরণ হলো একটি slice-এর ওপর `to_vec` method call করা। slice তার ভেতরের type instance-গুলোর মালিক নয়, কিন্তু `to_vec` থেকে return হওয়া vector-কে তার instance-গুলোর মালিক হতে হবে, তাই `to_vec` প্রতিটি item-এর ওপর `clone` call করে। ফলে slice-এ রাখা type-কে `Clone` implement করতে হবে।

`Copy` trait শুধু stack-এ রাখা bit copy করে একটি মানের প্রতিলিপি তৈরি করতে দেয়; কোনো যথেচ্ছ কোডের দরকার হয় না।

`Copy` trait কোনো method সংজ্ঞায়িত করে না, যাতে programmer-রা সেই method overload করে “কোনো যথেচ্ছ কোড চলছে না” এই অনুমান ভাঙতে না পারেন। এভাবে সব programmer ধরে নিতে পারেন যে একটি মান copy করা খুব দ্রুত হবে।

যে type-এর সব অংশ `Copy` implement করে, তাতে `Copy` derive করা যায়। `Copy` implement করা type-কে `Clone`-ও implement করতে হবে, কারণ `Copy` implement করা type-এ `Clone`-এর একটি সাদামাটা implementation থাকে, যা `Copy`-এর মতোই কাজ করে।

`Copy` trait কদাচিৎ দরকার হয়; `Copy` implement করা type-এর জন্য optimization পাওয়া যায়, অর্থাৎ আপনাকে `clone` call করতে হয় না, যা কোডকে আরও সংক্ষিপ্ত করে।

`Copy` দিয়ে যা সম্ভব তার সবকিছু `Clone` দিয়েও করা যায়, তবে কোড ধীর হতে পারে বা জায়গায় জায়গায় `clone` ব্যবহার করতে হতে পারে।

### একটি মানকে নির্দিষ্ট আকারের মানে map করার জন্য `Hash` {#hash-for-mapping-a-value-to-a-value-of-fixed-size}

`Hash` trait যেকোনো আকারের একটি type-এর instance নিয়ে একটি hash function ব্যবহার করে সেটিকে নির্দিষ্ট আকারের একটি মানে map করতে দেয়। `Hash` derive করলে `hash` method implement হয়। `hash` method-এর derive করা implementation type-এর প্রতিটি অংশের ওপর `hash` call করার ফল একত্র করে, অর্থাৎ `Hash` derive করতে সব field বা মানকেও `Hash` implement করতে হবে।

`Hash` কখন দরকার তার একটি উদাহরণ হলো দক্ষভাবে ডেটা রাখতে একটি `HashMap<K, V>`-এ key রাখা।

### default মানের জন্য `Default` {#default-for-default-values}

`Default` trait একটি type-এর জন্য একটি default মান তৈরি করতে দেয়। `Default` derive করলে `default` function implement হয়। `default` function-এর derive করা implementation type-এর প্রতিটি অংশের ওপর `default` function call করে, অর্থাৎ `Default` derive করতে type-এর সব field বা মানকেও `Default` implement করতে হবে।

`Default::default` function সাধারণত অধ্যায় 5-এর [“struct update syntax দিয়ে instance তৈরি
করা”][creating-instances-from-other-instances-with-struct-update-syntax]<!--
ignore --> অংশে আলোচিত struct update syntax-এর সঙ্গে মিলিয়ে ব্যবহার করা হয়। `..Default::default()` ব্যবহার করে একটি struct-এর কয়েকটি field নিজের মতো ঠিক করে বাকি field-গুলোর জন্য একটি default মান বসিয়ে ব্যবহার করতে পারেন।

যেমন, `Option<T>` instance-এর ওপর `unwrap_or_default` method ব্যবহার করলে `Default` trait দরকার হয়। `Option<T>` যদি `None` হয়, তাহলে `unwrap_or_default` method `Option<T>`-এ রাখা type `T`-এর জন্য `Default::default`-এর ফল return করবে।

[creating-instances-from-other-instances-with-struct-update-syntax]: ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax
[stack-only-data-copy]: ch04-01-what-is-ownership.html#stack-only-data-copy
[variables-and-data-interacting-with-clone]: ch04-01-what-is-ownership.html#variables-and-data-interacting-with-clone
[custom-derive-macros]: ch20-05-macros.html#custom-derive-macros
