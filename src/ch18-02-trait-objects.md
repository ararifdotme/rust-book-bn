<!-- Old headings. Do not remove or links may break. -->

<a id="using-trait-objects-that-allow-for-values-of-different-types"></a>

## সাধারণ আচরণের ওপর abstraction করতে trait object ব্যবহার {#using-trait-objects-to-abstract-over-shared-behavior}

অধ্যায় 8-এ আমরা বলেছিলাম যে vector-এর একটি সীমাবদ্ধতা হলো এতে শুধু একটি type-এর উপাদান রাখা যায়। Listing 8-9-এ আমরা একটি বিকল্প উপায় তৈরি করেছিলাম, যেখানে integer, float আর লেখা রাখার
variant-যুক্ত একটি `SpreadsheetCell` enum সংজ্ঞায়িত করেছিলাম। এতে প্রতিটি cell-এ ভিন্ন ভিন্ন type-এর ডেটা রাখা যেত, আর তবুও cell-এর একটি সারি বোঝানো একটি vector রাখা যেত। আমাদের অদলবদলযোগ্য
item-গুলো যখন এমন নির্দিষ্ট কিছু type-এর সেট, যা কোড compile হওয়ার সময় জানা, তখন এটি একেবারে ভালো একটি সমাধান।

তবে কখনো কখনো আমরা চাই আমাদের library-র ব্যবহারকারী একটি নির্দিষ্ট পরিস্থিতিতে বৈধ type-এর সেট বাড়াতে পারুক। কীভাবে তা করা যায় দেখাতে আমরা একটি graphical user interface (GUI) tool-এর
উদাহরণ তৈরি করব, যা item-এর একটি তালিকার ওপর দিয়ে iterate করে প্রতিটির ওপর একটি `draw` method call করে স্ক্রিনে আঁকে—GUI tool-এর একটি প্রচলিত কৌশল। আমরা `gui` নামে একটি library crate তৈরি
করব, যাতে একটি GUI library-র কাঠামো থাকবে। এই crate-এ মানুষের ব্যবহারের জন্য কিছু type থাকতে পারে, যেমন `Button` বা `TextField`। তা ছাড়া `gui`-এর ব্যবহারকারীরা আঁকা যায় এমন নিজস্ব type তৈরি
করতে চাইবে: যেমন, একজন প্রোগ্রামার হয়তো একটি `Image` যোগ করবেন, আরেকজন একটি `SelectBox`।

library লেখার সময় আমরা জানতে বা সংজ্ঞায়িত করতে পারি না অন্য প্রোগ্রামাররা কোন কোন type তৈরি করতে চাইতে পারেন। তবে আমরা জানি যে `gui`-কে ভিন্ন ভিন্ন type-এর অনেক মানের হিসাব রাখতে হবে, এবং
এই ভিন্ন type-এর মানগুলোর প্রতিটির ওপর একটি `draw` method call করতে হবে। `draw` method call করলে ঠিক কী ঘটবে তা এর জানার দরকার নেই, শুধু এটুকু জানলেই চলে যে মানটিতে আমাদের call করার মতো সেই
method আছে।

inheritance আছে এমন ভাষায় এটা করতে আমরা হয়তো `Component` নামের একটি class সংজ্ঞায়িত করতাম, যাতে `draw` নামের একটি method আছে। `Button`, `Image` আর `SelectBox`-এর মতো অন্য class-গুলো
`Component` থেকে উত্তরাধিকার পেত এবং তাই `draw` method-ও উত্তরাধিকার সূত্রে পেত। এরা প্রত্যেকে নিজস্ব আচরণ সংজ্ঞায়িত করতে `draw` method override করতে পারত, কিন্তু framework সব type-কে
`Component` instance হিসেবে গণ্য করে এদের ওপর `draw` call করতে পারত। কিন্তু Rust-এ inheritance নেই বলে ব্যবহারকারীদের library-র সঙ্গে সামঞ্জস্যপূর্ণ নতুন type তৈরি করতে দিতে `gui` library
সাজানোর অন্য উপায় লাগবে।

### সাধারণ আচরণের জন্য একটি trait সংজ্ঞায়িত করা {#defining-a-trait-for-common-behavior}

`gui`-এর কাছ থেকে আমরা যে আচরণ চাই তা implement করতে `Draw` নামের একটি trait সংজ্ঞায়িত করব, যার `draw` নামের একটি method থাকবে। তারপর এমন একটি vector সংজ্ঞায়িত করতে পারি যা একটি trait object
নেয়। একটি _trait object_ আমাদের উল্লিখিত trait implement করা একটি type-এর instance আর runtime-এ সেই type-এর trait method খুঁজে বের করতে ব্যবহৃত একটি table, দুটিকেই নির্দেশ করে। trait object
তৈরি করতে আমরা কোনো এক ধরনের pointer, যেমন একটি reference বা একটি `Box<T>` smart pointer উল্লেখ করি, তারপর `dyn` keyword, তারপর প্রাসঙ্গিক trait। (trait object-কে কেন pointer ব্যবহার করতে হয়,
তা নিয়ে অধ্যায় 20-এর [“Dynamically Sized Types and the
`Sized` Trait”][dynamically-sized]<!-- ignore -->-এ কথা বলব।) generic বা concrete type-এর জায়গায় trait object ব্যবহার করা যায়। যেখানেই trait object ব্যবহার করি, Rust-এর type system compile
time-এ নিশ্চিত করবে যে সেই প্রেক্ষাপটে ব্যবহৃত যেকোনো মান trait object-এর trait implement করবে। ফলে compile time-এ সব সম্ভাব্য type জানার দরকার নেই।

আমরা বলেছি যে Rust-এ অন্য ভাষার object থেকে আলাদা করতে struct আর enum-কে “object” বলা থেকে বিরত থাকি। একটি struct বা enum-এ struct-এর field-এর ডেটা আর `impl` block-এর আচরণ আলাদা থাকে,
অন্যদিকে অন্য ভাষায় ডেটা আর আচরণকে একটি ধারণায় মেলানো হলে প্রায়ই তাকে object বলা হয়। trait object অন্য ভাষার object থেকে এই দিক দিয়ে আলাদা যে trait object-এ ডেটা যোগ করা যায় না। অন্য ভাষার
object-এর মতো trait object সাধারণভাবে অতটা কাজের নয়: এদের নির্দিষ্ট উদ্দেশ্য হলো সাধারণ আচরণের ওপর abstraction সম্ভব করা।

Listing 18-3-এ দেখানো হয়েছে `draw` নামের একটি method-যুক্ত `Draw` নামের trait কীভাবে সংজ্ঞায়িত করতে হয়।

<Listing number="18-3" file-name="src/lib.rs" caption="`Draw` trait-এর সংজ্ঞা">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-03/src/lib.rs}}
```

</Listing>

অধ্যায় 10-এ trait কীভাবে সংজ্ঞায়িত করতে হয় তা নিয়ে আলোচনা থেকে এই syntax পরিচিত লাগার কথা। এরপর আসছে কিছু নতুন syntax: Listing 18-4 `Screen` নামের একটি struct সংজ্ঞায়িত করে, যা `components`
নামের একটি vector রাখে। এই vector-এর type `Box<dyn Draw>`, যা একটি trait object; এটি একটি `Box`-এর ভেতরে থাকা `Draw` trait implement করা যেকোনো type-এর বদলি।

<Listing number="18-4" file-name="src/lib.rs" caption="`Draw` trait implement করা trait object-এর একটি vector রাখা `components` field-সহ `Screen` struct-এর সংজ্ঞা">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-04/src/lib.rs:here}}
```

</Listing>

`Screen` struct-এ আমরা `run` নামের একটি method সংজ্ঞায়িত করব, যা এর প্রতিটি `components`-এর ওপর `draw` method call করবে, যেমনটি Listing 18-5-এ দেখানো হয়েছে।

<Listing number="18-5" file-name="src/lib.rs" caption="`Screen`-এর ওপর একটি `run` method, যা প্রতিটি component-এর ওপর `draw` method call করে">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-05/src/lib.rs:here}}
```

</Listing>

এটি trait bound-সহ একটি generic type parameter ব্যবহার করা struct সংজ্ঞায়িত করা থেকে ভিন্নভাবে কাজ করে। একটি generic type parameter-এর জায়গায় একবারে শুধু একটি concrete type বসানো যায়, অন্যদিকে
trait object runtime-এ trait object-এর জায়গায় একাধিক concrete type বসতে দেয়। যেমন, আমরা একটি generic type আর একটি trait bound ব্যবহার করে `Screen` struct সংজ্ঞায়িত করতে পারতাম, যেমনটি Listing
18-6-এ।

<Listing number="18-6" file-name="src/lib.rs" caption="generics আর trait bound ব্যবহার করে `Screen` struct আর এর `run` method-এর একটি বিকল্প implementation">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-06/src/lib.rs:here}}
```

</Listing>

এতে আমরা এমন একটি `Screen` instance-এ সীমাবদ্ধ থাকি, যার component-এর তালিকার সবগুলো `Button` type-এর অথবা সবগুলো `TextField` type-এর। আপনার যদি সবসময় শুধু একই ধরনের collection থাকে,
তাহলে generics আর trait bound ব্যবহার করাই ভালো, কারণ সংজ্ঞাগুলো compile time-এ monomorphize হয়ে concrete type ব্যবহার করবে।

অন্যদিকে trait object ব্যবহার করা method-এ একটি `Screen` instance এমন একটি `Vec<T>` রাখতে পারে, যাতে একটি `Box<Button>` আর একটি
`Box<TextField>` দুটিই থাকে। চলুন দেখি এটি কীভাবে কাজ করে, তারপর runtime performance-এর ওপর এর প্রভাব নিয়ে কথা বলব।

### trait implement করা {#implementing-the-trait}

এবার আমরা `Draw` trait implement করা কিছু type যোগ করব। আমরা `Button` type দেব। আবারও বলি, আসলে একটি GUI library implement করা এই বইয়ের আওতার বাইরে, তাই `draw` method-এর body-তে কোনো কাজের
implementation থাকবে না। implementation দেখতে কেমন হতে পারে তা কল্পনা করতে, একটি `Button` struct-এ `width`, `height` আর `label`-এর field থাকতে পারে, যেমনটি Listing 18-7-এ দেখানো হয়েছে।

<Listing number="18-7" file-name="src/lib.rs" caption="`Draw` trait implement করা একটি `Button` struct">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-07/src/lib.rs:here}}
```

</Listing>

`Button`-এর `width`, `height` আর `label` field অন্য component-এর field থেকে আলাদা হবে; যেমন, একটি `TextField` type-এ সেই একই field-গুলোর সঙ্গে একটি `placeholder` field-ও থাকতে পারে। স্ক্রিনে যে
type-গুলো আঁকতে চাই তার প্রতিটি `Draw` trait implement করবে, কিন্তু সেই নির্দিষ্ট type কীভাবে আঁকা হবে তা সংজ্ঞায়িত করতে `draw` method-এ ভিন্ন কোড ব্যবহার করবে, যেমন এখানে `Button` করেছে (আগেই
যেমন বলা হয়েছে, আসল GUI কোড ছাড়া)। যেমন, `Button` type-এ ব্যবহারকারী button-এ click করলে কী ঘটে সংক্রান্ত method-যুক্ত একটি বাড়তি `impl` block থাকতে পারে। এই ধরনের method `TextField`-এর মতো
type-এ প্রযোজ্য হবে না।

আমাদের library ব্যবহার করা কেউ যদি `width`, `height` আর `options` field-যুক্ত একটি `SelectBox` struct implement করার সিদ্ধান্ত নেন, তাহলে তিনি `SelectBox` type-এর ওপরও `Draw` trait implement
করবেন, যেমনটি Listing 18-8-এ দেখানো হয়েছে।

<Listing number="18-8" file-name="src/main.rs" caption="`gui` ব্যবহার করা এবং একটি `SelectBox` struct-এর ওপর `Draw` trait implement করা আরেকটি crate">

```rust,ignore
{{#rustdoc_include ../listings/ch18-oop/listing-18-08/src/main.rs:here}}
```

</Listing>

### trait ব্যবহার করা {#using-the-trait}

আমাদের library-র ব্যবহারকারী এখন একটি `Screen` instance তৈরি করতে তাদের `main` function লিখতে পারেন। `Screen` instance-এ তারা একটি `SelectBox` আর একটি `Button` যোগ করতে পারেন, প্রতিটিকে একটি
`Box<T>`-এ রেখে trait object বানিয়ে। তারপর `Screen` instance-এর ওপর `run` method call করতে পারেন, যা প্রতিটি component-এর ওপর `draw` call করবে। Listing 18-9-এ এই implementation দেখানো
হয়েছে।

<Listing number="18-9" file-name="src/main.rs" caption="একই trait implement করা ভিন্ন ভিন্ন type-এর মান রাখতে trait object ব্যবহার করা">

```rust,ignore
{{#rustdoc_include ../listings/ch18-oop/listing-18-09/src/main.rs:here}}
```

</Listing>

library লেখার সময় আমরা জানতাম না যে কেউ `SelectBox` type যোগ করতে পারে, তবু আমাদের `Screen` implementation নতুন type-টির ওপর কাজ করে একে আঁকতে পেরেছে, কারণ `SelectBox` `Draw` trait implement
করে, অর্থাৎ এটি `draw` method implement করে।

এই ধারণা—একটি মানের concrete type নয়, বরং মানটি কোন message-এ সাড়া দেয় শুধু তা নিয়েই মাথা ঘামানো—dynamically typed ভাষার _duck
typing_-এর ধারণার মতো: যদি এটি হাঁসের মতো হাঁটে আর হাঁসের মতো প্যাঁকপ্যাঁক করে, তাহলে এটি নিশ্চয়ই হাঁস! Listing 18-5-এ `Screen`-এর ওপর `run`-এর implementation-এ `run`-এর জানার দরকার নেই
প্রতিটি component-এর concrete type কী। এটি যাচাই করে না একটি component `Button` নাকি `SelectBox`-এর instance, শুধু component-এর ওপর `draw` method call করে। `components` vector-এর মানগুলোর type
হিসেবে `Box<dyn Draw>` উল্লেখ করে আমরা `Screen`-কে এমনভাবে সংজ্ঞায়িত করেছি যাতে এর এমন মান লাগে যার ওপর `draw` method call করা যায়।

duck typing ব্যবহার করা কোডের মতো কোড লিখতে trait object আর Rust-এর type system ব্যবহারের সুবিধা হলো, runtime-এ কখনো যাচাই করতে হয় না একটি মান নির্দিষ্ট কোনো method implement করে কি না, বা
কোনো মান method implement না করলেও call করে ফেললে error পাওয়ার চিন্তা করতে হয় না। trait object-এর দরকারি trait মানগুলো implement না করলে Rust আমাদের কোড compile করবে না।

যেমন, Listing 18-10 দেখায় component হিসেবে একটি `String` দিয়ে একটি `Screen` তৈরির চেষ্টা করলে কী ঘটে।

<Listing number="18-10" file-name="src/main.rs" caption="trait object-এর trait implement করে না এমন একটি type ব্যবহারের চেষ্টা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-oop/listing-18-10/src/main.rs}}
```

</Listing>

এই error পাব, কারণ `String` `Draw` trait implement করে না:

```console
{{#include ../listings/ch18-oop/listing-18-10/output.txt}}
```

এই error জানায় যে হয় আমরা `Screen`-এ এমন কিছু পাঠাচ্ছি যা পাঠাতে চাইনি এবং তাই ভিন্ন একটি type পাঠানো উচিত, নয়তো `String`-এর ওপর `Draw` implement করা উচিত, যাতে `Screen` এর ওপর `draw` call
করতে পারে।

<!-- BEGIN INTERVENTION: cce62358-5291-4eb3-84d6-fbc570873ee3 -->

### trait object ও type অনুমান {#trait-objects-and-type-inference}

trait object ব্যবহারের একটি অসুবিধা হলো type অনুমানের সঙ্গে এদের আচরণ। 
যেমন, `Vec<T>`-এর type অনুমানের কথা ভাবুন। `T` যখন trait object নয়,
তখন `T` অনুমান করতে Rust-এর শুধু vector-এর একটি উপাদানের type জানলেই চলে। তাই
একটি খালি vector type অনুমানের error ঘটায়:

```rust,ignore,does_not_compile
# fn main() {
let v = vec![];
// error[E0282]: type annotations needed for `Vec<T>`
# }
```

কিন্তু একটি উপাদান যোগ করলে Rust vector-এর type অনুমান করতে পারে:

```rust,ignore
# fn main() {
let v = vec!["Hello world"];
// ok, v : Vec<&str>
# }
```

trait object-এর ক্ষেত্রে type অনুমান আরও জটিল। যেমন, ধরুন আমরা Listing 18-9-এর `components` array-টি 
একটি আলাদা variable-এ বের করে আনার চেষ্টা করলাম, এভাবে:

```rust,ignore,does_not_compile
fn main() {
    let components = vec![
        Box::new(SelectBox { /* .. */ }),
        Box::new(Button { /* .. */ }),
    ];
    let screen = Screen { components };
    screen.run();
}
```

<span class="caption">Listing 18-11: components array বের করে আনলে একটি type error হয়</span>

এই refactor-এর ফলে প্রোগ্রামটি আর compile হয় না! compiler এই error দিয়ে প্রোগ্রামটি বাতিল করে:

```text
error[E0308]: mismatched types
   --> test.rs:55:14
    |
55  |       Box::new(Button {
    |  _____--------_^
    | |     |
    | |     arguments to this function are incorrect
56  | |       width: 50,
57  | |       height: 10,
58  | |       label: String::from("OK"),
59  | |     }),
    | |_____^ expected `SelectBox`, found `Button`
```

Listing 18-09-এ compiler বোঝে যে `components` vector-এর type হতে হবে
`Vec<Box<dyn Draw>>`, কারণ `Screen` struct-এর সংজ্ঞায় তা উল্লেখ করা আছে। কিন্তু Listing 18-11-এ
`components` যেখানে সংজ্ঞায়িত, সেখানে compiler সেই তথ্য হারিয়ে ফেলে। সমস্যাটি ঠিক করতে আপনাকে
type অনুমানের algorithm-কে একটি ইঙ্গিত দিতে হবে। তা হতে পারে vector-এর যেকোনো উপাদানের ওপর একটি স্পষ্ট cast দিয়ে, এভাবে:

```rust,ignore
let components = vec![
    Box::new(SelectBox { /* .. */ }) as Box<dyn Draw>,
    Box::new(Button { /* .. */ }),
];
```

অথবা let-binding-এ একটি type annotation দিয়ে, এভাবে:

```rust,ignore
let components: Vec<Box<dyn Draw>> = vec![
    Box::new(SelectBox { /* .. */ }),
    Box::new(Button { /* .. */ }),
];
```

সাধারণভাবে জেনে রাখা ভালো যে type অনুমানের ক্ষেত্রে trait object ব্যবহার API-র ব্যবহারকারী ডেভেলপারদের অভিজ্ঞতা খারাপ করতে পারে।

<!-- END INTERVENTION: cce62358-5291-4eb3-84d6-fbc570873ee3 -->


<!-- Old headings. Do not remove or links may break. -->

<a id="trait-objects-perform-dynamic-dispatch"></a>

### dynamic dispatch সম্পাদন করা {#performing-dynamic-dispatch}

অধ্যায় 10-এর [“generics ব্যবহার করা কোডের
performance”][performance-of-code-using-generics]<!-- ignore -->-এ compiler-এর generics-এর ওপর চালানো monomorphization প্রক্রিয়া নিয়ে আলোচনাটি মনে করুন: generic type parameter-এর জায়গায়
আমরা যে প্রতিটি concrete type ব্যবহার করি, তার জন্য compiler function আর method-এর generic নয় এমন implementation তৈরি করে। monomorphization থেকে তৈরি কোড _static
dispatch_ (স্থির dispatch) করে, অর্থাৎ compile time-এই compiler জানে আপনি কোন method call করছেন। এর বিপরীত হলো _dynamic dispatch_ (চলার সময় dispatch), যখন compiler compile time-এ বলতে পারে না
আপনি কোন method call করছেন। dynamic dispatch-এর ক্ষেত্রে compiler এমন কোড তৈরি করে, যা runtime-এ জানবে কোন method call করতে হবে।

trait object ব্যবহার করলে Rust-কে dynamic dispatch ব্যবহার করতেই হয়। trait object ব্যবহার করা কোডের সঙ্গে কোন কোন type ব্যবহার হতে পারে তা compiler জানে না, তাই এটি জানে না কোন type-এ implement
করা কোন method call করতে হবে। বরং runtime-এ কোন method call করতে হবে তা জানতে Rust trait object-এর ভেতরের pointer-গুলো ব্যবহার করে। এই খোঁজাখুঁজিতে runtime-এ একটি খরচ হয়, যা static dispatch-এ
হয় না। dynamic dispatch compiler-কে একটি method-এর কোড inline (call-এর জায়গায় সরাসরি বসানো) করতেও দেয় না, যা আবার কিছু optimization আটকে দেয়, আর কোথায় dynamic dispatch ব্যবহার করা যায় আর
কোথায় যায় না সে বিষয়ে Rust-এর কিছু নিয়ম আছে, যাকে বলা হয় _dyn compatibility_। সেই নিয়মগুলো এই আলোচনার আওতার বাইরে, তবে [reference-এ][dyn-compatibility]<!-- ignore --> এগুলো সম্পর্কে আরও পড়তে
পারেন। তবে Listing 18-5-এ লেখা কোডে আমরা বাড়তি নমনীয়তা পেয়েছি এবং Listing 18-9-এ তা সমর্থন করতে পেরেছি, তাই এটি বিবেচনা করার মতো একটি আপস।

{{#quiz ../quizzes/ch17-02-trait-objects.toml}}

[performance-of-code-using-generics]: ch10-01-syntax.html#performance-of-code-using-generics
[dynamically-sized]: ch20-03-advanced-types.html#dynamically-sized-types-and-the-sized-trait
[dyn-compatibility]: https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility
