## iterator দিয়ে একের পর এক item প্রক্রিয়া করা {#processing-a-series-of-items-with-iterators}

iterator pattern দিয়ে item-এর একটি ধারার প্রতিটির ওপর পালাক্রমে কোনো কাজ করা যায়। প্রতিটি item-এর ওপর দিয়ে iterate করা এবং ধারাটি কখন
শেষ হলো তা নির্ধারণ করার logic-এর দায়িত্ব iterator-এর। iterator ব্যবহার করলে সেই logic আপনাকে নিজে আবার implement করতে হয় না।

Rust-এ iterator _lazy_ (অলস), অর্থাৎ iterator-কে খরচ করে ফেলে (consume করে) এমন method call না করা পর্যন্ত এদের কোনো প্রভাব নেই।
যেমন, Listing 13-10-এর কোড `Vec<T>`-এর ওপর সংজ্ঞায়িত `iter` method call করে `v1` vector-এর item-গুলোর ওপর একটি iterator তৈরি করে। এই
কোড নিজে থেকে কোনো কাজের কিছু করে না।

<Listing number="13-10" file-name="src/main.rs" caption="একটি iterator তৈরি করা">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-10/src/main.rs:here}}
```

</Listing>

iterator-টি `v1_iter` variable-এ রাখা আছে। একবার iterator তৈরি করলে একে নানাভাবে ব্যবহার করা যায়। Listing 3-5-এ আমরা একটি `for` loop
ব্যবহার করে একটি array-র ওপর দিয়ে iterate করে এর প্রতিটি item-এর ওপর কিছু কোড চালিয়েছিলাম। ভেতরে ভেতরে এটি অপ্রকাশ্যভাবে একটি iterator
তৈরি করে খরচ করেছিল, কিন্তু এটি ঠিক কীভাবে কাজ করে তা আমরা এতক্ষণ এড়িয়ে গেছি।

Listing 13-11-এর উদাহরণে আমরা iterator তৈরি করা আর `for` loop-এ iterator ব্যবহার করা আলাদা করেছি। `v1_iter`-এর iterator দিয়ে `for` loop
চালালে iterator-এর প্রতিটি উপাদান loop-এর একেকটি পুনরাবৃত্তিতে ব্যবহৃত হয়, যা প্রতিটি মান print করে।

<Listing number="13-11" file-name="src/main.rs" caption="একটি `for` loop-এ iterator ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-11/src/main.rs:here}}
```

</Listing>

যেসব ভাষার standard library iterator দেয় না, সেখানে আপনি সম্ভবত একই কার্যক্ষমতা লিখতেন এভাবে: একটি variable index 0 থেকে শুরু করে, সেই
variable দিয়ে vector-এ index করে একটি মান নিয়ে, আর loop-এ variable-এর মান বাড়াতে বাড়াতে যতক্ষণ না এটি vector-এর মোট item সংখ্যায়
পৌঁছায়।

iterator এই সব logic আপনার হয়ে সামলায়, ফলে এমন পুনরাবৃত্ত কোড কমে যায় যাতে আপনি ভুল করে ফেলতে পারতেন। iterator আপনাকে একই logic অনেক
রকম ধারার সঙ্গে ব্যবহারের নমনীয়তা দেয়, শুধু vector-এর মতো index করা যায় এমন data structure নয়। চলুন দেখি iterator কীভাবে তা করে।

### `Iterator` trait ও `next` method {#the-iterator-trait-and-the-next-method}

সব iterator standard library-তে সংজ্ঞায়িত `Iterator` নামের একটি trait implement করে। trait-টির সংজ্ঞা দেখতে এমন:

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```

খেয়াল করুন, এই সংজ্ঞায় কিছু নতুন syntax আছে: `type Item` আর `Self::Item`, যা এই trait-এর সঙ্গে একটি associated type (trait-এর সঙ্গে যুক্ত
type) সংজ্ঞায়িত করছে। associated type নিয়ে গভীরভাবে আলোচনা করব অধ্যায় 20-এ। আপাতত শুধু জানলেই চলবে যে এই কোড বলছে, `Iterator` trait
implement করতে হলে একটি `Item` type-ও সংজ্ঞায়িত করতে হবে, আর এই `Item` type `next` method-এর return type-এ ব্যবহৃত হয়। অন্যভাবে বললে,
`Item` type হবে iterator থেকে return হওয়া type।

`Iterator` trait implement-কারীদের কাছে শুধু একটি method সংজ্ঞায়িত করার দাবি করে: `next` method, যা একবারে iterator-এর একটি item `Some`-এ
মুড়ে return করে, আর iteration শেষ হলে `None` return করে।

আমরা iterator-এর ওপর সরাসরি `next` method call করতে পারি; Listing 13-12 দেখায় vector থেকে তৈরি iterator-এর ওপর বারবার `next` call করলে
কোন মানগুলো return হয়।

<Listing number="13-12" file-name="src/lib.rs" caption="একটি iterator-এর ওপর `next` method call করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-12/src/lib.rs:here}}
```

</Listing>

খেয়াল করুন, `v1_iter`-কে mutable করতে হয়েছে: একটি iterator-এর ওপর `next` method call করলে iterator ধারার কোথায় আছে তা মনে রাখতে যে
অভ্যন্তরীণ অবস্থা ব্যবহার করে, তা বদলে যায়। অন্যভাবে বললে, এই কোড iterator-কে _consume_ করে, বা খরচ করে ফেলে। `next`-এর প্রতিটি call
iterator থেকে একটি item খেয়ে ফেলে। `for` loop ব্যবহারের সময় `v1_iter`-কে mutable করতে হয়নি, কারণ loop `v1_iter`-এর ownership নিয়ে
পর্দার আড়ালে একে mutable করে নিয়েছিল।

আরও খেয়াল করুন, `next` call থেকে পাওয়া মানগুলো vector-এর মানগুলোর immutable reference। `iter` method immutable reference-এর ওপর একটি
iterator তৈরি করে। যদি এমন iterator তৈরি করতে চাই যা `v1`-এর ownership নেয় এবং owned মান return করে, তাহলে `iter`-এর বদলে `into_iter`
call করতে পারি। একইভাবে, mutable reference-এর ওপর iterate করতে চাইলে `iter`-এর বদলে `iter_mut` call করতে পারি।

### iterator খরচ করে এমন method {#methods-that-consume-the-iterator}

`Iterator` trait-এ standard library-র দেওয়া default implementation-সহ অনেক রকম method আছে; standard library-র API documentation-এ
`Iterator` trait দেখলে এই method-গুলো সম্পর্কে জানতে পারবেন। এর কিছু method তাদের সংজ্ঞায় `next` method call করে, এ কারণেই `Iterator`
trait implement করার সময় `next` method implement করা বাধ্যতামূলক।

যে method-গুলো `next` call করে সেগুলোকে বলা হয় _consuming adapters_ (iterator খরচ করে এমন adapter), কারণ এগুলো call করলে iterator খরচ
হয়ে যায়। একটি উদাহরণ হলো `sum` method, যা iterator-এর ownership নিয়ে বারবার `next` call করে item-গুলোর ওপর দিয়ে iterate করে, ফলে
iterator খরচ হয়ে যায়। iterate করার সময় এটি প্রতিটি item একটি চলমান যোগফলে যোগ করে এবং iteration শেষ হলে যোগফলটি return করে। Listing
13-13-এ `sum` method-এর ব্যবহার দেখানো একটি test আছে।

<Listing number="13-13" file-name="src/lib.rs" caption="iterator-এর সব item-এর যোগফল পেতে `sum` method call করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-13/src/lib.rs:here}}
```

</Listing>

`sum` call করার পর আমরা `v1_iter` ব্যবহার করতে পারি না, কারণ `sum` যে iterator-এর ওপর call করা হয় তার ownership নিয়ে নেয়।

### অন্য iterator তৈরি করে এমন method {#methods-that-produce-other-iterators}

_Iterator adapters_ (iterator adapter) হলো `Iterator` trait-এর ওপর সংজ্ঞায়িত এমন method, যা iterator খরচ করে না। বরং মূল iterator-এর
কোনো দিক বদলে এরা ভিন্ন iterator তৈরি করে।

Listing 13-14-এ iterator adapter method `map` call করার একটি উদাহরণ দেখানো হয়েছে, যা item-গুলোর ওপর দিয়ে iterate করার সময় প্রতিটি item-এর
ওপর call করার জন্য একটি closure নেয়। `map` method একটি নতুন iterator return করে, যা পরিবর্তিত item-গুলো তৈরি করে। এখানকার closure এমন একটি
নতুন iterator তৈরি করে, যাতে vector-এর প্রতিটি item 1 করে বাড়ানো হবে।

<Listing number="13-14" file-name="src/main.rs" caption="নতুন iterator তৈরি করতে iterator adapter `map` call করা">

```rust,not_desired_behavior
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-14/src/main.rs:here}}
```

</Listing>

কিন্তু এই কোড একটি warning দেয়:

```console
{{#include ../listings/ch13-functional-features/listing-13-14/output.txt}}
```

Listing 13-14-এর কোড কিছুই করে না; আমাদের দেওয়া closure কখনো call হয় না। warning আমাদের মনে করিয়ে দেয় কেন: iterator adapter lazy, আর
এখানে আমাদের iterator খরচ করতে হবে।

এই warning ঠিক করে iterator খরচ করতে আমরা `collect` method ব্যবহার করব, যা Listing 12-1-এ `env::args`-এর সঙ্গে ব্যবহার করেছিলাম। এই method
iterator খরচ করে এবং ফলাফলের মানগুলো একটি collection data type-এ জড়ো করে।

Listing 13-15-এ `map` call থেকে return হওয়া iterator-এর ওপর দিয়ে iterate করার ফলগুলো আমরা একটি vector-এ জড়ো করি। এই vector-এ শেষ পর্যন্ত
মূল vector-এর প্রতিটি item 1 করে বাড়ানো অবস্থায় থাকবে।

<Listing number="13-15" file-name="src/main.rs" caption="একটি নতুন iterator তৈরি করতে `map` method call করা, তারপর নতুন iterator খরচ করে একটি vector তৈরি করতে `collect` method call করা">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-15/src/main.rs:here}}
```

</Listing>

`map` যেহেতু একটি closure নেয়, তাই প্রতিটি item-এর ওপর আমরা যেকোনো operation চালাতে পারি। `Iterator` trait-এর দেওয়া iteration-এর আচরণ
পুনর্ব্যবহার করে closure কীভাবে কিছু আচরণ নিজের মতো সাজাতে দেয়, এটি তার একটি চমৎকার উদাহরণ।

জটিল কাজ পড়তে সহজ উপায়ে করার জন্য iterator adapter-এর একাধিক call পরপর জুড়ে দিতে (chain করতে) পারেন। কিন্তু সব iterator যেহেতু lazy,
তাই iterator adapter call-এর ফল পেতে consuming adapter method-গুলোর কোনো একটি call করতে হবে।

<!-- Old headings. Do not remove or links may break. -->

<a id="using-closures-that-capture-their-environment"></a>

### environment capture করে এমন closure {#closures-that-capture-their-environment}

অনেক iterator adapter argument হিসেবে closure নেয়, আর সাধারণত iterator adapter-এর argument হিসেবে আমরা যে closure দেব তা হবে environment
capture করে এমন closure।

এই উদাহরণে আমরা `filter` method ব্যবহার করব, যা একটি closure নেয়। closure-টি iterator থেকে একটি item পায় এবং একটি `bool` return করে।
closure `true` return করলে মানটি `filter`-এর তৈরি iteration-এ যুক্ত হবে। closure `false` return করলে মানটি যুক্ত হবে না।

Listing 13-16-এ আমরা `Shoe` struct instance-এর একটি collection-এর ওপর দিয়ে iterate করতে এমন একটি closure-সহ `filter` ব্যবহার করি, যা তার
environment থেকে `shoe_size` variable capture করে। এটি শুধু নির্দিষ্ট মাপের জুতাগুলো return করবে।

<Listing number="13-16" file-name="src/lib.rs" caption="`shoe_size` capture করে এমন একটি closure-সহ `filter` method ব্যবহার করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-16/src/lib.rs}}
```

</Listing>

`shoes_in_size` function parameter হিসেবে জুতার একটি vector-এর ownership আর একটি জুতার মাপ নেয়। এটি এমন একটি vector return করে যাতে শুধু
নির্দিষ্ট মাপের জুতা থাকে।

`shoes_in_size`-এর body-তে আমরা `into_iter` call করে এমন একটি iterator তৈরি করি যা vector-এর ownership নেয়। তারপর `filter` call করে সেই
iterator-কে এমন একটি নতুন iterator-এ রূপান্তর করি, যাতে শুধু সেই উপাদানগুলো থাকে যাদের জন্য closure `true` return করে।

closure-টি environment থেকে `shoe_size` parameter capture করে এবং মানটিকে প্রতিটি জুতার মাপের সঙ্গে তুলনা করে, শুধু নির্দিষ্ট মাপের জুতাগুলো
রেখে দেয়। শেষে `collect` call করে রূপান্তরিত iterator-এর return করা মানগুলো একটি vector-এ জড়ো করা হয়, যা function return করে।

test দেখায় যে `shoes_in_size` call করলে আমরা শুধু সেই জুতাগুলো ফেরত পাই, যাদের মাপ আমাদের দেওয়া মানের সমান।

{{#quiz ../quizzes/ch13-02-iterators.toml}}
