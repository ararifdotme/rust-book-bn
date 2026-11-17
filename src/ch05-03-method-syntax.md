## method {#methods}

method (টাইপের সঙ্গে যুক্ত function) অনেকটা function-এর মতো: আমরা এগুলো `fn` keyword আর
একটি নাম দিয়ে ঘোষণা করি, এদের parameter আর return মান থাকতে পারে, আর এদের ভেতরে এমন
কিছু কোড থাকে, যা অন্য কোথাও থেকে method-টি call করা হলে চলে। তবে function-এর বিপরীতে,
method সংজ্ঞায়িত হয় একটি struct (বা একটি enum বা একটি trait object (trait-এর মাধ্যমে
বিভিন্ন type ব্যবহার করার মান), যেগুলো নিয়ে আমরা যথাক্রমে [অধ্যায় 6][enums]<!-- ignore -->
আর [অধ্যায়
18][trait-objects]<!-- ignore -->-এ আলোচনা করব)-এর প্রেক্ষাপটে, আর এদের প্রথম parameter
সবসময় `self`, যা struct-এর সেই instance-টিকে প্রকাশ করে, যার ওপর method-টি call করা
হচ্ছে।

<!-- Old headings. Do not remove or links may break. -->

<a id="defining-methods"></a>

### method syntax {#method-syntax}

চলুন `Rectangle` instance-কে parameter হিসেবে নেওয়া `area` function-টি বদলে, তার বদলে
`Rectangle` struct-এর ওপর সংজ্ঞায়িত একটি `area` method বানাই, Listing 5-13-এ যেমন
দেখানো হয়েছে।

<Listing number="5-13" file-name="src/main.rs" caption="`Rectangle` struct-এর ওপর একটি `area` method সংজ্ঞায়িত করা">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-13/src/main.rs}}
```

</Listing>

`Rectangle`-এর প্রেক্ষাপটে function-টি সংজ্ঞায়িত করতে আমরা `Rectangle`-এর জন্য একটি
`impl` (implementation) block (একটি type-এর method ও associated function সংজ্ঞায়িত করার block) শুরু করি। এই `impl` block-এর ভেতরের সবকিছু `Rectangle`
type-এর সঙ্গে যুক্ত হবে। তারপর `area` function-টি `impl`-এর কার্লি ব্র্যাকেটের ভেতরে
সরিয়ে আনি, আর signature-এ ও body-র সবখানে প্রথম (এবং এই ক্ষেত্রে একমাত্র) parameter-টি
বদলে `self` করি। `main`-এ, যেখানে আমরা `area` function call করে `rect1` argument হিসেবে
পাঠাতাম, সেখানে এর বদলে _method syntax_ ব্যবহার করে আমাদের `Rectangle` instance-এর ওপর
`area` method call করতে পারি। method syntax বসে একটি instance-এর পরে: আমরা একটি বিন্দু,
তারপর method-এর নাম, বন্ধনী আর যেকোনো argument যোগ করি।

`area`-এর signature-এ আমরা `rectangle: &Rectangle`-এর বদলে `&self` ব্যবহার করি।
`&self` আসলে `self: &Self`-এর সংক্ষিপ্ত রূপ। একটি `impl` block-এর ভেতরে `Self` type হলো
যে type-এর জন্য `impl` block-টি লেখা, তার একটি alias (বিকল্প নাম)। method-এর প্রথম
parameter হিসেবে `Self` type-এর `self` নামের একটি parameter থাকতেই হবে, তাই Rust প্রথম
parameter-এর জায়গায় শুধু `self` নাম লিখে এটি সংক্ষেপ করতে দেয়। লক্ষ করুন, এই
method যে `Self` instance-টিকে borrow করে, তা বোঝাতে `self` সংক্ষিপ্ত রূপের সামনে আমাদের
তবু `&` বসাতে হয়, ঠিক যেমন `rectangle: &Rectangle`-এ করেছিলাম। অন্য যেকোনো
parameter-এর মতোই method `self`-এর ownership নিতে পারে, এখানে যেমন করেছি সেভাবে `self`-কে
immutable-ভাবে borrow করতে পারে, অথবা `self`-কে mutable-ভাবে borrow করতে পারে।

function সংস্করণে `&Rectangle` যে কারণে ব্যবহার করেছিলাম, এখানে `&self` বেছে নিয়েছি
সেই একই কারণে: আমরা ownership নিতে চাই না, আর struct-এর ডেটা শুধু পড়তে চাই, তাতে লিখতে
চাই না। method-এর কাজের অংশ হিসেবে যদি সেই instance-টি বদলাতে চাইতাম, যার ওপর method
call করেছি, তাহলে প্রথম parameter হিসেবে `&mut self` ব্যবহার করতাম। প্রথম parameter
হিসেবে শুধু `self` ব্যবহার করে instance-এর ownership নেয় এমন method বিরল; এই কৌশল সাধারণত
তখন ব্যবহার হয়, যখন method `self`-কে অন্য কিছুতে রূপান্তর করে এবং আপনি চান রূপান্তরের
পর caller (যে কোড function-টি call করে) যেন মূল instance-টি আর ব্যবহার করতে না পারে।

function-এর বদলে method ব্যবহারের প্রধান কারণ, method syntax পাওয়া আর প্রতিটি method-এর
signature-এ `self`-এর type বারবার লিখতে না হওয়ার পাশাপাশি, হলো কোড গুছিয়ে রাখা। একটি
type-এর instance দিয়ে যা যা করা যায়, তার সবকিছু আমরা একটি `impl` block-এ রেখেছি, যাতে
ভবিষ্যতে আমাদের কোড যারা ব্যবহার করবেন, তাদের আমাদের দেওয়া library-র নানা জায়গায়
`Rectangle`-এর সক্ষমতাগুলো খুঁজে বেড়াতে না হয়।

লক্ষ করুন, struct-এর কোনো একটি field-এর নামেই একটি method-এর নাম রাখা যায়। যেমন,
`Rectangle`-এর ওপর আমরা এমন একটি method সংজ্ঞায়িত করতে পারি, যার নামও
`width`:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-06-method-field-interaction/src/main.rs:here}}
```

</Listing>

এখানে আমরা ঠিক করেছি, instance-এর `width` field-এর মান `0`-এর চেয়ে বড় হলে `width`
method `true` return করবে, আর মান `0` হলে `false`: একই নামের method-এর ভেতরে আমরা
যেকোনো উদ্দেশ্যে field ব্যবহার করতে পারি। `main`-এ যখন `rect1.width`-এর পরে বন্ধনী দিই,
Rust বোঝে আমরা `width` method বোঝাচ্ছি। বন্ধনী না দিলে Rust বোঝে আমরা `width` field
বোঝাচ্ছি।

প্রায়ই, তবে সবসময় নয়, যখন কোনো method-এর নাম একটি field-এর নামে রাখি, তখন চাই এটি শুধু
field-এর মানটি return করুক, আর কিছু না করুক। এ ধরনের method-কে বলা হয় _getter_ (শুধু
একটি field-এর মান ফেরত দেয় এমন method), আর অন্য কিছু ভাষার মতো Rust struct-এর field-এর
জন্য এগুলো আপনা-আপনি implement করে না। getter কাজের, কারণ field-টিকে private (বাইরে থেকে ব্যবহার করা যায় না) আর
method-টিকে public (বাইরে থেকে ব্যবহার করা যায়) করে type-এর public API-র অংশ
হিসেবে ওই field-এ শুধু-পড়ার প্রবেশাধিকার দেওয়া যায়। public আর private কী, আর কীভাবে
একটি field বা method-কে public বা private হিসেবে চিহ্নিত করতে হয়, তা আমরা [অধ্যায়
7][public]<!-- ignore -->-এ আলোচনা করব।

### আরও parameter-সহ method {#methods-with-more-parameters}

`Rectangle` struct-এ দ্বিতীয় একটি method implement করে চলুন method ব্যবহার অনুশীলন করি।
এবার আমরা চাই `Rectangle`-এর একটি instance `Rectangle`-এর আরেকটি instance নেবে এবং দ্বিতীয়
`Rectangle`-টি যদি পুরোপুরি `self` (প্রথম `Rectangle`)-এর ভেতরে এঁটে যায়, তাহলে `true`
return করবে; অন্যথায় `false` return করবে।
অর্থাৎ `can_hold` method সংজ্ঞায়িত করার পর আমরা Listing 5-14-এ দেখানো প্রোগ্রামটি লিখতে
চাই।

<Listing number="5-14" file-name="src/main.rs" caption="এখনো না লেখা `can_hold` method ব্যবহার করা">

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-14/src/main.rs}}
```

</Listing>

প্রত্যাশিত আউটপুট নিচের মতো হবে, কারণ `rect2`-এর দুটি মাপই `rect1`-এর মাপের চেয়ে ছোট,
কিন্তু `rect3` `rect1`-এর চেয়ে বেশি চওড়া:

```text
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

আমরা জানি একটি method সংজ্ঞায়িত করতে চাই, তাই এটি থাকবে `impl Rectangle` block-এর
ভেতরে। method-এর নাম হবে `can_hold`, আর এটি parameter হিসেবে আরেকটি `Rectangle`-এর
একটি immutable borrow নেবে। method-টি call করা কোড দেখে parameter-এর type কী হবে, তা
আমরা বলতে পারি: `rect1.can_hold(&rect2)` পাঠায় `&rect2`, যা `Rectangle`-এর একটি instance
`rect2`-এর একটি immutable borrow। এটা যুক্তিসংগত, কারণ আমাদের শুধু `rect2` পড়তে হবে
(লেখার দরকার হলে mutable borrow লাগত), আর আমরা চাই `main` `rect2`-এর ownership ধরে রাখুক,
যাতে `can_hold` method call করার পরও আবার এটি ব্যবহার করতে পারি। `can_hold`-এর return
মান হবে একটি Boolean, আর implementation যাচাই করবে `self`-এর প্রস্থ ও উচ্চতা যথাক্রমে অন্য
`Rectangle`-টির প্রস্থ ও উচ্চতার চেয়ে বড় কি না। চলুন Listing 5-13-এর `impl` block-এ নতুন
`can_hold` method যোগ করি, যা Listing 5-15-এ দেখানো হয়েছে।

<Listing number="5-15" file-name="src/main.rs" caption="`Rectangle`-এর ওপর `can_hold` method implement করা, যা parameter হিসেবে আরেকটি `Rectangle` instance নেয়">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-15/src/main.rs:here}}
```

</Listing>

Listing 5-14-এর `main` function দিয়ে এই কোড চালালে আমরা কাঙ্ক্ষিত আউটপুট পাব। method
একাধিক parameter নিতে পারে, যেগুলো আমরা signature-এ `self` parameter-এর পরে যোগ করি, আর
সেই parameter-গুলো function-এর parameter-এর মতোই কাজ করে।


### associated function {#associated-functions}

একটি `impl` block-এর ভেতরে সংজ্ঞায়িত সব function-কে বলা হয় _associated function_,
কারণ এরা `impl`-এর পরে নাম লেখা type-টির সঙ্গে যুক্ত। আমরা এমন associated function-ও
সংজ্ঞায়িত করতে পারি, যাদের প্রথম parameter `self` নয় (এবং তাই এরা
method নয়), কারণ কাজ করার জন্য এদের type-টির কোনো instance দরকার হয় না।
এমন একটি function আমরা আগেই ব্যবহার করেছি: `String` type-এর ওপর সংজ্ঞায়িত `String::from`
function।

method নয় এমন associated function প্রায়ই constructor (নতুন instance তৈরি করে এমন
function) হিসেবে ব্যবহার হয়, যা struct-এর একটি নতুন instance return করে। এগুলোর নাম প্রায়ই
`new` রাখা হয়, কিন্তু `new` কোনো বিশেষ নাম নয় এবং ভাষার মধ্যে বিল্ট-ইন নয়। যেমন, আমরা
`square` নামে একটি associated function দিতে পারি, যার একটি মাপের parameter থাকবে এবং সেটিই
প্রস্থ ও উচ্চতা দুটো হিসেবে ব্যবহার করবে, ফলে একই মান দুবার নির্দিষ্ট না করে সহজেই একটি
বর্গাকার `Rectangle` তৈরি করা যাবে:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-03-associated-functions/src/main.rs:here}}
```

return type আর function-এর body-তে থাকা `Self` keyword-গুলো হলো `impl` keyword-এর পরে
থাকা type-টির alias, যা এই ক্ষেত্রে `Rectangle`।

এই associated function call করতে আমরা struct-এর নামের সঙ্গে `::` syntax ব্যবহার করি;
`let sq = Rectangle::square(3);` একটি উদাহরণ। এই function struct-টির namespace (নামগুলোর
আলাদা একটি পরিসর)-এর ভেতরে থাকে: `::` syntax associated function আর module-এর তৈরি
namespace দুটোর জন্যই ব্যবহার হয়। module নিয়ে আমরা [অধ্যায়
7][modules]<!-- ignore -->-এ আলোচনা করব।

### একাধিক `impl` block {#multiple-impl-blocks}

প্রতিটি struct-এর একাধিক `impl` block থাকতে পারে। যেমন, Listing
5-15 Listing 5-16-এ দেখানো কোডের সমতুল্য, যেখানে প্রতিটি method নিজের আলাদা `impl`
block-এ আছে।

<Listing number="5-16" caption="একাধিক `impl` block ব্যবহার করে Listing 5-15 নতুন করে লেখা">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-16/src/main.rs:here}}
```

</Listing>

এখানে এই method-গুলোকে একাধিক `impl` block-এ আলাদা করার কোনো কারণ নেই, তবে এটি বৈধ
syntax। একাধিক `impl` block কাজে আসে এমন একটি ক্ষেত্র আমরা অধ্যায় 10-এ দেখব, যেখানে
generic type আর trait নিয়ে আলোচনা করব।

### method call হলো function call-এর syntactic sugar {#method-calls-are-syntactic-sugar-for-function-calls}

এ পর্যন্ত আলোচিত ধারণাগুলো ব্যবহার করে এখন আমরা দেখতে পারি, method call কীভাবে function call-এর syntactic sugar (একই কাজের সহজতর লেখার রূপ)। যেমন, ধরুন আমাদের কাছে একটি rectangle struct আছে, যার একটি `area` method আর একটি `set_width` method আছে:

```rust,ignore
# struct Rectangle {
#     width: u32,
#     height: u32,
# }
# 
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}
```

আর ধরুন আমাদের কাছে একটি rectangle `r` আছে। তাহলে `r.area()` আর `r.set_width(2)` method call দুটি এর সমতুল্য:

```rust
# struct Rectangle {
#     width: u32,
#     height: u32,
# }
# 
# impl Rectangle {
#     fn area(&self) -> u32 {
#        self.width * self.height
#      }
# 
#     fn set_width(&mut self, width: u32) {
#         self.width = width;
#     }
# }
# 
# fn main() {
let mut r = Rectangle { 
    width: 1,
    height: 2
};
let area1 = r.area();
let area2 = Rectangle::area(&r);
assert_eq!(area1, area2);

r.set_width(2);
Rectangle::set_width(&mut r, 2);
# }
```

`r.area()` method call হয়ে যায় `Rectangle::area(&r)`। function-এর নাম হলো associated function `Rectangle::area`। function-এর argument হলো `&self` parameter। Rust আপনা-আপনি borrowing operator `&` বসিয়ে দেয়।

> *দ্রষ্টব্য:* আপনি যদি C বা C++-এর সঙ্গে পরিচিত হন, তাহলে method call-এর দুটি আলাদা syntax-এ অভ্যস্ত: `r.area()` আর `r->area()`। Rust-এ তীর operator `->`-এর কোনো সমতুল্য নেই। dot operator ব্যবহার করলে Rust আপনা-আপনিই method receiver (যে মানের ওপর method call করা হয়)-কে reference ও dereference করে।

`r.set_width(2)` method call-ও একইভাবে হয়ে যায় `Rectangle::set_width(&mut r, 2)`। এই method `&mut self` আশা করে, তাই প্রথম argument হলো একটি mutable borrow `&mut r`। দ্বিতীয় argument হুবহু একই, সংখ্যা 2।

অধ্যায় 4.2-এর ["pointer dereference করলে তার ডেটায় পৌঁছানো যায়"](ch04-02-references-and-borrowing.html#dereferencing-a-pointer-accesses-its-data)-এ যেমন বর্ণনা করেছি, `self` parameter-এর type মেলাতে Rust যত দরকার তত reference আর dereference বসিয়ে দেবে। যেমন, box-এ রাখা একটি rectangle-এর mutable reference-এর জন্য `area`-এর দুটি সমতুল্য call এখানে দেওয়া হলো:

```rust
# struct Rectangle {
#     width: u32,
#     height: u32,
# }
# 
# impl Rectangle {
#     fn area(&self) -> u32 {
#        self.width * self.height
#      }
# 
#     fn set_width(&mut self, width: u32) {
#         self.width = width;
#     }
# }
# fn main() {
let r = &mut Box::new(Rectangle { 
    width: 1,
    height: 2
});
let area1 = r.area();
let area2 = Rectangle::area(&**r);
assert_eq!(area1, area2);
# }
```

Rust দুটি dereference যোগ করবে (একবার mutable reference-এর জন্য, একবার box-এর জন্য), তারপর একটি immutable borrow, কারণ `area` `&Rectangle` আশা করে। লক্ষ করুন, এটিও এমন একটি পরিস্থিতি, যেখানে একটি mutable reference-কে একটি shared reference-এ "নামিয়ে আনা" হয়, যেমনটি আমরা [অধ্যায় 4.2](ch04-02-references-and-borrowing.html#mutable-references-provide-unique-and-non-owning-access-to-data)-এ আলোচনা করেছি। উল্টোভাবে, `&Rectangle` বা `&Box<Rectangle>` type-এর মানের ওপর `set_width` call করার অনুমতি আপনি পাবেন না।

{{#quiz ../quizzes/ch05-03-method-syntax-sec1.toml}}


### method ও ownership {#methods-and-ownership}

অধ্যায় 4.2-এর ["reference ও borrowing"](ch04-02-references-and-borrowing.html)-এ যেমন আলোচনা করেছি, method call করতে হয় এমন struct-এর ওপর, যার প্রয়োজনীয় permission (অনুমতি) আছে। চলমান উদাহরণ হিসেবে আমরা এই তিনটি method ব্যবহার করব, যেগুলো যথাক্রমে `&self`, `&mut self` আর `self` নেয়।

```rust,ignore
impl Rectangle {    
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle { 
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}
```

#### `&self` আর `&mut self` দিয়ে পড়া ও লেখা {#reads-and-writes-with-self-and-mut-self}

`let rect = Rectangle { ... }` দিয়ে আমরা যদি একটি owned rectangle তৈরি করি, তাহলে `rect`-এর @Perm{read} আর @Perm{own} permission থাকে। এই permission-গুলো থাকায় `area` আর `max` method call করা চলে:

```aquascope,permissions,boundaries,stepper
#struct Rectangle {
#    width: u32,
#    height: u32,
#}
#impl Rectangle {    
#  fn area(&self) -> u32 {
#    self.width * self.height
#  }
#
#  fn set_width(&mut self, width: u32) {
#    self.width = width;
#  }
#
#  fn max(self, other: Self) -> Self {
#    let w = self.width.max(other.width);
#    let h = self.height.max(other.height);
#    Rectangle { 
#      width: w,
#      height: h
#    }
#  }
#}
#fn main() {
let rect = Rectangle {
    width: 0,
    height: 0
};`(focus,rxpaths:^rect$)`
println!("{}", rect.area());`{}`

let other_rect = Rectangle { width: 1, height: 1 };
let max_rect = rect.max(other_rect);`{}`
#}
```

তবে `set_width` call করার চেষ্টা করলে দেখা যায়, আমাদের @Perm{write} permission নেই:

```aquascope,permissions,boundaries,shouldFail
#struct Rectangle {
#    width: u32,
#    height: u32,
#}
#impl Rectangle {    
#  fn area(&self) -> u32 {
#    self.width * self.height
#  }
#
#  fn set_width(&mut self, width: u32) {
#    self.width = width;
#  }
#
#  fn max(self, other: Self) -> Self {
#    let w = self.width.max(other.width);
#    let h = self.height.max(other.height);
#    Rectangle { 
#      width: w,
#      height: h
#    }
#  }
#}
#fn main() {
let rect = Rectangle {
    width: 0,
    height: 0
};
rect.set_width(0);`{}`
#}
```

Rust সংশ্লিষ্ট error দিয়ে এই প্রোগ্রাম বাতিল করবে:

```text
error[E0596]: cannot borrow `rect` as mutable, as it is not declared as mutable
  --> test.rs:28:1
   |
24 | let rect = Rectangle {
   |     ---- help: consider changing this to be mutable: `mut rect`
...
28 | rect.set_width(0);
   | ^^^^^^^^^^^^^^^^^ cannot borrow as mutable
```

একটি `Rectangle`-এর immutable reference-এর ওপর `set_width` call করার চেষ্টা করলেও একই রকম error পাব, এমনকি মূল rectangle-টি mutable হলেও:

```aquascope,permissions,boundaries,stepper,shouldFail
#struct Rectangle {
#    width: u32,
#    height: u32,
#}
#impl Rectangle {    
#  fn area(&self) -> u32 {
#    self.width * self.height
#  }
#
#  fn set_width(&mut self, width: u32) {
#    self.width = width;
#  }
#
#  fn max(self, other: Self) -> Self {
#    let w = self.width.max(other.width);
#    let h = self.height.max(other.height);
#    Rectangle { 
#      width: w,
#      height: h
#    }
#  }
#}
#fn main() {
// Added the mut keyword to the let-binding
let mut rect = Rectangle {
    width: 0,
    height: 0
};`(focus,rxpaths:^rect$)`
rect.set_width(1);`{}`     // this is now ok

let rect_ref = &rect;`(focus,rxpaths:^\*rect_ref$)`
rect_ref.set_width(2);`{}` // but this is still not ok
#}
```

#### `self` দিয়ে move {#moves-with-self}

`self` আশা করে এমন method call করলে ইনপুট struct-টি move হয়ে যাবে (যদি না struct-টি `Copy` implement করে)। যেমন, `max`-এ পাঠানোর পর আমরা একটি `Rectangle` আর ব্যবহার করতে পারি না:

```aquascope,permissions,boundaries,stepper,shouldFail
#struct Rectangle {
#    width: u32,
#    height: u32,
#}
#impl Rectangle {    
#  fn area(&self) -> u32 {
#    self.width * self.height
#  }
#
#  fn set_width(&mut self, width: u32) {
#    self.width = width;
#  }
#
#  fn max(self, other: Self) -> Self {
#    let w = self.width.max(other.width);
#    let h = self.height.max(other.height);
#    Rectangle { 
#      width: w,
#      height: h
#    }
#  }
#}
#fn main() {
let rect = Rectangle {
    width: 0,
    height: 0
};`(focus,rxpaths:^rect$)`
let other_rect = Rectangle { 
    width: 1, 
    height: 1 
};
let max_rect = rect.max(other_rect);`(focus,rxpaths:^rect$)`
println!("{}", rect.area());`{}`
#}
```

`rect.max(..)` call করার সঙ্গে সঙ্গে আমরা `rect` move করে ফেলি, ফলে এর ওপর সব permission হারাই। এই প্রোগ্রাম compile করার চেষ্টা করলে নিচের error পাব:

```text
error[E0382]: borrow of moved value: `rect`
  --> test.rs:33:16
   |
24 | let rect = Rectangle {
   |     ---- move occurs because `rect` has type `Rectangle`, which does not implement the `Copy` trait
...
32 | let max_rect = rect.max(other_rect);
   |                     --------------- `rect` moved due to this method call
33 | println!("{}", rect.area());
   |                ^^^^^^^^^^^ value borrowed here after move
```

একটি reference-এর ওপর একটি `self` method call করার চেষ্টা করলেও একই রকম পরিস্থিতি তৈরি হয়। যেমন, ধরুন আমরা `set_to_max` নামে একটি method বানানোর চেষ্টা করলাম, যা `self.max(..)`-এর আউটপুট `self`-এ assign করে:

```aquascope,permissions,boundaries,stepper,shouldFail
#struct Rectangle {
#    width: u32,
#    height: u32,
#}
impl Rectangle {    
#  fn area(&self) -> u32 {
#    self.width * self.height
#  }
#
#  fn set_width(&mut self, width: u32) {
#    self.width = width;
#  }
#
#  fn max(self, other: Self) -> Self {
#    let w = self.width.max(other.width);
#    let h = self.height.max(other.height);
#    Rectangle { 
#      width: w,
#      height: h
#    }
#  }
    fn set_to_max(&mut self, other: Rectangle) {`(focus,rxpaths:^\*self$)`
        *self = self.max(other);`{}`
    }
}
```

তখন দেখতে পাই, `self.max(..)` operation-এ `self`-এর @Perm{own} permission নেই। তাই Rust নিচের error দিয়ে এই প্রোগ্রাম বাতিল করে:

```text
error[E0507]: cannot move out of `*self` which is behind a mutable reference
  --> test.rs:23:17
   |
23 |         *self = self.max(other);
   |                 ^^^^^----------
   |                 |    |
   |                 |    `*self` moved due to this method call
   |                 move occurs because `*self` has type `Rectangle`, which does not implement the `Copy` trait
   |
```

এটি সেই একই ধরনের error, যা আমরা অধ্যায় 4.3-এর ["collection থেকে কপি বনাম move করা"](ch04-03-fixing-ownership-errors.html#fixing-an-unsafe-program-copying-vs-moving-out-of-a-collection)-এ আলোচনা করেছি।

#### ভালো move আর খারাপ move {#good-moves-and-bad-moves}

আপনি হয়তো ভাবছেন: `*self` থেকে move করে বের করলে কী এসে যায়? আসলে `Rectangle`-এর ক্ষেত্রে `*self` থেকে move করে বের করা সত্যিই নিরাপদ, যদিও Rust আপনাকে তা করতে দেয় না। যেমন, বাতিল হওয়া `set_to_max` call করে এমন একটি প্রোগ্রাম অনুকরণ করলে দেখতে পাবেন, কোনো অনিরাপদ কিছুই ঘটে না:

```aquascope,interpreter,shouldFail,horizontal
#struct Rectangle {
#    width: u32,
#    height: u32,
#}
impl Rectangle {    
#  fn max(self, other: Self) -> Self {
#    let w = self.width.max(other.width);
#    let h = self.height.max(other.height);
#    Rectangle { 
#      width: w,
#      height: h
#    }
#  }
    fn set_to_max(&mut self, other: Rectangle) {
        let max = self.max(other);`[]`
        *self = max;
    }
}

fn main() {
    let mut rect = Rectangle { width: 0, height: 1 };
    let other_rect = Rectangle { width: 1, height: 0 };`[]`
    rect.set_to_max(other_rect);`[]`
}
```

`*self` থেকে move করে বের করা নিরাপদ হওয়ার কারণ হলো, `Rectangle` কোনো heap ডেটার owner নয়।
আসলে শুধু `Rectangle`-এর সংজ্ঞায় `#[derive(Copy, Clone)]` যোগ করেই আমরা Rust-কে দিয়ে `set_to_max` compile করাতে পারি:

```aquascope,permissions,boundaries,stepper
\#[derive(Copy, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {    
#  fn max(self, other: Self) -> Self {
#    let w = self.width.max(other.width);
#    let h = self.height.max(other.height);
#    Rectangle { 
#      width: w,
#      height: h
#    }
#  }
    fn set_to_max(&mut self, other: Rectangle) {`(focus,rxpaths:^\*self$)`
        *self = self.max(other);`{}`
    }
}
```

লক্ষ করুন, আগের বিপরীতে `self.max(other)`-এর জন্য এখন আর `*self` বা `other`-এর ওপর @Perm{own} permission লাগে না। মনে রাখবেন, `self.max(other)` desugar (syntactic sugar খুলে মূল রূপে লেখা) হয়ে দাঁড়ায় `Rectangle::max(*self, other)`। `Rectangle` copyable হলে `*self` dereference-এর জন্য `*self`-এর ওপর ownership লাগে না।

আপনি হয়তো ভাবছেন: Rust তাহলে `Rectangle`-এর জন্য আপনা-আপনি `Copy` derive করে না কেন? API বদলালেও যাতে স্থিতিশীলতা থাকে, সেজন্য Rust আপনা-আপনি `Copy` derive করে না। কল্পনা করুন, `Rectangle` type-এর লেখক একটি `name: String` field যোগ করার সিদ্ধান্ত নিলেন। তাহলে `Rectangle`-এর `Copy` হওয়ার ওপর নির্ভর করা সব ব্যবহারকারী কোড হঠাৎ compiler-এর কাছে বাতিল হয়ে যেত। এই সমস্যা এড়াতে API-র লেখকদের স্পষ্টভাবে `#[derive(Copy)]` যোগ করতে হয়, যাতে বোঝা যায় তারা আশা করেন তাদের struct সবসময় `Copy` থাকবে।

সমস্যাটি আরও ভালোভাবে বুঝতে চলুন একটি অনুকরণ চালাই। ধরুন আমরা `Rectangle`-এ `name: String` যোগ করলাম। Rust যদি `set_to_max`-কে compile হতে দিত, তাহলে কী হতো?

```aquascope,interpreter,shouldFail,horizontal
struct Rectangle {
    width: u32,
    height: u32,
    name: String,
}

impl Rectangle {    
#  fn max(self, other: Self) -> Self {
#    let w = self.width.max(other.width);
#    let h = self.height.max(other.height);
#    Rectangle { 
#      width: w,
#      height: h,
#      name: String::from("max")
#    }
#  }
    fn set_to_max(&mut self, other: Rectangle) {
        `[]`let max = self.max(other);`[]`
        drop(*self);`[]` // This is usually implicit,
                         // but added here for clarity.
        *self = max;
    }
}

fn main() {
    let mut r1 = Rectangle { 
        width: 9, 
        height: 9, 
        name: String::from("r1") 
    };
    let r2 = Rectangle {
        width: 16,
        height: 16,
        name: String::from("r2")
    };
    r1.set_to_max(r2);
}
```

এই প্রোগ্রামে আমরা দুটি rectangle `r1` আর `r2` দিয়ে `set_to_max` call করি। `self` হলো `r1`-এর একটি mutable reference, আর `other` হলো `r2`-এর একটি move। `self.max(other)` call করার পর `max` method দুটি rectangle-এরই ownership নিয়ে নেয়। `max` return করলে Rust heap-এ থাকা "r1" আর "r2" দুটি string-ই deallocate করে। সমস্যাটি লক্ষ করুন: L2 অবস্থানে `*self`-এর পড়া ও লেখার উপযোগী থাকার কথা। কিন্তু `(*self).name` (আসলে `r1.name`) deallocate হয়ে গেছে।

তাই আমরা যখন `*self = max` করি, তখন undefined behavior-এর মুখোমুখি হই। `*self`-এর ওপর নতুন করে লিখলে Rust আগে `*self`-এ থাকা ডেটা নিজে থেকেই drop করবে। এই আচরণটি স্পষ্ট করতে আমরা `drop(*self)` যোগ করেছি। `drop(*self)` call করার পর Rust দ্বিতীয়বার `(*self).name` free করার চেষ্টা করে। এই কাজটি একটি double-free, যা undefined behavior।

তাই মনে রাখবেন: "cannot move out of `*self`"-এর মতো error দেখলে, তা সাধারণত এ কারণে যে আপনি `&self` বা `&mut self`-এর মতো একটি reference-এর ওপর একটি `self` method call করার চেষ্টা করছেন। Rust আপনাকে একটি double-free থেকে রক্ষা করছে।


## সারসংক্ষেপ {#summary}

struct দিয়ে আপনি এমন নিজস্ব type তৈরি করতে পারেন, যা আপনার বিষয়ক্ষেত্রের জন্য অর্থবহ।
struct ব্যবহার করে আপনি সম্পর্কিত ডেটার অংশগুলোকে পরস্পরের সঙ্গে যুক্ত রাখতে পারেন, আর
প্রতিটি অংশের নাম দিয়ে আপনার কোড স্পষ্ট করতে পারেন। `impl` block-এ আপনি এমন function
সংজ্ঞায়িত করতে পারেন, যা আপনার type-এর সঙ্গে যুক্ত, আর method হলো এক ধরনের associated
function, যা দিয়ে আপনার struct-এর instance-গুলোর আচরণ নির্দিষ্ট করা যায়।

তবে নিজস্ব type তৈরির একমাত্র উপায় struct নয়: আপনার হাতিয়ারের বাক্সে আরেকটি হাতিয়ার
যোগ করতে চলুন Rust-এর enum বৈশিষ্ট্যের দিকে যাই।

{{#quiz ../quizzes/ch05-03-method-syntax-sec2.toml}}

[enums]: ch06-00-enums.html
[trait-objects]: ch18-02-trait-objects.md
[public]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#exposing-paths-with-the-pub-keyword
[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
