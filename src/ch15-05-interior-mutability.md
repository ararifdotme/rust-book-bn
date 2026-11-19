## `RefCell<T>` ও interior mutability pattern {#refcellt-and-the-interior-mutability-pattern}

_Interior mutability_ (ভেতরের মান পরিবর্তনযোগ্যতা) Rust-এর একটি design pattern, যা ডেটার immutable reference থাকা অবস্থাতেও ডেটা পরিবর্তন করতে দেয়; সাধারণত
borrowing-এর নিয়ম এই কাজ অনুমোদন করে না। ডেটা পরিবর্তন করতে এই pattern একটি data structure-এর ভেতরে `unsafe` কোড ব্যবহার করে পরিবর্তন আর borrowing নিয়ন্ত্রণ করা
Rust-এর সাধারণ নিয়মগুলো একটু বাঁকিয়ে নেয়। unsafe কোড compiler-কে জানায় যে compiler-এর ওপর নির্ভর না করে আমরা নিজেরাই হাতে নিয়মগুলো যাচাই করছি; unsafe কোড নিয়ে
অধ্যায় 20-এ আরও আলোচনা করব।

interior mutability pattern ব্যবহার করা type আমরা কেবল তখনই ব্যবহার করতে পারি, যখন নিশ্চিত করতে পারি যে runtime-এ borrowing-এর নিয়ম মানা হবে, যদিও compiler তার
নিশ্চয়তা দিতে পারে না। জড়িত `unsafe` কোড তখন একটি নিরাপদ API-তে মুড়ে রাখা হয়, আর বাইরের type তখনো immutable থাকে।

interior mutability pattern মেনে চলা `RefCell<T>` type দেখে চলুন ধারণাটি বুঝি।

<!-- Old headings. Do not remove or links may break. -->

<a id="enforcing-borrowing-rules-at-runtime-with-refcellt"></a>

### runtime-এ borrowing-এর নিয়ম প্রয়োগ করা {#enforcing-borrowing-rules-at-runtime}

`Rc<T>`-এর মতো নয়, `RefCell<T>` type তার রাখা ডেটার ওপর একক ownership বোঝায়। তাহলে `Box<T>`-এর মতো type থেকে `RefCell<T>`-কে আলাদা করে কী? অধ্যায় 4-এ শেখা
borrowing-এর নিয়মগুলো মনে করুন:

- যেকোনো সময়ে আপনার কাছে _হয়_ একটি mutable reference _নয়তো_ যেকোনো সংখ্যক immutable reference থাকতে পারে (দুটি একসঙ্গে নয়)।
- reference সবসময় বৈধ থাকতে হবে।

reference আর `Box<T>`-এর ক্ষেত্রে borrowing-এর নিয়মের invariant-গুলো compile time-এ প্রয়োগ করা হয়। `RefCell<T>`-এর ক্ষেত্রে এই invariant-গুলো প্রয়োগ করা হয় _runtime-এ_।
reference-এর ক্ষেত্রে এই নিয়ম ভাঙলে compiler error পাবেন। `RefCell<T>`-এর ক্ষেত্রে এই নিয়ম ভাঙলে আপনার প্রোগ্রাম panic করে বন্ধ হয়ে যাবে।

compile time-এ borrowing-এর নিয়ম যাচাইয়ের সুবিধা হলো development প্রক্রিয়ায় error আগেই ধরা পড়ে, আর runtime-এর performance-এ কোনো প্রভাব পড়ে না, কারণ সব বিশ্লেষণ
আগেই শেষ হয়ে যায়। এসব কারণে বেশিরভাগ ক্ষেত্রে compile time-এ borrowing-এর নিয়ম যাচাই করাই সেরা পছন্দ, আর এ কারণেই এটি Rust-এর default।

এর বদলে runtime-এ borrowing-এর নিয়ম যাচাইয়ের সুবিধা হলো, কিছু মেমোরি-নিরাপদ পরিস্থিতি তখন অনুমোদিত হয়, যা compile time-এর যাচাইয়ে বাতিল হয়ে যেত। Rust compiler-এর
মতো static analysis (কোড না চালিয়ে বিশ্লেষণ) স্বভাবতই রক্ষণশীল। কোডের কিছু বৈশিষ্ট্য কোড বিশ্লেষণ করে শনাক্ত করা অসম্ভব: সবচেয়ে বিখ্যাত উদাহরণ হলো Halting Problem,
যা এই বইয়ের আওতার বাইরে, তবে খুঁজে পড়ার মতো একটি মজার বিষয়।

কিছু বিশ্লেষণ অসম্ভব বলে, কোড ownership-এর নিয়ম মানে কি না তা Rust compiler নিশ্চিত হতে না পারলে এটি একটি সঠিক প্রোগ্রামও বাতিল করতে পারে; এভাবেই এটি রক্ষণশীল।
Rust যদি একটি ভুল প্রোগ্রাম গ্রহণ করত, তাহলে ব্যবহারকারীরা Rust-এর দেওয়া নিশ্চয়তায় ভরসা রাখতে পারত না। কিন্তু Rust একটি সঠিক প্রোগ্রাম বাতিল করলে প্রোগ্রামারের
অসুবিধা হবে, তবে বিপর্যয়কর কিছু ঘটবে না। `RefCell<T>` type কাজে লাগে যখন আপনি নিশ্চিত যে আপনার কোড borrowing-এর নিয়ম মানছে, কিন্তু compiler তা বুঝতে ও নিশ্চিত
করতে পারছে না।

`Rc<T>`-এর মতোই `RefCell<T>` শুধু single-threaded পরিস্থিতিতে ব্যবহারের জন্য, এবং multithreaded প্রেক্ষাপটে ব্যবহারের চেষ্টা করলে compile-time error দেবে। multithreaded
প্রোগ্রামে কীভাবে `RefCell<T>`-এর কার্যক্ষমতা পাওয়া যায়, তা নিয়ে অধ্যায় 16-এ কথা বলব।

`Box<T>`, `Rc<T>` বা `RefCell<T>` বেছে নেওয়ার কারণগুলোর সারসংক্ষেপ:

- `Rc<T>` একই ডেটার একাধিক owner সম্ভব করে; `Box<T>` আর `RefCell<T>`-এর owner একজনই।
- `Box<T>` compile time-এ যাচাই হওয়া immutable বা mutable borrow অনুমোদন করে; `Rc<T>` শুধু compile time-এ যাচাই হওয়া immutable borrow অনুমোদন করে; `RefCell<T>`
  runtime-এ যাচাই হওয়া immutable বা mutable borrow অনুমোদন করে।
- `RefCell<T>` যেহেতু runtime-এ যাচাই হওয়া mutable borrow অনুমোদন করে, তাই `RefCell<T>` immutable হলেও `RefCell<T>`-এর ভেতরের মান পরিবর্তন করা যায়।

একটি immutable মানের ভেতরের মান পরিবর্তন করাই হলো interior mutability pattern। চলুন এমন একটি পরিস্থিতি দেখি যেখানে interior mutability কাজে লাগে, এবং দেখি এটি কীভাবে
সম্ভব।

<!-- Old headings. Do not remove or links may break. -->

<a id="interior-mutability-a-mutable-borrow-to-an-immutable-value"></a>

### interior mutability ব্যবহার করা {#using-interior-mutability}

borrowing-এর নিয়মের একটি ফল হলো, আপনার কাছে একটি immutable মান থাকলে সেটি mutable-ভাবে borrow করা যায় না। যেমন, এই কোড compile হবে না:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/no-listing-01-cant-borrow-immutable-as-mutable/src/main.rs}}
```

এই কোড compile করার চেষ্টা করলে এই error পেতেন:

```console
{{#include ../listings/ch15-smart-pointers/no-listing-01-cant-borrow-immutable-as-mutable/output.txt}}
```

তবে এমন পরিস্থিতিও আছে যেখানে একটি মান তার method-এর ভেতরে নিজেকে পরিবর্তন করতে পারলে কিন্তু অন্য কোডের কাছে immutable দেখালে কাজে লাগত। মানটির method-এর বাইরের
কোড মানটি পরিবর্তন করতে পারত না। `RefCell<T>` ব্যবহার করা interior mutability পাওয়ার একটি উপায়, তবে `RefCell<T>` borrowing-এর নিয়ম পুরোপুরি এড়িয়ে যায় না:
compiler-এর borrow checker এই interior mutability অনুমোদন করে, আর borrowing-এর নিয়ম যাচাই হয় runtime-এ। নিয়ম ভাঙলে compiler error-এর বদলে একটি `panic!` পাবেন।

চলুন একটি ব্যবহারিক উদাহরণ নিয়ে কাজ করি, যেখানে একটি immutable মান পরিবর্তন করতে `RefCell<T>` ব্যবহার করা যায়, এবং দেখি কেন তা কাজে লাগে।

<!-- Old headings. Do not remove or links may break. -->

<a id="a-use-case-for-interior-mutability-mock-objects"></a>

#### mock object দিয়ে test করা {#testing-with-mock-objects}

কখনো কখনো test করার সময় প্রোগ্রামার নির্দিষ্ট কোনো আচরণ পর্যবেক্ষণ করতে এবং তা সঠিকভাবে implement করা হয়েছে কি না assert করতে এক type-এর জায়গায় আরেকটি type
ব্যবহার করেন। এই বদলি type-কে বলা হয় _test double_। চলচ্চিত্রের stunt double-এর কথা ভাবুন, যেখানে বিশেষ কঠিন একটি দৃশ্যে একজন অভিনেতার বদলে অন্য একজন কাজ করেন।
test চালানোর সময় test double অন্য type-এর জায়গায় দাঁড়ায়। _Mock objects_ (mock object) হলো নির্দিষ্ট ধরনের test double, যা test চলার সময় কী ঘটছে তা লিখে রাখে,
যাতে আপনি assert করতে পারেন যে সঠিক কাজগুলো ঘটেছে।

অন্য ভাষায় যে অর্থে object থাকে, Rust-এ সেই অর্থে object নেই, আর কিছু ভাষার মতো Rust-এর standard library-তে mock object-এর কার্যক্ষমতা built-in নেই। তবে mock object-এর
মতো একই উদ্দেশ্য পূরণ করবে এমন struct আপনি অবশ্যই তৈরি করতে পারেন।

আমরা যে পরিস্থিতি test করব তা এই: আমরা এমন একটি library তৈরি করব যা একটি সর্বোচ্চ মানের বিপরীতে একটি মানের হিসাব রাখে, এবং বর্তমান মান সর্বোচ্চ মানের কতটা কাছে
তার ওপর ভিত্তি করে message পাঠায়। যেমন, একজন ব্যবহারকারী কতগুলো API call করতে পারবেন, তার quota-র হিসাব রাখতে এই library ব্যবহার করা যেতে পারে।

আমাদের library শুধু একটি মান সর্বোচ্চের কতটা কাছে আর কখন কোন message হওয়া উচিত, তার হিসাব রাখার কার্যক্ষমতা দেবে। আমাদের library ব্যবহার করা application-গুলো message
পাঠানোর ব্যবস্থা দেবে বলে আশা করা হয়: application সরাসরি ব্যবহারকারীকে message দেখাতে পারে, email পাঠাতে পারে, text message পাঠাতে পারে, বা অন্য কিছু করতে পারে।
library-র এই খুঁটিনাটি জানার দরকার নেই। এর শুধু দরকার এমন কিছু, যা আমাদের দেওয়া `Messenger` নামের একটি trait implement করে। Listing 15-20-এ library-র কোড দেখানো
হয়েছে।

<Listing number="15-20" file-name="src/lib.rs" caption="একটি মান সর্বোচ্চ মানের কতটা কাছে তার হিসাব রাখা এবং মান নির্দিষ্ট স্তরে পৌঁছালে সতর্ক করার একটি library">

```rust,noplayground
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-20/src/lib.rs}}
```

</Listing>

এই কোডের একটি গুরুত্বপূর্ণ অংশ হলো, `Messenger` trait-এর `send` নামের একটি method আছে, যা `self`-এর একটি immutable reference আর message-এর লেখা নেয়। এই trait-টিই
সেই interface, যা আমাদের mock object-কে implement করতে হবে, যাতে mock-টি একটি আসল object-এর মতোই ব্যবহার করা যায়। অন্য গুরুত্বপূর্ণ অংশ হলো, আমরা `LimitTracker`-এর
`set_value` method-এর আচরণ test করতে চাই। `value` parameter-এ কী পাঠাচ্ছি তা বদলাতে পারি, কিন্তু `set_value` এমন কিছু return করে না যার ওপর assertion করা যায়। আমরা
বলতে চাই যে `Messenger` trait implement করে এমন কিছু আর `max`-এর একটি নির্দিষ্ট মান দিয়ে একটি `LimitTracker` তৈরি করলে, `value`-এর জন্য ভিন্ন ভিন্ন সংখ্যা পাঠালে
messenger-কে উপযুক্ত message পাঠাতে বলা হয়।

আমাদের এমন একটি mock object দরকার, যা `send` call করলে email বা text message না পাঠিয়ে শুধু যে message-গুলো পাঠাতে বলা হয়েছে সেগুলোর হিসাব রাখবে। mock object-এর
একটি নতুন instance তৈরি করে, mock object ব্যবহার করা একটি `LimitTracker` তৈরি করে, `LimitTracker`-এর ওপর `set_value` method call করে, তারপর যাচাই করতে পারি যে mock
object-এ প্রত্যাশিত message-গুলো আছে কি না। Listing 15-21-এ ঠিক এটা করতে একটি mock object implement করার চেষ্টা দেখানো হয়েছে, কিন্তু borrow checker তা অনুমোদন করবে
না।

<Listing number="15-21" file-name="src/lib.rs" caption="borrow checker অনুমোদন করে না এমন একটি `MockMessenger` implement করার চেষ্টা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-21/src/lib.rs:here}}
```

</Listing>

এই test কোড একটি `MockMessenger` struct সংজ্ঞায়িত করে, যার `sent_messages` field-এ `String` মানের একটি `Vec` থাকে, যা পাঠাতে বলা message-গুলোর হিসাব রাখে। আমরা `new`
নামের একটি associated function-ও সংজ্ঞায়িত করি, যাতে খালি message তালিকা দিয়ে শুরু হওয়া নতুন `MockMessenger` মান সহজে তৈরি করা যায়। তারপর `MockMessenger`-এর জন্য
`Messenger` trait implement করি, যাতে একটি `LimitTracker`-কে একটি `MockMessenger` দিতে পারি। `send` method-এর সংজ্ঞায় parameter হিসেবে পাঠানো message নিয়ে সেটি
`MockMessenger`-এর `sent_messages` তালিকায় রাখি।

test-এ আমরা পরীক্ষা করছি, `LimitTracker`-কে `value`-এর মান `max` মানের 75 শতাংশের বেশি কিছুতে বসাতে বললে কী ঘটে। প্রথমে একটি নতুন `MockMessenger` তৈরি করি, যা খালি
message তালিকা দিয়ে শুরু হবে। তারপর একটি নতুন `LimitTracker` তৈরি করে তাকে নতুন `MockMessenger`-এর একটি reference আর `max` মান `100` দিই। `LimitTracker`-এর ওপর `80`
মান দিয়ে `set_value` method call করি, যা 100-এর 75 শতাংশের বেশি। তারপর assert করি যে `MockMessenger` যে message তালিকার হিসাব রাখছে তাতে এখন একটি message থাকা উচিত।

কিন্তু এই test-এ একটি সমস্যা আছে, যেমনটি এখানে দেখানো হয়েছে:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-21/output.txt}}
```

message-এর হিসাব রাখতে আমরা `MockMessenger` পরিবর্তন করতে পারি না, কারণ `send` method `self`-এর একটি immutable reference নেয়। error-এর লেখার পরামর্শ মেনে `impl`
method আর trait-এর সংজ্ঞা দুটিতেই `&mut self` ব্যবহারও করতে পারি না। শুধু test-এর খাতিরে আমরা `Messenger` trait বদলাতে চাই না। বরং বিদ্যমান নকশার সঙ্গেই আমাদের test
কোড ঠিকমতো কাজ করানোর একটি উপায় খুঁজতে হবে।

এটি এমন একটি পরিস্থিতি, যেখানে interior mutability সাহায্য করতে পারে! আমরা `sent_messages` একটি `RefCell<T>`-এর ভেতরে রাখব, তাহলে `send` method দেখা message-গুলো
রাখতে `sent_messages` পরিবর্তন করতে পারবে। Listing 15-22-এ দেখানো হয়েছে এটি দেখতে কেমন।

<Listing number="15-22" file-name="src/lib.rs" caption="বাইরের মান immutable গণ্য হওয়া অবস্থায় ভেতরের মান পরিবর্তন করতে `RefCell<T>` ব্যবহার করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-22/src/lib.rs:here}}
```

</Listing>

`sent_messages` field-এর type এখন `Vec<String>`-এর বদলে `RefCell<Vec<String>>`। `new` function-এ আমরা খালি vector-টির চারপাশে একটি নতুন `RefCell<Vec<String>>`
instance তৈরি করি।

`send` method-এর implementation-এ প্রথম parameter এখনো `self`-এর একটি immutable borrow, যা trait-এর সংজ্ঞার সঙ্গে মেলে। `self.sent_messages`-এর `RefCell<Vec<String>>`-এর
ওপর `borrow_mut` call করে আমরা `RefCell<Vec<String>>`-এর ভেতরের মান, অর্থাৎ vector-টির একটি mutable reference পাই। তারপর test চলার সময় পাঠানো message-গুলোর হিসাব
রাখতে vector-এর mutable reference-এর ওপর `push` call করতে পারি।

শেষ যে পরিবর্তনটি করতে হবে তা assertion-এ: ভেতরের vector-এ কতগুলো item আছে তা দেখতে আমরা `RefCell<Vec<String>>`-এর ওপর `borrow` call করে vector-এর একটি immutable
reference পাই।

`RefCell<T>` কীভাবে ব্যবহার করতে হয় তা দেখলেন, এবার চলুন ভেতরে ঢুকে দেখি এটি কীভাবে কাজ করে!

<!-- Old headings. Do not remove or links may break. -->

<a id="keeping-track-of-borrows-at-runtime-with-refcellt"></a>

#### runtime-এ borrow-এর হিসাব রাখা {#tracking-borrows-at-runtime}

immutable আর mutable reference তৈরির সময় আমরা যথাক্রমে `&` আর `&mut` syntax ব্যবহার করি। `RefCell<T>`-এর ক্ষেত্রে আমরা `borrow` আর `borrow_mut` method ব্যবহার করি,
যা `RefCell<T>`-এর নিরাপদ API-র অংশ। `borrow` method smart pointer type `Ref<T>` return করে, আর `borrow_mut` return করে smart pointer type `RefMut<T>`। দুটি type-ই
`Deref` implement করে, তাই এদের সাধারণ reference-এর মতো ব্যবহার করা যায়।

`RefCell<T>` হিসাব রাখে এই মুহূর্তে কতগুলো `Ref<T>` আর `RefMut<T>` smart pointer সক্রিয় আছে। প্রতিবার `borrow` call করলে `RefCell<T>` সক্রিয় immutable borrow-এর
count বাড়ায়। একটি `Ref<T>` মান scope-এর বাইরে গেলে immutable borrow-এর count 1 কমে যায়। compile time-এর borrowing নিয়মের মতোই `RefCell<T>` যেকোনো মুহূর্তে আমাদের
অনেকগুলো immutable borrow অথবা একটি mutable borrow রাখতে দেয়।

এই নিয়মগুলো ভাঙার চেষ্টা করলে reference-এর ক্ষেত্রে যেমন compiler error পেতাম, তেমন না পেয়ে `RefCell<T>`-এর implementation runtime-এ panic করবে। Listing 15-23-এ
Listing 15-22-এর `send`-এর implementation-এর একটি পরিবর্তন দেখানো হয়েছে। `RefCell<T>` যে runtime-এ আমাদের এটি করতে দেয় না তা দেখাতে আমরা ইচ্ছে করে একই scope-এ
দুটি সক্রিয় mutable borrow তৈরির চেষ্টা করছি।

<Listing number="15-23" file-name="src/lib.rs" caption="`RefCell<T>` যে panic করবে তা দেখতে একই scope-এ দুটি mutable reference তৈরি করা">

```rust,ignore,panics
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-23/src/lib.rs:here}}
```

</Listing>

`borrow_mut` থেকে return হওয়া `RefMut<T>` smart pointer-এর জন্য আমরা `one_borrow` variable তৈরি করি। তারপর একইভাবে `two_borrow` variable-এ আরেকটি mutable borrow তৈরি
করি। এতে একই scope-এ দুটি mutable reference হয়, যা অনুমোদিত নয়। আমাদের library-র test চালালে Listing
15-23-এর কোড কোনো error ছাড়াই compile হবে, কিন্তু test fail করবে:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-23/output.txt}}
```

খেয়াল করুন, কোডটি `already borrowed:
BorrowMutError` message দিয়ে panic করেছে। runtime-এ borrowing-এর নিয়ম ভঙ্গ `RefCell<T>` এভাবেই সামলায়।

এখানে যেমন করেছি, borrowing error compile time-এর বদলে runtime-এ ধরার পথ বেছে নেওয়ার মানে হলো, আপনি হয়তো development প্রক্রিয়ার পরের দিকে কোডের ভুল খুঁজে পাবেন:
হয়তো কোড production-এ deploy হওয়ার আগে পর্যন্ত নয়। তা ছাড়া compile time-এর বদলে runtime-এ borrow-এর হিসাব রাখার ফলে আপনার কোডের runtime performance-এ সামান্য মাশুল
দিতে হবে। তবে `RefCell<T>` ব্যবহার করলে এমন একটি mock object লেখা সম্ভব হয়, যা শুধু immutable মান অনুমোদিত এমন প্রেক্ষাপটে ব্যবহারের সময়ও দেখা message-গুলোর হিসাব
রাখতে নিজেকে পরিবর্তন করতে পারে। সুবিধা-অসুবিধা সত্ত্বেও সাধারণ reference-এর চেয়ে বেশি কার্যক্ষমতা পেতে আপনি `RefCell<T>` ব্যবহার করতে পারেন।

<!-- Old headings. Do not remove or links may break. -->

<a id="having-multiple-owners-of-mutable-data-by-combining-rc-t-and-ref-cell-t"></a>
<a id="allowing-multiple-owners-of-mutable-data-with-rct-and-refcellt"></a>

### পরিবর্তনযোগ্য ডেটার একাধিক owner রাখা {#allowing-multiple-owners-of-mutable-data}

`RefCell<T>` ব্যবহারের একটি প্রচলিত উপায় হলো `Rc<T>`-এর সঙ্গে মিলিয়ে ব্যবহার করা। মনে করুন, `Rc<T>` কিছু ডেটার একাধিক owner রাখতে দেয়, কিন্তু সেই ডেটায় শুধু immutable
প্রবেশাধিকার দেয়। আপনার কাছে যদি এমন একটি `Rc<T>` থাকে যা একটি `RefCell<T>` রাখে, তাহলে এমন একটি মান পান যার একাধিক owner থাকতে পারে _এবং_ যা আপনি পরিবর্তন করতে
পারেন!

যেমন, Listing 15-18-এর cons list উদাহরণটি মনে করুন, যেখানে একাধিক তালিকাকে আরেকটি তালিকার ownership ভাগ করতে দিতে আমরা `Rc<T>` ব্যবহার করেছিলাম। `Rc<T>` যেহেতু
শুধু immutable মান রাখে, তাই তালিকার মানগুলো একবার তৈরি করার পর আর কোনোটি বদলাতে পারি না। তালিকার মান বদলাতে পারার ক্ষমতার জন্য চলুন `RefCell<T>` যোগ করি। Listing 15-24
দেখায় যে `Cons`-এর সংজ্ঞায় একটি `RefCell<T>` ব্যবহার করে সব তালিকায় রাখা মান পরিবর্তন করা যায়।

<Listing number="15-24" file-name="src/main.rs" caption="পরিবর্তন করা যায় এমন একটি `List` তৈরি করতে `Rc<RefCell<i32>>` ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-24/src/main.rs}}
```

</Listing>

আমরা `Rc<RefCell<i32>>`-এর একটি instance হিসেবে একটি মান তৈরি করে `value` নামের একটি variable-এ রাখি, যাতে পরে সরাসরি এটি পেতে পারি। তারপর `a`-তে এমন একটি `Cons`
variant দিয়ে একটি `List` তৈরি করি, যা `value` রাখে। `value` clone করতে হবে, যাতে `value` থেকে `a`-তে ownership হস্তান্তর না করে বা `a`-কে `value` থেকে borrow না করিয়ে
`a` আর `value` দুটিই ভেতরের `5` মানের ownership পায়।

তালিকা `a`-কে আমরা একটি `Rc<T>`-এ মুড়ে দিই, যাতে `b` আর `c` তালিকা তৈরির সময় দুটিই `a`-কে নির্দেশ করতে পারে, যেমনটি Listing 15-18-এ করেছিলাম।

`a`, `b` আর `c`-তে তালিকাগুলো তৈরি করার পর আমরা `value`-এর মানের সঙ্গে 10 যোগ করতে চাই। এটা করি `value`-এর ওপর `borrow_mut` call করে, যা অধ্যায় 4-এ আলোচিত স্বয়ংক্রিয়
dereference বৈশিষ্ট্য ব্যবহার করে `Rc<T>`-কে dereference করে ভেতরের `RefCell<T>` মানে পৌঁছায়। `borrow_mut`
method একটি `RefMut<T>` smart pointer return করে, আর আমরা এর ওপর dereference operator ব্যবহার করে ভেতরের মান বদলাই।

`a`, `b` আর `c` print করলে দেখতে পাই সবগুলোতেই `5`-এর বদলে পরিবর্তিত মান `15` আছে:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-24/output.txt}}
```

এই কৌশলটি বেশ চমৎকার! `RefCell<T>` ব্যবহার করে আমরা বাইরে থেকে immutable একটি `List` মান পাই। কিন্তু `RefCell<T>`-এর যে method-গুলো এর interior mutability-তে প্রবেশাধিকার
দেয়, সেগুলো ব্যবহার করে দরকার হলে ডেটা পরিবর্তন করতে পারি। borrowing-এর নিয়মের runtime যাচাই আমাদের data race থেকে রক্ষা করে, আর data structure-এ এই নমনীয়তার জন্য
কখনো কখনো একটু গতি ছেড়ে দেওয়া সার্থক। খেয়াল করুন, `RefCell<T>` multithreaded কোডে কাজ করে না!
`Mutex<T>` হলো `RefCell<T>`-এর thread-safe সংস্করণ, আর `Mutex<T>` নিয়ে অধ্যায় 16-এ আলোচনা করব।

{{#quiz ../quizzes/ch15-05-interior-mutability.toml}}

[wheres-the---operator]: ch05-03-method-syntax.html#wheres-the---operator
