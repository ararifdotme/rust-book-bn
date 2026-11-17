## struct সংজ্ঞায়িত ও instantiate করা {#defining-and-instantiating-structs}

struct অনেকটা [“tuple type”][tuples]<!--
ignore --> অংশে আলোচিত tuple-এর মতো, কারণ দুটিই একাধিক সম্পর্কিত মান ধরে রাখে।
tuple-এর মতোই struct-এর অংশগুলো বিভিন্ন type-এর হতে পারে। তবে tuple-এর বিপরীতে,
struct-এ আপনি ডেটার প্রতিটি অংশের নাম দেবেন, যাতে মানগুলোর অর্থ স্পষ্ট হয়। এই নামগুলো
যোগ করার ফলে struct tuple-এর চেয়ে বেশি নমনীয়: একটি instance (কোনো type-এর একটি
নির্দিষ্ট মান)-এর মান নির্দিষ্ট করতে বা পড়তে ডেটার ক্রমের ওপর নির্ভর করতে হয় না।

struct সংজ্ঞায়িত করতে আমরা `struct` keyword (ভাষার সংরক্ষিত শব্দ) লিখি এবং পুরো
struct-টির একটি নাম দিই। struct-এর নাম এমন হওয়া উচিত, যা একত্র করা ডেটার অংশগুলোর
তাৎপর্য বোঝায়। তারপর কার্লি ব্র্যাকেটের ভেতরে ডেটার অংশগুলোর নাম আর type সংজ্ঞায়িত
করি, যেগুলোকে আমরা বলি _field_ (struct বা tuple-এর একেকটি অংশ)। যেমন, Listing 5-1-এ
একটি struct দেখানো হয়েছে, যা একটি ব্যবহারকারী অ্যাকাউন্টের তথ্য রাখে।

<Listing number="5-1" file-name="src/main.rs" caption="একটি `User` struct-এর সংজ্ঞা">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-01/src/main.rs:here}}
```

</Listing>

একটি struct সংজ্ঞায়িত করার পর সেটি ব্যবহার করতে আমরা প্রতিটি field-এর জন্য নির্দিষ্ট
মান দিয়ে সেই struct-এর একটি _instance_ তৈরি করি। instance তৈরি করতে struct-এর নাম
লিখি, তারপর কার্লি ব্র্যাকেট যোগ করি, যার ভেতরে থাকে _`key:
value`_ জোড়া; এখানে key হলো field-এর নাম, আর value হলো সেই field-এ যে ডেটা রাখতে
চাই। struct-এ field-গুলো যে ক্রমে ঘোষণা করেছি, একই ক্রমে সেগুলো নির্দিষ্ট করতে হবে না।
অন্যভাবে বললে, struct-এর সংজ্ঞা হলো type-টির একটি সাধারণ ছাঁচের মতো, আর instance-গুলো
নির্দিষ্ট ডেটা দিয়ে সেই ছাঁচ পূরণ করে type-টির মান তৈরি করে। যেমন, Listing 5-2-এ
দেখানো উপায়ে আমরা একজন নির্দিষ্ট ব্যবহারকারী ঘোষণা করতে পারি।

```aquascope,interpreter
#struct User {
#    active: bool,
#    username: String,
#    email: String,
#    sign_in_count: u64,
#}
fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };`[]`
}
```

struct থেকে একটি নির্দিষ্ট মান পেতে আমরা dot notation (`.` দিয়ে field বা method-এ
পৌঁছানোর লিখনরীতি) ব্যবহার করি। যেমন, এই ব্যবহারকারীর ইমেইল ঠিকানা পেতে আমরা
`user1.email` ব্যবহার করি। instance-টি mutable (পরিবর্তনযোগ্য) হলে dot notation ব্যবহার
করে একটি নির্দিষ্ট field-এ assign করে আমরা মান বদলাতে পারি। Listing 5-3-এ দেখানো
হয়েছে, একটি mutable `User` instance-এর `email` field-এর মান কীভাবে বদলাতে হয়।

```aquascope,interpreter
#struct User {
#    active: bool,
#    username: String,
#    email: String,
#    sign_in_count: u64,
#}
fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };`[]`

    user1.email = String::from("anotheremail@example.com");`[]`
}
```

লক্ষ করুন, পুরো instance-টিকেই mutable হতে হবে; Rust শুধু নির্দিষ্ট কিছু field-কে
mutable হিসেবে চিহ্নিত করার অনুমতি দেয় না। অন্য যেকোনো expression (মান তৈরি করে এমন
কোড-অংশ)-এর মতোই, function body-র শেষ expression হিসেবে struct-এর একটি নতুন
instance তৈরি করে আমরা সেই নতুন instance-টি নিজে থেকেই return (ফেরত দেওয়া) করতে পারি।

Listing 5-4-এ একটি `build_user` function দেখানো হয়েছে, যা দেওয়া ইমেইল আর
username দিয়ে একটি `User` instance return করে। `active` field পায় `true` মান, আর
`sign_in_count` পায় `1` মান।

<Listing number="5-4" file-name="src/main.rs" caption="একটি `build_user` function, যা একটি ইমেইল ও username নিয়ে একটি `User` instance return করে">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-04/src/main.rs:here}}
```

</Listing>

function-এর parameter-গুলোর নাম struct-এর field-এর নামেই রাখা যুক্তিসংগত, কিন্তু
`email` আর `username` field-এর নাম আর variable বারবার লিখতে হওয়াটা একটু ক্লান্তিকর।
struct-এ আরও বেশি field থাকলে প্রতিটি নাম বারবার লেখা আরও বিরক্তিকর হতো। সৌভাগ্যক্রমে,
এর একটি সুবিধাজনক সংক্ষিপ্ত রূপ আছে!

<!-- Old headings. Do not remove or links may break. -->

<a id="using-the-field-init-shorthand-when-variables-and-fields-have-the-same-name"></a>

### field init shorthand ব্যবহার করা {#using-the-field-init-shorthand}

Listing 5-4-এ parameter-এর নাম আর struct-এর field-এর নাম হুবহু এক বলে আমরা _field init
shorthand_ (variable আর field-এর নাম এক হলে সংক্ষিপ্ত লেখা) syntax ব্যবহার করে
`build_user` নতুন করে লিখতে পারি, যাতে এটি হুবহু একই আচরণ করে, কিন্তু `username`
আর `email` বারবার লিখতে হয় না, Listing 5-5-এ যেমন দেখানো হয়েছে।

<Listing number="5-5" file-name="src/main.rs" caption="একটি `build_user` function, যা field init shorthand ব্যবহার করে, কারণ `username` আর `email` parameter-এর নাম struct-এর field-এর নামের সমান">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-05/src/main.rs:here}}
```

</Listing>

এখানে আমরা `User` struct-এর একটি নতুন instance তৈরি করছি, যার একটি field-এর নাম
`email`। আমরা `email` field-এর মান হিসেবে `build_user` function-এর `email`
parameter-এর মানটি বসাতে চাই। `email` field আর `email` parameter-এর নাম এক বলে
`email: email` লেখার বদলে শুধু `email` লিখলেই চলে।

<!-- Old headings. Do not remove or links may break. -->

<a id="creating-instances-from-other-instances-with-struct-update-syntax"></a>

### struct update syntax দিয়ে instance তৈরি করা {#creating-instances-with-struct-update-syntax}

প্রায়ই একটি struct-এর এমন নতুন instance তৈরি করা কাজে লাগে, যাতে একই type-এর অন্য
একটি instance-এর বেশিরভাগ মান থাকে, কিন্তু কয়েকটি বদলানো হয়।
struct update syntax ব্যবহার করে আপনি এটি করতে পারেন।

প্রথমে, Listing 5-6-এ আমরা দেখাব কীভাবে update syntax ছাড়া, স্বাভাবিক উপায়ে
`user2`-তে একটি নতুন `User` instance তৈরি করা যায়। আমরা `email`-এর জন্য একটি নতুন মান
দিই, কিন্তু বাকিগুলোর জন্য Listing 5-2-এ তৈরি `user1`-এর মানগুলোই ব্যবহার করি।

```aquascope,interpreter
#struct User {
#    active: bool,
#    username: String,
#    email: String,
#    sign_in_count: u64,
#}
fn main() {
#   let user1 = User {
#      email: String::from("someone@example.com"),
#      username: String::from("someusername123"),
#      active: true,
#      sign_in_count: 1,
#   };
    // --snip--

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };`[]`
}
```

<span class="caption">Listing 5-6: `user1`-এর একটি ছাড়া বাকি সব মান ব্যবহার করে একটি
নতুন `User` instance তৈরি করা</span>

struct update syntax (অন্য instance থেকে বাকি field-এর মান নেওয়ার syntax) ব্যবহার করে
আমরা কম কোডে একই ফল পেতে পারি, Listing 5-7-এ যেমন দেখানো হয়েছে। `..` syntax বোঝায় যে
স্পষ্টভাবে সেট না করা বাকি field-গুলোর মান হবে দেওয়া instance-এর field-গুলোর মানের সমান।

<Listing number="5-7" file-name="src/main.rs" caption="একটি `User` instance-এর জন্য নতুন `email` মান সেট করতে, কিন্তু বাকি মানগুলো `user1` থেকে নিতে struct update syntax ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-07/src/main.rs:here}}
```

</Listing>

Listing 5-7-এর কোডও `user2`-তে এমন একটি instance তৈরি করে, যার `email`-এর মান আলাদা,
কিন্তু `username`, `active` আর `sign_in_count` field-এর মান `user1`-এর মতোই। বাকি
সব field যে `user1`-এর সংশ্লিষ্ট field থেকে মান পাবে, তা বোঝাতে `..user1` সবার শেষে আসতে
হবে, তবে struct-এর সংজ্ঞায় field-গুলোর ক্রম যা-ই হোক, আমরা যত খুশি field-এর মান যেকোনো
ক্রমে নির্দিষ্ট করতে পারি।

লক্ষ করুন, struct update syntax একটি assignment-এর মতো `=` ব্যবহার করে; কারণ এটি ডেটা
move (ownership এক variable থেকে আরেকটিতে সরানো) করে, ঠিক যেমনটি আমরা ["ownership কী?"][move]<!-- ignore --> অংশে দেখেছি। এই উদাহরণে `user2` তৈরির পর `user1` আংশিকভাবে অবৈধ হয়ে যায়, কারণ `user1`-এর
`username` field-এর `String`-টি `user2`-তে move হয়েছে। আমরা যদি `user2`-কে `email` আর
`username` দুটোর জন্যই নতুন `String` মান দিতাম, এবং ফলে `user1` থেকে শুধু `active` আর
`sign_in_count`-এর মান ব্যবহার করতাম, তাহলে `user2` তৈরির পরও `user1` পুরোপুরি বৈধ থাকত। `active` আর `sign_in_count`-এর type এমন
type, যা `Copy` trait (টাইপের সাধারণ আচরণের সংজ্ঞা) implement করে, তাই
[“collection থেকে কপি বনাম move করা”][copy]<!-- ignore --> অংশে আলোচিত আচরণ এখানে প্রযোজ্য হবে।

<!-- Old headings. Do not remove or links may break. -->

<a id="using-tuple-structs-without-named-fields-to-create-different-types"></a>

### tuple struct দিয়ে আলাদা type তৈরি করা {#creating-different-types-with-tuple-structs}

Rust tuple-এর মতো দেখতে struct-ও সমর্থন করে, যেগুলোকে বলা হয় _tuple struct_ (field-এর
নাম ছাড়া tuple-এর মতো struct)। struct-এর নাম থেকে tuple struct বাড়তি অর্থ পায়, কিন্তু
এর field-গুলোর সঙ্গে কোনো নাম যুক্ত থাকে না; বরং থাকে শুধু field-গুলোর type। tuple struct
কাজে আসে যখন আপনি পুরো tuple-টিকে একটি নাম দিতে চান এবং tuple-টিকে অন্যান্য tuple থেকে
আলাদা একটি type বানাতে চান, আর যখন সাধারণ struct-এর মতো প্রতিটি field-এর নাম দেওয়া
অপ্রয়োজনীয় বা বাহুল্য হয়ে যায়।

tuple struct সংজ্ঞায়িত করতে `struct` keyword আর struct-এর নাম দিয়ে শুরু করুন, তারপর
tuple-এর type-গুলো লিখুন। যেমন, এখানে আমরা `Color` আর `Point` নামে দুটি tuple
struct সংজ্ঞায়িত করে ব্যবহার করছি:

<Listing file-name="src/main.rs">

```aquascope,interpreter
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);`[]`
}
```

</Listing>

লক্ষ করুন, `black` আর `origin` মান দুটির type আলাদা, কারণ এরা ভিন্ন ভিন্ন tuple struct-এর
instance। আপনার সংজ্ঞায়িত প্রতিটি struct নিজেই একটি আলাদা type, এমনকি struct-এর ভেতরের
field-গুলোর type একই হলেও। যেমন, `Color` type-এর parameter নেয় এমন একটি function
argument হিসেবে একটি `Point` নিতে পারে না, যদিও দুটি type-ই তিনটি `i32` মান দিয়ে গঠিত।
এছাড়া tuple struct-এর instance tuple-এর মতোই: এগুলোকে আলাদা আলাদা অংশে destructure করা
যায়, আর একটি `.` এবং তারপর index লিখে একটি একক মানে পৌঁছানো যায়। তবে tuple-এর বিপরীতে,
tuple struct destructure করার সময় struct-এর type-এর নাম লিখতে হয়। যেমন,
`origin` point-এর মানগুলোকে `x`, `y` আর `z` নামের variable-এ destructure করতে আমরা
লিখব `let Point(x, y, z) = origin;`।

<!-- Old headings. Do not remove or links may break. -->

<a id="unit-like-structs-without-any-fields"></a>

### unit-like struct সংজ্ঞায়িত করা {#defining-unit-like-structs}

কোনো field নেই এমন struct-ও সংজ্ঞায়িত করা যায়! এগুলোকে বলা হয় _unit-like struct_
(কোনো field নেই এমন struct), কারণ এরা `()`-এর মতো আচরণ করে, যা হলো
[“tuple type”][tuples]<!-- ignore --> অংশে উল্লেখ করা unit type। unit-like
struct কাজে আসতে পারে যখন কোনো type-এর ওপর একটি trait implement করার দরকার হয়, কিন্তু
type-টির ভেতরে রাখার মতো কোনো ডেটা আপনার নেই। trait নিয়ে আমরা অধ্যায় 10-এ আলোচনা
করব। এখানে `AlwaysEqual` নামের একটি unit struct ঘোষণা ও instantiate করার
একটি উদাহরণ:

```aquascope,interpreter
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;`[]`
}
```

`AlwaysEqual` সংজ্ঞায়িত করতে আমরা `struct` keyword, আমাদের পছন্দের নাম, আর তারপর একটি
সেমিকোলন ব্যবহার করি। কার্লি ব্র্যাকেট বা বন্ধনীর দরকার নেই! তারপর একই রকম উপায়ে
`subject` variable-এ `AlwaysEqual`-এর একটি instance পেতে পারি: সংজ্ঞায়িত নামটি ব্যবহার
করে, কোনো কার্লি ব্র্যাকেট বা বন্ধনী ছাড়া। কল্পনা করুন, পরে আমরা এই type-এর জন্য এমন
আচরণ implement করব, যাতে `AlwaysEqual`-এর প্রতিটি instance অন্য যেকোনো type-এর প্রতিটি
instance-এর সঙ্গে সবসময় সমান হয়, হয়তো testing-এর উদ্দেশ্যে একটি জানা ফল পাওয়ার জন্য।
সেই আচরণ implement করতে আমাদের কোনো ডেটার দরকার হতো না! অধ্যায় 10-এ দেখবেন কীভাবে
trait সংজ্ঞায়িত করে unit-like struct-সহ যেকোনো type-এর ওপর implement করতে হয়।

> ### struct-এর ডেটার ownership {#ownership-of-struct-data}
>
> Listing 5-1-এর `User` struct-এর সংজ্ঞায় আমরা `&str` string slice type-এর বদলে
> owned `String` type ব্যবহার করেছি। এটি একটি সচেতন সিদ্ধান্ত, কারণ আমরা চাই এই struct-এর
> প্রতিটি instance তার সব ডেটার owner হোক, আর পুরো struct যতক্ষণ বৈধ থাকে, সেই ডেটাও
> ততক্ষণ বৈধ থাকুক।
>
> struct-এর পক্ষে অন্য কিছুর মালিকানাধীন ডেটার reference রাখাও সম্ভব, তবে তা করতে হলে
> _lifetime_ (reference কতক্ষণ বৈধ থাকে) ব্যবহার করতে হয়, যা Rust-এর একটি বৈশিষ্ট্য এবং
> যা নিয়ে আমরা অধ্যায় 10-এ আলোচনা করব। lifetime নিশ্চিত করে যে struct যতক্ষণ বৈধ, struct-এর
> reference করা ডেটাও ততক্ষণ বৈধ। ধরুন আপনি lifetime নির্দিষ্ট না করে একটি struct-এ
> reference রাখার চেষ্টা করলেন, *src/main.rs*-এ নিচের মতো করে; এটি
> কাজ করবে না:
>
> <Listing file-name="src/main.rs">
>
> <!-- CAN'T EXTRACT SEE https://github.com/rust-lang/mdBook/issues/1127 -->
>
> ```rust,ignore,does_not_compile
> struct User {
>     active: bool,
>     username: &str,
>     email: &str,
>     sign_in_count: u64,
> }
>
> fn main() {
>     let user1 = User {
>         active: true,
>         username: "someusername123",
>         email: "someone@example.com",
>         sign_in_count: 1,
>     };
> }
> ```
>
> </Listing>
>
> compiler আপত্তি জানাবে যে এর lifetime specifier (lifetime নির্দেশক) দরকার:
>
> ```console
> $ cargo run
>    Compiling structs v0.1.0 (file:///projects/structs)
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:3:15
>   |
> 3 |     username: &str,
>   |               ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 ~     username: &'a str,
>   |
>
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:4:12
>   |
> 4 |     email: &str,
>   |            ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 |     username: &str,
> 4 ~     email: &'a str,
>   |
>
> For more information about this error, try `rustc --explain E0106`.
> error: could not compile `structs` (bin "structs") due to 2 previous errors
> ```
>
> অধ্যায় 10-এ আমরা আলোচনা করব কীভাবে এই error-গুলো ঠিক করে struct-এ reference রাখা যায়,
> তবে আপাতত আমরা এ ধরনের error ঠিক করব `&str`-এর মতো reference-এর বদলে `String`-এর মতো
> owned type (নিজের ডেটার মালিক এমন type) ব্যবহার করে।

### struct-এর field borrow করা {#borrowing-fields-of-a-struct}

["আলাদা tuple field"][differentfields]-এ আমাদের আলোচনার মতোই, Rust-এর borrow checker (borrow-এর নিয়ম যাচাই করা compiler-এর অংশ) struct-স্তরে এবং
field-স্তরে—দুই জায়গাতেই ownership permission (অনুমতি)-এর হিসাব রাখে। যেমন, আমরা যদি একটি `Point` structure-এর একটি field `x` borrow (ধার নেওয়া) করি, তাহলে `p` আর `p.x` দুটোই সাময়িকভাবে তাদের permission হারায় (কিন্তু `p.y` হারায় না):

```aquascope,permissions,stepper,boundaries
#fn main() {
struct Point { x: i32, y: i32 }

let mut p = Point { x: 0, y: 0 };`(focus,paths:p)`
let x = &mut p.x;`(focus,paths:p)`
*x += 1;`(focus,paths:p)`
println!("{}, {}", p.x, p.y);
#}
```

ফলে `p.x` এভাবে mutable-ভাবে borrow থাকা অবস্থায় যদি আমরা `p` ব্যবহারের চেষ্টা করি:

```aquascope,permissions,stepper,boundaries,shouldFail
struct Point { x: i32, y: i32 }

fn print_point(p: &Point) {
    println!("{}, {}", p.x, p.y);
}

fn main() {
    let mut p = Point { x: 0, y: 0 };`(focus,paths:p)`
    let x = &mut p.x;`(focus,paths:p)`
    print_point(&p);`{}`
    *x += 1;`(focus,paths:p)`
}
```

তাহলে compiler নিচের error দিয়ে আমাদের প্রোগ্রাম বাতিল করে দেবে:

```text
error[E0502]: cannot borrow `p` as immutable because it is also borrowed as mutable
  --> test.rs:10:17
   |
9  |     let x = &mut p.x;
   |             -------- mutable borrow occurs here
10 |     print_point(&p);
   |                 ^^ immutable borrow occurs here
11 |     *x += 1;
   |     ------- mutable borrow later used here
```

আরও সাধারণভাবে, struct জড়িত কোনো ownership error-এর মুখোমুখি হলে ভেবে দেখুন, আপনার structure-এর কোন field
কোন permission নিয়ে borrow হওয়ার কথা। তবে borrow checker-এর সীমাবদ্ধতা সম্পর্কে সচেতন থাকুন, কারণ Rust কখনো কখনো
আসলের চেয়ে বেশি field borrow হয়েছে বলে ধরে নিতে পারে।

{{#quiz ../quizzes/ch05-01-structs.toml}}


<!-- manual-regeneration
for the error above
after running update-rustc.sh:
pbcopy < listings/ch05-using-structs-to-structure-related-data/no-listing-02-reference-in-struct/output.txt
paste above
add `> ` before every line -->

[tuples]: ch03-02-data-types.html#the-tuple-type
[move]: ch04-01-what-is-ownership.html
[copy]: ch04-03-fixing-ownership-errors.html#fixing-an-unsafe-program-copying-vs-moving-out-of-a-collection
[differentfields]: ch04-03-fixing-ownership-errors.html#fixing-a-safe-program-mutating-different-tuple-fields
