## module tree-র কোনো item উল্লেখ করার path {#paths-for-referring-to-an-item-in-the-module-tree}

module tree-তে একটি item কোথায় পাওয়া যাবে তা Rust-কে দেখাতে আমরা একটি path ব্যবহার করি, ঠিক যেমন
ফাইল সিস্টেমে ঘোরাফেরার সময় path ব্যবহার করি। একটি function call করতে হলে তার path জানতে হবে।

একটি path দুটি রূপ নিতে পারে:

- একটি _absolute path_ (crate root থেকে শুরু হওয়া পূর্ণ পথ) হলো crate root থেকে শুরু হওয়া পুরো
  path; বাইরের কোনো crate-এর কোডের জন্য absolute path শুরু হয় crate-এর নাম দিয়ে, আর বর্তমান
  crate-এর কোডের জন্য এটি শুরু হয় literal `crate` দিয়ে।
- একটি _relative path_ (বর্তমান module থেকে শুরু হওয়া পথ) শুরু হয় বর্তমান module থেকে, আর
  `self`, `super` বা বর্তমান module-এর কোনো identifier ব্যবহার করে।

absolute আর relative দুই ধরনের path-এর পরেই থাকে দুটি কোলন (`::`) দিয়ে আলাদা করা এক বা একাধিক
identifier।

Listing 7-1-এ ফিরে গিয়ে, ধরুন আমরা `add_to_waitlist` function call করতে চাই। এটি এই প্রশ্ন করার
সমান: `add_to_waitlist` function-এর path কী? Listing 7-3-এ Listing 7-1-ই আছে, তবে কিছু module আর
function সরিয়ে দেওয়া হয়েছে।

crate root-এ সংজ্ঞায়িত একটি নতুন function `eat_at_restaurant` থেকে `add_to_waitlist` function call করার
দুটি উপায় আমরা দেখাব। এই path-গুলো সঠিক, কিন্তু আরেকটি সমস্যা থেকে যায়, যার কারণে এই উদাহরণটি
এভাবে compile হবে না। একটু পরেই আমরা এর কারণ ব্যাখ্যা করব।

`eat_at_restaurant` function আমাদের library crate-এর public API-র অংশ, তাই আমরা এটিকে `pub` keyword
দিয়ে চিহ্নিত করি। [“`pub` keyword দিয়ে path
প্রকাশ করা”][pub]<!-- ignore --> অংশে আমরা `pub` নিয়ে আরও বিস্তারিত আলোচনা করব।

<Listing number="7-3" file-name="src/lib.rs" caption="absolute আর relative path ব্যবহার করে `add_to_waitlist` function call করা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-03/src/lib.rs}}
```

</Listing>

`eat_at_restaurant`-এ প্রথমবার `add_to_waitlist` function call করার সময় আমরা একটি absolute path
ব্যবহার করি। `add_to_waitlist` function `eat_at_restaurant`-এর একই crate-এ সংজ্ঞায়িত, অর্থাৎ absolute
path শুরু করতে আমরা `crate` keyword ব্যবহার করতে পারি। তারপর `add_to_waitlist`-এ পৌঁছানো পর্যন্ত
পরপর প্রতিটি module লিখি। একই কাঠামোর একটি ফাইল সিস্টেম কল্পনা করতে পারেন: `add_to_waitlist` প্রোগ্রাম
চালাতে আমরা `/front_of_house/hosting/add_to_waitlist` path লিখতাম; crate root থেকে শুরু করতে `crate`
নাম ব্যবহার করা মানে শেলে ফাইল সিস্টেমের root থেকে শুরু করতে `/` ব্যবহার করার মতো।

`eat_at_restaurant`-এ দ্বিতীয়বার `add_to_waitlist` call করার সময় আমরা একটি relative path ব্যবহার
করি। path-টি শুরু হয় `front_of_house` দিয়ে, যা module tree-তে `eat_at_restaurant`-এর একই স্তরে
সংজ্ঞায়িত module-এর নাম। এখানে ফাইল সিস্টেমের সমতুল্য হতো
`front_of_house/hosting/add_to_waitlist` path ব্যবহার করা। একটি module-এর নাম দিয়ে শুরু করা মানে
path-টি relative।

relative নাকি absolute path ব্যবহার করবেন, সেই সিদ্ধান্ত আপনি নেবেন আপনার প্রজেক্টের ভিত্তিতে, আর তা
নির্ভর করে item-এর সংজ্ঞার কোড আর item ব্যবহারকারী কোডকে আপনি আলাদাভাবে নাকি একসঙ্গে সরানোর
সম্ভাবনা বেশি, তার ওপর। যেমন, আমরা যদি `front_of_house` module আর `eat_at_restaurant` function-কে
`customer_experience` নামের একটি module-এ সরিয়ে নিতাম, তাহলে `add_to_waitlist`-এর absolute path
আপডেট করতে হতো, কিন্তু relative path তবু বৈধ থাকত। আবার, আমরা যদি শুধু `eat_at_restaurant` function-কে
আলাদাভাবে `dining` নামের একটি module-এ সরাতাম, তাহলে `add_to_waitlist` call-এর absolute path একই
থাকত, কিন্তু relative path আপডেট করতে হতো। সাধারণভাবে আমাদের পছন্দ হলো absolute path ব্যবহার করা,
কারণ কোডের সংজ্ঞা আর item call-গুলো পরস্পর থেকে স্বাধীনভাবে সরানোর সম্ভাবনাই বেশি।

চলুন Listing 7-3 compile করার চেষ্টা করে দেখি এটি এখনো কেন compile হবে না! আমরা যে error পাই, তা
Listing 7-4-এ দেখানো হয়েছে।

<Listing number="7-4" caption="Listing 7-3-এর কোড build করার সময় পাওয়া compiler error">

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-03/output.txt}}
```

</Listing>

error message বলছে যে module `hosting` private। অন্যভাবে বললে, `hosting` module আর `add_to_waitlist`
function-এর সঠিক path আমাদের কাছে আছে, কিন্তু Rust আমাদের সেগুলো ব্যবহার করতে দেবে না, কারণ private
অংশগুলোতে এর প্রবেশাধিকার নেই। Rust-এ সব item (function, method, struct, enum, module আর constant)
ডিফল্টভাবে parent module-গুলোর কাছে private। একটি function বা struct-এর মতো item-কে private করতে চাইলে
সেটিকে একটি module-এ রাখুন।

parent module-এর item-গুলো child module-এর ভেতরের private item ব্যবহার করতে পারে না, কিন্তু child
module-এর item-গুলো তাদের ancestor (ওপরের যেকোনো স্তরের) module-এর item ব্যবহার করতে পারে। এর কারণ হলো,
child module তাদের implementation-এর খুঁটিনাটি মুড়ে লুকিয়ে রাখে, কিন্তু child module যে প্রেক্ষাপটে
সংজ্ঞায়িত, তা দেখতে পায়। আমাদের রূপক চালিয়ে গেলে, privacy-র নিয়মগুলোকে একটি রেস্তোরাঁর পেছনের
অফিসের মতো ভাবুন: সেখানে যা ঘটে, তা রেস্তোরাঁর খদ্দেরদের কাছে private, কিন্তু অফিসের ম্যানেজাররা
তাদের চালানো রেস্তোরাঁর সবকিছু দেখতে আর করতে পারেন।

Rust module system এভাবে কাজ করার পথ বেছে নিয়েছে, যাতে ভেতরের implementation-এর খুঁটিনাটি লুকিয়ে
রাখাই হয় ডিফল্ট। এভাবে আপনি জানেন ভেতরের কোডের কোন অংশ বাইরের কোড না ভেঙে বদলাতে পারবেন। তবে
Rust আপনাকে `pub` keyword ব্যবহার করে একটি item public করে child module-এর কোডের ভেতরের অংশ বাইরের
ancestor module-এর কাছে প্রকাশ করার সুযোগও দেয়।

### `pub` keyword দিয়ে path প্রকাশ করা {#exposing-paths-with-the-pub-keyword}

Listing 7-4-এর সেই error-এ ফিরে যাই, যা আমাদের বলেছিল `hosting` module private। আমরা চাই parent
module-এর `eat_at_restaurant` function child module-এর `add_to_waitlist` function-এ প্রবেশাধিকার পাক,
তাই আমরা `hosting` module-কে `pub` keyword দিয়ে চিহ্নিত করি, Listing 7-5-এ যেমন দেখানো হয়েছে।

<Listing number="7-5" file-name="src/lib.rs" caption="`eat_at_restaurant` থেকে ব্যবহার করতে `hosting` module-কে `pub` হিসেবে ঘোষণা করা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-05/src/lib.rs:here}}
```

</Listing>

দুর্ভাগ্যবশত, Listing 7-5-এর কোডেও এখনো compiler error আসে, Listing 7-6-এ যেমন
দেখানো হয়েছে।

<Listing number="7-6" caption="Listing 7-5-এর কোড build করার সময় পাওয়া compiler error">

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-05/output.txt}}
```

</Listing>

কী হলো? `mod hosting`-এর সামনে `pub` keyword যোগ করায় module-টি public হয়েছে। এই পরিবর্তনের ফলে
আমরা `front_of_house`-এ পৌঁছাতে পারলে `hosting`-এও পৌঁছাতে পারি। কিন্তু `hosting`-এর _বিষয়বস্তু_
এখনো private; module public করলেই তার বিষয়বস্তু public হয় না। একটি module-এর ওপর `pub` keyword শুধু
তার ancestor module-এর কোডকে module-টি উল্লেখ করতে দেয়, তার ভেতরের কোডে পৌঁছাতে দেয় না। module
যেহেতু ধারক, তাই শুধু module public করে খুব বেশি কিছু করা যায় না; আমাদের আরও এগিয়ে module-এর ভেতরের
এক বা একাধিক item-কেও public করতে হবে।

Listing 7-6-এর error-গুলো বলছে যে `add_to_waitlist` function private। privacy-র নিয়ম module-এর পাশাপাশি
struct, enum, function আর method-এর ক্ষেত্রেও প্রযোজ্য।

চলুন `add_to_waitlist` function-এর সংজ্ঞার আগে `pub` keyword যোগ করে এটিকেও public করি, Listing
7-7-এর মতো।

<Listing number="7-7" file-name="src/lib.rs" caption="`mod hosting` আর `fn add_to_waitlist`-এ `pub` keyword যোগ করলে আমরা `eat_at_restaurant` থেকে function-টি call করতে পারি।">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-07/src/lib.rs:here}}
```

</Listing>

এবার কোডটি compile হবে! privacy-র নিয়মের আলোকে `pub` keyword যোগ করায় কেন `eat_at_restaurant`-এ এই
path-গুলো ব্যবহার করা যায়, তা দেখতে চলুন absolute আর relative path দুটো দেখি।

absolute path-এ আমরা শুরু করি `crate` দিয়ে, যা আমাদের crate-এর module tree-র মূল। `front_of_house`
module crate root-এ সংজ্ঞায়িত। `front_of_house` public না হলেও, যেহেতু `eat_at_restaurant` function
`front_of_house`-এর একই module-এ সংজ্ঞায়িত (অর্থাৎ `eat_at_restaurant` আর `front_of_house` sibling),
তাই `eat_at_restaurant` থেকে আমরা `front_of_house` উল্লেখ করতে পারি। এরপর আসে `pub` দিয়ে চিহ্নিত
`hosting` module। `hosting`-এর parent module-এ আমরা পৌঁছাতে পারি, তাই `hosting`-এও পৌঁছাতে পারি।
সবশেষে, `add_to_waitlist` function `pub` দিয়ে চিহ্নিত, আর এর parent module-এ আমরা পৌঁছাতে পারি, তাই এই
function call কাজ করে!

relative path-এ প্রথম ধাপ ছাড়া বাকি যুক্তি absolute path-এর মতোই: crate root থেকে শুরু করার বদলে
path-টি শুরু হয় `front_of_house` থেকে। `front_of_house` module `eat_at_restaurant`-এর একই module-এ
সংজ্ঞায়িত, তাই যে module-এ `eat_at_restaurant` সংজ্ঞায়িত, সেখান থেকে শুরু হওয়া relative path কাজ করে।
তারপর `hosting` আর `add_to_waitlist` যেহেতু `pub` দিয়ে চিহ্নিত, তাই path-এর বাকিটাও কাজ করে, এবং এই
function call বৈধ!

আপনি যদি আপনার library crate শেয়ার করার পরিকল্পনা করেন, যাতে অন্য প্রজেক্ট আপনার কোড ব্যবহার করতে
পারে, তাহলে আপনার public API হলো আপনার crate-এর ব্যবহারকারীদের সঙ্গে আপনার চুক্তি, যা নির্ধারণ করে
তারা আপনার কোডের সঙ্গে কীভাবে কাজ করতে পারবে। আপনার crate-এর ওপর মানুষের নির্ভর করা সহজ করতে public
API-র পরিবর্তন সামলানো নিয়ে অনেক বিবেচনার বিষয় আছে। এই বিবেচনাগুলো এই বইয়ের পরিধির বাইরে; এই বিষয়ে
আগ্রহী হলে দেখুন [Rust API Guidelines][api-guidelines]।

> #### binary আর library দুটোই আছে এমন package-এর জন্য ভালো চর্চা {#best-practices-for-packages-with-a-binary-and-a-library}
>
> আমরা বলেছিলাম, একটি package-এ একটি _src/main.rs_ binary crate root আর একটি _src/lib.rs_ library
> crate root দুটোই থাকতে পারে, আর ডিফল্টভাবে দুটি crate-এরই নাম হবে package-এর নাম। সাধারণত library
> আর binary crate দুটোই থাকে এমন package-এর binary crate-এ শুধু ততটুকু কোড থাকে, যতটুকু একটি
> executable শুরু করে library crate-এ সংজ্ঞায়িত কোড call করতে দরকার। এতে অন্য প্রজেক্টগুলো package-এর
> দেওয়া কার্যকারিতার সর্বোচ্চ সুবিধা পায়, কারণ library crate-এর কোড শেয়ার করা যায়।
>
> module tree সংজ্ঞায়িত করা উচিত _src/lib.rs_-এ। তাহলে package-এর নাম দিয়ে path শুরু করে binary crate-এ
> যেকোনো public item ব্যবহার করা যায়। binary crate তখন library crate-এর একজন ব্যবহারকারী হয়ে যায়,
> ঠিক যেভাবে একটি সম্পূর্ণ বাইরের crate library crate ব্যবহার করত: এটি শুধু public API ব্যবহার করতে
> পারে। এতে একটি ভালো API নকশা করতে সুবিধা হয়; আপনি শুধু লেখক নন, একজন ব্যবহারকারীও!
>
> [অধ্যায় 12][ch12]<!-- ignore -->-এ আমরা একটি কমান্ড লাইন প্রোগ্রাম দিয়ে এই সাংগঠনিক চর্চা দেখাব,
> যাতে একটি binary crate আর একটি library crate দুটোই থাকবে।

{{#quiz ../quizzes/ch07-03-paths-sec1.toml}}

### `super` দিয়ে relative path শুরু করা {#starting-relative-paths-with-super}

path-এর শুরুতে `super` ব্যবহার করে আমরা এমন relative path তৈরি করতে পারি, যা বর্তমান module বা crate
root-এর বদলে parent module থেকে শুরু হয়। এটি ফাইল সিস্টেমের path `..` syntax দিয়ে শুরু করার মতো,
যার মানে parent ডিরেক্টরিতে যাওয়া। `super` ব্যবহার করে আমরা এমন একটি item উল্লেখ করতে পারি, যা আমরা
জানি parent module-এ আছে; module-টি parent-এর সঙ্গে ঘনিষ্ঠভাবে সম্পর্কিত হলে, কিন্তু parent হয়তো কোনো
একদিন module tree-র অন্য কোথাও সরানো হতে পারে এমন হলে, এতে module tree নতুন করে সাজানো সহজ হয়।

Listing 7-8-এর কোডটি দেখুন, যা এমন একটি পরিস্থিতির মডেল, যেখানে একজন শেফ একটি ভুল অর্ডার ঠিক করে
নিজেই সেটি খদ্দেরের কাছে নিয়ে আসেন। `back_of_house` module-এ সংজ্ঞায়িত `fix_incorrect_order` function
`super` দিয়ে শুরু করে `deliver_order`-এর path লিখে parent module-এ সংজ্ঞায়িত `deliver_order` function
call করে।

<Listing number="7-8" file-name="src/lib.rs" caption="`super` দিয়ে শুরু হওয়া একটি relative path ব্যবহার করে একটি function call করা">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-08/src/lib.rs}}
```

</Listing>

`fix_incorrect_order` function আছে `back_of_house` module-এ, তাই `back_of_house`-এর parent module-এ
যেতে আমরা `super` ব্যবহার করতে পারি, যা এই ক্ষেত্রে `crate`, অর্থাৎ root। সেখান থেকে আমরা
`deliver_order` খুঁজি এবং পেয়ে যাই। সফল! আমাদের মনে হয়, `back_of_house` module আর `deliver_order`
function সম্ভবত পরস্পরের সঙ্গে একই সম্পর্কে থাকবে, আর crate-এর module tree নতুন করে সাজানোর সিদ্ধান্ত
নিলে একসঙ্গেই সরানো হবে। তাই আমরা `super` ব্যবহার করেছি, যাতে ভবিষ্যতে এই কোড অন্য module-এ সরানো
হলে কোড আপডেট করার জায়গা কম থাকে।

### struct আর enum public করা {#making-structs-and-enums-public}

struct আর enum-কে public হিসেবে চিহ্নিত করতেও আমরা `pub` ব্যবহার করতে পারি, তবে struct আর enum-এর
সঙ্গে `pub` ব্যবহারে কয়েকটি বাড়তি খুঁটিনাটি আছে। একটি struct-এর সংজ্ঞার আগে `pub` ব্যবহার করলে
struct-টি public হয়, কিন্তু struct-এর field-গুলো তবু private থাকে। প্রতিটি field আলাদাভাবে, প্রয়োজন
অনুযায়ী, public করা বা না করা যায়। Listing 7-9-এ আমরা একটি public `back_of_house::Breakfast` struct
সংজ্ঞায়িত করেছি, যার `toast` field public কিন্তু `seasonal_fruit` field private। এটি রেস্তোরাঁর এমন
একটি পরিস্থিতির মডেল, যেখানে খদ্দের খাবারের সঙ্গে আসা রুটির ধরন বেছে নিতে পারেন, কিন্তু কোন মৌসুমে
কী পাওয়া যায় আর মজুত কী আছে, তার ভিত্তিতে খাবারের সঙ্গে কোন ফল যাবে তা শেফ ঠিক করেন। পাওয়া যায় এমন
ফল দ্রুত বদলায়, তাই খদ্দেররা ফল বেছে নিতে পারেন না, এমনকি কোন ফল পাবেন তা-ও দেখতে পারেন না।

<Listing number="7-9" file-name="src/lib.rs" caption="একটি struct, যার কিছু field public আর কিছু field private">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-09/src/lib.rs}}
```

</Listing>

`back_of_house::Breakfast` struct-এর `toast` field যেহেতু public, তাই `eat_at_restaurant`-এ dot notation
ব্যবহার করে আমরা `toast` field-এ লিখতে আর পড়তে পারি। লক্ষ করুন, `eat_at_restaurant`-এ আমরা
`seasonal_fruit` field ব্যবহার করতে পারি না, কারণ `seasonal_fruit` private। `seasonal_fruit` field-এর
মান বদলানোর লাইনটির কমেন্ট তুলে দিয়ে দেখুন কী error পান!

আরও লক্ষ করুন, `back_of_house::Breakfast`-এর একটি private field থাকায় struct-টিকে একটি public associated
function দিতে হবে, যা `Breakfast`-এর একটি instance তৈরি করে (এখানে আমরা এর নাম দিয়েছি `summer`)।
`Breakfast`-এর এমন কোনো function না থাকলে আমরা `eat_at_restaurant`-এ `Breakfast`-এর instance তৈরি
করতে পারতাম না, কারণ `eat_at_restaurant`-এ private `seasonal_fruit` field-এর মান সেট করতে পারতাম না।

অন্যদিকে, একটি enum public করলে তার সব variant-ই public হয়ে যায়। Listing 7-10-এ যেমন দেখানো হয়েছে,
শুধু `enum` keyword-এর আগে `pub` দিলেই চলে।

<Listing number="7-10" file-name="src/lib.rs" caption="একটি enum-কে public হিসেবে চিহ্নিত করলে তার সব variant public হয়ে যায়।">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-10/src/lib.rs}}
```

</Listing>

`Appetizer` enum-টিকে public করায় `eat_at_restaurant`-এ আমরা `Soup` আর `Salad` variant ব্যবহার
করতে পারি।

variant-গুলো public না হলে enum খুব একটা কাজের নয়; প্রতিটি ক্ষেত্রে সব enum variant-কে `pub` দিয়ে
annotate করতে হলে বিরক্তিকর হতো, তাই enum variant-এর ডিফল্ট হলো public হওয়া। struct প্রায়ই field
public না করেও কাজের, তাই struct-এর field সাধারণ নিয়ম মেনে চলে: `pub` দিয়ে annotate না করা পর্যন্ত
সবকিছু ডিফল্টভাবে private।

`pub` জড়িত আরও একটি পরিস্থিতি আছে, যা আমরা এখনো আলোচনা করিনি, আর সেটি আমাদের module system-এর শেষ
বৈশিষ্ট্য: `use` keyword। আমরা প্রথমে আলাদাভাবে `use` নিয়ে আলোচনা করব, তারপর দেখাব কীভাবে `pub` আর
`use` একসঙ্গে ব্যবহার করা যায়।

{{#quiz ../quizzes/ch07-03-paths-sec2.toml}}

[pub]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#exposing-paths-with-the-pub-keyword
[api-guidelines]: https://rust-lang.github.io/api-guidelines/
[ch12]: ch12-00-an-io-project.html
