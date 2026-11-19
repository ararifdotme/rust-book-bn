## উন্নত trait {#advanced-traits}

trait নিয়ে আমরা প্রথম আলোচনা করেছিলাম অধ্যায় 10-এর [“trait দিয়ে সাধারণ আচরণ সংজ্ঞায়িত
করা”][traits]<!-- ignore --> অংশে, কিন্তু আরও উন্নত খুঁটিনাটি নিয়ে আলোচনা করিনি। Rust সম্পর্কে আরও জানার পর এখন আমরা খুঁটিনাটির গভীরে যেতে পারি।

<!-- Old headings. Do not remove or links may break. -->

<a id="specifying-placeholder-types-in-trait-definitions-with-associated-types"></a>
<a id="associated-types"></a>

### associated type দিয়ে trait সংজ্ঞায়িত করা {#defining-traits-with-associated-types}

_Associated types_ (associated type) একটি type placeholder-কে একটি trait-এর সঙ্গে এমনভাবে যুক্ত করে যে trait-এর method-এর সংজ্ঞাগুলো তাদের signature-এ এই placeholder type ব্যবহার করতে পারে।
trait-এর implement-কারী নির্দিষ্ট implementation-এর জন্য placeholder type-এর বদলে কোন concrete type ব্যবহার হবে তা উল্লেখ করবে। এভাবে আমরা এমন একটি trait সংজ্ঞায়িত করতে পারি, যা কিছু type
ব্যবহার করে, trait implement না হওয়া পর্যন্ত সেই type-গুলো ঠিক কী তা না জেনেই।

এই অধ্যায়ের বেশিরভাগ উন্নত বৈশিষ্ট্যকে আমরা কদাচিৎ দরকার হয় বলে বর্ণনা করেছি। associated type মাঝামাঝি কোথাও: বইয়ের বাকি অংশে ব্যাখ্যা করা বৈশিষ্ট্যগুলোর চেয়ে এগুলো কম ব্যবহৃত হয়, তবে এই
অধ্যায়ে আলোচিত অন্য অনেক বৈশিষ্ট্যের চেয়ে বেশি।

associated type-যুক্ত trait-এর একটি উদাহরণ হলো standard library-র দেওয়া `Iterator` trait। associated type-টির নাম `Item`, আর এটি `Iterator` trait implement করা type যে মানগুলোর ওপর দিয়ে
iterate করছে তাদের type-এর বদলি। `Iterator` trait-এর সংজ্ঞা Listing
20-13-এ দেখানো হয়েছে।

<Listing number="20-13" caption="associated type `Item`-যুক্ত `Iterator` trait-এর সংজ্ঞা">

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-13/src/lib.rs}}
```

</Listing>

`Item` type একটি placeholder, আর `next` method-এর সংজ্ঞা দেখায় যে এটি `Option<Self::Item>` type-এর মান return করবে। `Iterator` trait-এর implement-কারীরা `Item`-এর জন্য concrete type উল্লেখ করবে,
আর `next` method সেই concrete type-এর একটি মানযুক্ত একটি `Option` return করবে।

associated type-কে generics-এর মতো একটি ধারণা মনে হতে পারে, কারণ generics দিয়ে একটি function কোন type সামলাতে পারে তা উল্লেখ না করেই সংজ্ঞায়িত করা যায়। দুটি ধারণার পার্থক্য পরীক্ষা করতে আমরা
`Counter` নামের একটি type-এর ওপর `Iterator` trait-এর একটি implementation দেখব, যা উল্লেখ করে যে `Item` type হলো `u32`:

<Listing file-name="src/lib.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-22-iterator-on-counter/src/lib.rs:ch19}}
```

</Listing>

এই syntax generics-এর syntax-এর সঙ্গে তুলনীয় মনে হয়। তাহলে কেন শুধু generics দিয়ে `Iterator` trait সংজ্ঞায়িত করা হয় না, যেমনটি Listing 20-14-এ দেখানো হয়েছে?

<Listing number="20-14" caption="generics ব্যবহার করে `Iterator` trait-এর একটি কাল্পনিক সংজ্ঞা">

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-14/src/lib.rs}}
```

</Listing>

পার্থক্য হলো, Listing 20-14-এর মতো generics ব্যবহার করলে প্রতিটি implementation-এ type annotate করতে হয়; কারণ আমরা `Iterator<String> for Counter` বা অন্য যেকোনো type-ও implement করতে পারি, তাই
`Counter`-এর জন্য `Iterator`-এর একাধিক implementation থাকতে পারত। অন্যভাবে বললে, একটি trait-এর generic parameter থাকলে একে একটি type-এর জন্য একাধিকবার implement করা যায়, প্রতিবার generic type
parameter-এর concrete type বদলে। `Counter`-এর ওপর `next` method ব্যবহার করলে `Iterator`-এর কোন implementation ব্যবহার করতে চাই তা বোঝাতে type annotation দিতে হতো।

associated type-এর ক্ষেত্রে type annotate করতে হয় না, কারণ একটি type-এর ওপর একটি trait একাধিকবার implement করা যায় না। associated type ব্যবহার করা Listing 20-13-এর সংজ্ঞায় `Item`-এর type কী
হবে তা আমরা মাত্র একবারই বেছে নিতে পারি, কারণ শুধু একটি `impl Iterator for Counter` থাকতে পারে। `Counter`-এর ওপর যেখানেই `next` call করি, সেখানেই আমরা `u32` মানের iterator চাই তা উল্লেখ করতে
হয় না।

associated type trait-এর চুক্তির অংশও হয়ে যায়: trait-এর implement-কারীদের associated type placeholder-এর বদলি হিসেবে একটি type দিতে হবে। associated type-এর নাম প্রায়ই বর্ণনা করে type-টি কীভাবে
ব্যবহার হবে, আর API documentation-এ associated type-এর বিবরণ লেখা একটি ভালো অভ্যাস।

<!-- Old headings. Do not remove or links may break. -->

<a id="default-generic-type-parameters-and-operator-overloading"></a>

### default generic parameter ও operator overloading ব্যবহার {#using-default-generic-parameters-and-operator-overloading}

generic type parameter ব্যবহারের সময় generic type-এর জন্য একটি default concrete type উল্লেখ করা যায়। default type কাজ করলে trait-এর implement-কারীদের আর concrete type উল্লেখ করতে হয় না।
`<PlaceholderType=ConcreteType>` syntax দিয়ে একটি generic type ঘোষণার সময় default type উল্লেখ করেন।

এই কৌশল কাজে লাগে এমন পরিস্থিতির একটি চমৎকার উদাহরণ হলো _operator
overloading_, যেখানে নির্দিষ্ট পরিস্থিতিতে একটি operator-এর (যেমন `+`) আচরণ নিজের মতো সাজান।

Rust আপনাকে নিজস্ব operator তৈরি করতে বা ইচ্ছামতো operator overload করতে দেয় না। তবে operator-এর সঙ্গে যুক্ত trait implement করে `std::ops`-এ তালিকাভুক্ত operation আর সংশ্লিষ্ট trait overload করতে
পারেন। যেমন, Listing 20-15-এ আমরা দুটি `Point` instance যোগ করতে `+` operator overload করি। এটা করি একটি `Point` struct-এর ওপর `Add` trait implement করে।

<Listing number="20-15" file-name="src/main.rs" caption="`Point` instance-এর জন্য `+` operator overload করতে `Add` trait implement করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-15/src/main.rs}}
```

</Listing>

`add` method দুটি `Point` instance-এর `x` মান আর দুটি `Point` instance-এর `y` মান যোগ করে একটি নতুন `Point` তৈরি করে। `Add` trait-এর `Output` নামের একটি associated type আছে, যা ঠিক করে `add`
method থেকে কোন type return হবে।

এই কোডের default generic type `Add` trait-এর ভেতরে। এর সংজ্ঞা এই:

```rust
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

এই কোড মোটামুটি পরিচিত লাগার কথা: একটি method আর একটি associated type-যুক্ত একটি trait। নতুন অংশটি হলো `Rhs=Self`: এই syntax-কে বলা হয় _default
type parameters_ (default type parameter)। `Rhs` generic type parameter (“right-hand
side”, অর্থাৎ ডান পাশ-এর সংক্ষেপ) `add` method-এর `rhs` parameter-এর type সংজ্ঞায়িত করে। `Add` trait implement করার সময় `Rhs`-এর জন্য কোনো concrete type উল্লেখ না করলে `Rhs`-এর type default-ভাবে
`Self` হবে, যা সেই type, যার ওপর আমরা `Add` implement করছি।

`Point`-এর জন্য `Add` implement করার সময় আমরা `Rhs`-এর default ব্যবহার করেছিলাম, কারণ দুটি `Point` instance যোগ করতে চেয়েছিলাম। চলুন `Add` trait implement করার এমন একটি উদাহরণ দেখি, যেখানে
default ব্যবহার না করে `Rhs` type নিজের মতো সাজাতে চাই।

আমাদের দুটি struct আছে, `Millimeters` আর `Meters`, যা ভিন্ন ভিন্ন এককে মান রাখে। একটি বিদ্যমান type-কে আরেকটি struct-এ এই পাতলা মোড়ানোকে বলা হয় _newtype pattern_, যা [“newtype pattern দিয়ে বাইরের
trait implement করা”][newtype]<!-- ignore --> অংশে আরও বিস্তারিত বর্ণনা করব। আমরা millimeter-এর মান meter-এর মানের সঙ্গে যোগ করতে চাই এবং `Add`-এর implementation দিয়ে রূপান্তরটি সঠিকভাবে করাতে চাই।
`Rhs` হিসেবে `Meters` দিয়ে `Millimeters`-এর জন্য `Add` implement করতে পারি, যেমনটি Listing 20-16-এ দেখানো হয়েছে।

<Listing number="20-16" file-name="src/lib.rs" caption="`Millimeters` আর `Meters` যোগ করতে `Millimeters`-এর ওপর `Add` trait implement করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-16/src/lib.rs}}
```

</Listing>

`Millimeters` আর `Meters` যোগ করতে আমরা `Self`-এর default ব্যবহার না করে `Rhs` type parameter-এর মান বসাতে `impl Add<Meters>` উল্লেখ করি।

default type parameter আপনি দুটি প্রধান উপায়ে ব্যবহার করবেন:

1. বিদ্যমান কোড না ভেঙে একটি type বাড়াতে
2. এমন নির্দিষ্ট ক্ষেত্রে নিজের মতো সাজানোর সুযোগ দিতে, যা বেশিরভাগ ব্যবহারকারীর দরকার হবে না

standard library-র `Add` trait দ্বিতীয় উদ্দেশ্যের একটি উদাহরণ: সাধারণত আপনি একই ধরনের দুটি type যোগ করবেন, তবে `Add` trait এর বাইরেও নিজের মতো সাজানোর ক্ষমতা দেয়। `Add` trait-এর সংজ্ঞায় একটি default
type parameter ব্যবহার করার মানে বেশিরভাগ সময় বাড়তি parameter উল্লেখ করতে হয় না। অন্যভাবে বললে, implementation-এর কিছু গৎবাঁধা কোড লাগে না, ফলে trait ব্যবহার সহজ হয়।

প্রথম উদ্দেশ্যটি দ্বিতীয়টির মতোই, তবে উল্টো দিক থেকে: একটি বিদ্যমান trait-এ একটি type parameter যোগ করতে চাইলে একে একটি default দিতে পারেন, যাতে বিদ্যমান implementation-এর কোড না ভেঙেই trait-এর
কার্যক্ষমতা বাড়ানো যায়।

<!-- Old headings. Do not remove or links may break. -->

<a id="fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name"></a>
<a id="disambiguating-between-methods-with-the-same-name"></a>

### একই নামের method-এর মধ্যে দ্ব্যর্থতা দূর করা {#disambiguating-between-identically-named-methods}

Rust-এ এমন কিছু নেই যা একটি trait-কে অন্য trait-এর method-এর একই নামের method রাখতে বাধা দেয়, আর Rust আপনাকে একটি type-এর ওপর দুটি trait-ই implement করতেও বাধা দেয় না। trait-এর method-এর
একই নামে সরাসরি type-এর ওপর একটি method implement করাও সম্ভব।

একই নামের method call করার সময় Rust-কে বলে দিতে হবে কোনটি ব্যবহার করতে চান। Listing 20-17-এর কোডটি দেখুন, যেখানে আমরা দুটি trait, `Pilot` আর `Wizard`, সংজ্ঞায়িত করেছি, যাদের দুটিরই `fly` নামের
একটি method আছে। তারপর এমন একটি type `Human`-এর ওপর দুটি trait-ই implement করি, যার ওপর আগে থেকেই `fly` নামের একটি method implement করা আছে। প্রতিটি `fly` method ভিন্ন কিছু করে।

<Listing number="20-17" file-name="src/main.rs" caption="`fly` method-যুক্ত দুটি trait সংজ্ঞায়িত করে `Human` type-এর ওপর implement করা হয়েছে, আর সরাসরি `Human`-এর ওপর একটি `fly` method implement করা হয়েছে।">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-17/src/main.rs:here}}
```

</Listing>

`Human`-এর একটি instance-এর ওপর `fly` call করলে compiler default-ভাবে সেই method call করে, যা সরাসরি type-টির ওপর implement করা, যেমনটি Listing 20-18-এ দেখানো হয়েছে।

<Listing number="20-18" file-name="src/main.rs" caption="`Human`-এর একটি instance-এর ওপর `fly` call করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-18/src/main.rs:here}}
```

</Listing>

এই কোড চালালে print হবে `*waving arms furiously*`, যা দেখায় যে Rust সরাসরি `Human`-এর ওপর implement করা `fly` method call করেছে।

`Pilot` trait বা `Wizard` trait-এর `fly` method call করতে আমরা কোন `fly` method বোঝাচ্ছি তা উল্লেখ করতে আরও স্পষ্ট syntax ব্যবহার করতে হবে। Listing 20-19-এ এই syntax দেখানো হয়েছে।

<Listing number="20-19" file-name="src/main.rs" caption="কোন trait-এর `fly` method call করতে চাই তা উল্লেখ করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-19/src/main.rs:here}}
```

</Listing>

method-এর নামের আগে trait-এর নাম উল্লেখ করলে Rust-এর কাছে স্পষ্ট হয় আমরা `fly`-এর কোন implementation call করতে চাই। আমরা
`Human::fly(&person)`-ও লিখতে পারতাম, যা Listing 20-19-এ ব্যবহার করা `person.fly()`-এর সমতুল্য, তবে দ্ব্যর্থতা দূর করার দরকার না থাকলে এটি লিখতে একটু লম্বা।

এই কোড চালালে এই output print হয়:

```console
{{#include ../listings/ch20-advanced-features/listing-20-19/output.txt}}
```

`fly` method যেহেতু একটি `self` parameter নেয়, তাই যদি দুটি _type_ থাকত যারা দুজনেই একটি _trait_ implement করে, তাহলে `self`-এর type-এর ভিত্তিতে Rust বের করতে পারত একটি trait-এর কোন implementation
ব্যবহার করতে হবে।

তবে method নয় এমন associated function-এর `self` parameter থাকে না। যখন একাধিক type বা trait একই function নামে method নয় এমন function সংজ্ঞায়িত করে, তখন fully qualified syntax (পূর্ণ পরিচয়সহ
syntax) ব্যবহার না করলে Rust সবসময় জানে না আপনি কোন type বোঝাচ্ছেন। যেমন, Listing 20-20-এ আমরা এমন একটি পশু আশ্রয়কেন্দ্রের জন্য trait তৈরি করি, যা সব কুকুরছানার নাম রাখতে চায় Spot। আমরা method
নয় এমন একটি associated function `baby_name`-সহ একটি `Animal` trait বানাই। `Animal` trait implement করা হয় struct `Dog`-এর জন্য, যার ওপর আমরা সরাসরি method নয় এমন একটি associated function `baby_name`-ও
দিই।

<Listing number="20-20" file-name="src/main.rs" caption="একটি associated function-যুক্ত একটি trait, আর একই নামের একটি associated function-যুক্ত একটি type, যা trait-টিও implement করে">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-20/src/main.rs}}
```

</Listing>

সব কুকুরছানার নাম Spot রাখার কোড আমরা `Dog`-এর ওপর সংজ্ঞায়িত `baby_name` associated function-এ implement করি। `Dog` type `Animal` trait-ও implement করে, যা সব পশুর সাধারণ বৈশিষ্ট্য বর্ণনা করে।
কুকুরের বাচ্চাকে বলা হয় puppy, আর তা প্রকাশ পায় `Dog`-এর ওপর `Animal` trait-এর implementation-এ, `Animal` trait-এর সঙ্গে যুক্ত `baby_name` function-এ।

`main`-এ আমরা `Dog::baby_name` function call করি, যা সরাসরি `Dog`-এর ওপর সংজ্ঞায়িত associated function call করে। এই কোড এই output print করে:

```console
{{#include ../listings/ch20-advanced-features/listing-20-20/output.txt}}
```

এই output আমরা চাইনি। আমরা চাই `Dog`-এর ওপর implement করা `Animal` trait-এর অংশ `baby_name` function call করতে, যাতে কোড print করে `A baby dog is called a puppy`। Listing 20-19-এ ব্যবহার
করা trait-এর নাম উল্লেখের কৌশল এখানে সাহায্য করে না; `main` বদলে Listing 20-21-এর কোড করলে compilation error পাব।

<Listing number="20-21" file-name="src/main.rs" caption="`Animal` trait থেকে `baby_name` function call করার চেষ্টা, কিন্তু কোন implementation ব্যবহার করতে হবে Rust তা জানে না">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-21/src/main.rs:here}}
```

</Listing>

`Animal::baby_name`-এর যেহেতু কোনো `self` parameter নেই, এবং `Animal` trait implement করা অন্য type-ও থাকতে পারে, তাই আমরা `Animal::baby_name`-এর কোন implementation চাই তা Rust বের করতে পারে না।
আমরা এই compiler error পাব:

```console
{{#include ../listings/ch20-advanced-features/listing-20-21/output.txt}}
```

দ্ব্যর্থতা দূর করে Rust-কে বলতে যে আমরা অন্য কোনো type-এর জন্য `Animal`-এর implementation নয়, বরং `Dog`-এর জন্য `Animal`-এর implementation ব্যবহার করতে চাই, fully qualified syntax ব্যবহার করতে হবে।
Listing 20-22-এ দেখানো হয়েছে কীভাবে fully qualified syntax ব্যবহার করতে হয়।

<Listing number="20-22" file-name="src/main.rs" caption="`Dog`-এর ওপর implement করা `Animal` trait-এর `baby_name` function call করতে চাই তা উল্লেখ করতে fully qualified syntax ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-22/src/main.rs:here}}
```

</Listing>

আমরা angle bracket-এর ভেতরে Rust-কে একটি type annotation দিচ্ছি, যা বোঝায় যে এই function call-এর জন্য `Dog` type-কে একটি `Animal` হিসেবে গণ্য করতে চাই বলে আমরা `Dog`-এর ওপর implement করা `Animal`
trait-এর `baby_name` method call করতে চাই। এই কোড এখন আমাদের কাঙ্ক্ষিত output print করবে:

```console
{{#include ../listings/ch20-advanced-features/listing-20-22/output.txt}}
```

সাধারণভাবে fully qualified syntax এভাবে সংজ্ঞায়িত:

```rust,ignore
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```

method নয় এমন associated function-এর ক্ষেত্রে কোনো `receiver` থাকবে না: শুধু অন্য argument-গুলোর তালিকা থাকবে। function বা method call করার সব জায়গায় fully qualified syntax ব্যবহার করতে পারতেন। তবে
প্রোগ্রামের অন্য তথ্য থেকে Rust এই syntax-এর যে অংশ বের করতে পারে, তা বাদ দেওয়ার অনুমতি আছে। শুধু সেসব ক্ষেত্রেই এই বেশি শব্দবহুল syntax ব্যবহার করতে হয়, যেখানে একই নাম ব্যবহার করা একাধিক
implementation আছে এবং আপনি কোন implementation call করতে চান তা শনাক্ত করতে Rust-এর সাহায্য দরকার।

<!-- Old headings. Do not remove or links may break. -->

<a id="using-supertraits-to-require-one-traits-functionality-within-another-trait"></a>

### supertrait ব্যবহার করা {#using-supertraits}

কখনো কখনো আপনি এমন একটি trait-এর সংজ্ঞা লিখতে পারেন, যা আরেকটি trait-এর ওপর নির্ভর করে: একটি type-কে প্রথম trait implement করতে হলে আপনি চান সেই type দ্বিতীয় trait-ও implement করুক। এটা করবেন যাতে
আপনার trait-এর সংজ্ঞা দ্বিতীয় trait-এর associated item ব্যবহার করতে পারে। আপনার trait-এর সংজ্ঞা যে trait-এর ওপর নির্ভর করছে তাকে বলা হয় আপনার trait-এর _supertrait_ (ঊর্ধ্বতন trait)।

যেমন ধরুন, আমরা একটি `outline_print` method-যুক্ত `OutlinePrint` trait বানাতে চাই, যা একটি প্রদত্ত মান তারকাচিহ্নের ফ্রেমে বাঁধিয়ে print করবে। অর্থাৎ, standard library-র `Display` trait implement করে
`(x, y)` দেয় এমন একটি `Point` struct দেওয়া থাকলে, `x`-এ `1` আর `y`-এ `3` থাকা একটি `Point` instance-এর ওপর `outline_print` call করলে এটি এই output print করবে:

```text
**********
*        *
* (1, 3) *
*        *
**********
```

`outline_print` method-এর implementation-এ আমরা `Display` trait-এর কার্যক্ষমতা ব্যবহার করতে চাই। তাই বলে দিতে হবে যে `OutlinePrint` trait কেবল সেই type-গুলোর জন্য কাজ করবে, যা `Display`-ও implement
করে এবং `OutlinePrint`-এর দরকারি কার্যক্ষমতা দেয়। trait-এর সংজ্ঞায় `OutlinePrint: Display` উল্লেখ করে তা করা যায়। এই কৌশল trait-এ একটি trait bound যোগ করার মতো। Listing 20-23-এ `OutlinePrint`
trait-এর একটি implementation দেখানো হয়েছে।

<Listing number="20-23" file-name="src/main.rs" caption="`Display`-এর কার্যক্ষমতা দরকার এমন `OutlinePrint` trait implement করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-23/src/main.rs:here}}
```

</Listing>

আমরা যেহেতু উল্লেখ করেছি যে `OutlinePrint`-এর `Display` trait দরকার, তাই `Display` implement করা যেকোনো type-এর জন্য স্বয়ংক্রিয়ভাবে implement হওয়া `to_string` function ব্যবহার করতে পারি। trait-এর
নামের পরে colon দিয়ে `Display` trait উল্লেখ না করে `to_string` ব্যবহারের চেষ্টা করলে error পেতাম, যা বলত বর্তমান scope-এ `&Self` type-এর জন্য `to_string` নামের কোনো method পাওয়া যায়নি।

চলুন দেখি `Display` implement করে না এমন একটি type-এর ওপর, যেমন `Point` struct, `OutlinePrint` implement করার চেষ্টা করলে কী ঘটে:

<Listing file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-02-impl-outlineprint-for-point/src/main.rs:here}}
```

</Listing>

আমরা একটি error পাই, যা বলে `Display` দরকার কিন্তু implement করা নেই:

```console
{{#include ../listings/ch20-advanced-features/no-listing-02-impl-outlineprint-for-point/output.txt}}
```

এটি ঠিক করতে আমরা `Point`-এর ওপর `Display` implement করি এবং `OutlinePrint`-এর দরকারি শর্ত পূরণ করি, এভাবে:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-03-impl-display-for-point/src/main.rs:here}}
```

</Listing>

তারপর `Point`-এর ওপর `OutlinePrint` trait implement করা সফলভাবে compile হবে, আর একটি `Point` instance-এর ওপর `outline_print` call করে একে তারকাচিহ্নের ফ্রেমের ভেতরে দেখাতে পারি।

<!-- Old headings. Do not remove or links may break. -->

<a id="using-the-newtype-pattern-to-implement-external-traits-on-external-types"></a>
<a id="using-the-newtype-pattern-to-implement-external-traits"></a>

### newtype pattern দিয়ে বাইরের trait implement করা {#implementing-external-traits-with-the-newtype-pattern}

অধ্যায় 10-এর [“একটি type-এর ওপর trait implement করা”][implementing-a-trait-on-a-type]<!--
ignore --> অংশে আমরা orphan rule-এর কথা বলেছিলাম, যা বলে একটি type-এর ওপর trait implement করা যায় কেবল তখনই, যখন trait অথবা type, কিংবা দুটোই, আমাদের crate-এর নিজস্ব। newtype pattern ব্যবহার করে এই
সীমাবদ্ধতা এড়ানো সম্ভব, যার মধ্যে আছে একটি tuple struct-এ একটি নতুন type তৈরি করা। (tuple struct নিয়ে অধ্যায় 5-এর [“tuple struct দিয়ে আলাদা type তৈরি
করা”][tuple-structs]<!-- ignore --> অংশে আলোচনা করেছি।) tuple struct-টির একটি field থাকবে এবং এটি সেই type-এর চারপাশে একটি পাতলা মোড়ক হবে, যার জন্য আমরা একটি trait implement করতে চাই। তখন মোড়ক
type-টি আমাদের crate-এর নিজস্ব, আর মোড়কটির ওপর trait implement করা যায়। _Newtype_ পরিভাষাটি এসেছে Haskell প্রোগ্রামিং ভাষা থেকে। এই pattern ব্যবহারের জন্য runtime-এ performance-এর কোনো মাশুল নেই,
আর compile time-এ মোড়ক type-টি বাদ পড়ে যায়।

উদাহরণ হিসেবে ধরুন আমরা `Vec<T>`-এর ওপর `Display` implement করতে চাই, যা orphan rule সরাসরি করতে দেয় না, কারণ `Display` trait আর `Vec<T>` type আমাদের crate-এর বাইরে সংজ্ঞায়িত। আমরা একটি
`Wrapper` struct বানাতে পারি, যা `Vec<T>`-এর একটি instance রাখে; তারপর `Wrapper`-এর ওপর `Display` implement করে `Vec<T>` মান ব্যবহার করতে পারি, যেমনটি Listing 20-24-এ দেখানো হয়েছে।

<Listing number="20-24" file-name="src/main.rs" caption="`Display` implement করতে `Vec<String>`-এর চারপাশে একটি `Wrapper` type তৈরি করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-24/src/main.rs}}
```

</Listing>

`Display`-এর implementation ভেতরের `Vec<T>` পেতে `self.0` ব্যবহার করে, কারণ `Wrapper` একটি tuple struct, আর `Vec<T>` হলো tuple-এর index 0-এর item। তারপর `Wrapper`-এর ওপর `Display` trait-এর
কার্যক্ষমতা ব্যবহার করতে পারি।

এই কৌশল ব্যবহারের অসুবিধা হলো `Wrapper` একটি নতুন type, তাই এর ভেতরে রাখা মানের method-গুলো এতে নেই। `Vec<T>`-এর সব method সরাসরি `Wrapper`-এর ওপর implement করতে হতো, যাতে method-গুলো `self.0`-এর
হাতে কাজ তুলে দেয়, তাহলে `Wrapper`-কে হুবহু একটি `Vec<T>`-এর মতো ব্যবহার করা যেত। আমরা যদি চাইতাম নতুন type-টিতে ভেতরের type-এর প্রতিটি method থাকুক, তাহলে ভেতরের type return করতে `Wrapper`-এর
ওপর `Deref` trait implement করা একটি সমাধান হতো (অধ্যায় 15-এর [“smart pointer-কে সাধারণ reference-এর মতো ব্যবহার
করা”][smart-pointer-deref]<!-- ignore --> অংশে `Deref` trait implement করা নিয়ে আলোচনা করেছি)। আর যদি না চাইতাম যে `Wrapper` type-এ ভেতরের type-এর সব method থাকুক—যেমন, `Wrapper` type-এর আচরণ
সীমিত রাখতে—তাহলে শুধু যে method-গুলো চাই সেগুলো হাতে implement করতে হতো।

trait জড়িত না থাকলেও এই newtype pattern কাজে লাগে। চলুন মনোযোগ বদলে Rust-এর type system-এর সঙ্গে কাজ করার কিছু উন্নত উপায় দেখি।

{{#quiz ../quizzes/ch19-03-advanced-traits.toml}}

[newtype]: ch20-02-advanced-traits.html#implementing-external-traits-with-the-newtype-pattern
[implementing-a-trait-on-a-type]: ch10-02-traits.html#implementing-a-trait-on-a-type
[traits]: ch10-02-traits.html
[smart-pointer-deref]: ch15-02-deref.html#treating-smart-pointers-like-regular-references
[tuple-structs]: ch05-01-defining-structs.html#creating-different-types-with-tuple-structs
