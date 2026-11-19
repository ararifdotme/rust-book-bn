## module-গুলোকে আলাদা ফাইলে ভাগ করা {#separating-modules-into-different-files}

এ পর্যন্ত এই অধ্যায়ের সব উদাহরণে একটি ফাইলেই একাধিক module সংজ্ঞায়িত ছিল। module বড় হয়ে গেলে কোডে
ঘোরাফেরা সহজ করতে আপনি হয়তো তাদের সংজ্ঞা আলাদা ফাইলে সরিয়ে নিতে চাইবেন।

যেমন, চলুন Listing 7-17-এর কোড থেকে শুরু করি, যাতে রেস্তোরাঁর একাধিক module ছিল। সব module crate
root ফাইলে সংজ্ঞায়িত রাখার বদলে আমরা module-গুলোকে আলাদা ফাইলে বের করে আনব। এই ক্ষেত্রে crate root
ফাইল হলো _src/lib.rs_, তবে এই পদ্ধতি এমন binary crate-এর ক্ষেত্রেও কাজ করে, যার crate root ফাইল
_src/main.rs_।

প্রথমে আমরা `front_of_house` module-টিকে তার নিজস্ব ফাইলে বের করে আনব। `front_of_house` module-এর
কার্লি ব্র্যাকেটের ভেতরের কোড মুছে শুধু `mod front_of_house;` ঘোষণাটি রেখে দিন, যাতে _src/lib.rs_-এ
Listing 7-21-এ দেখানো কোডটি থাকে। লক্ষ করুন, Listing 7-22-এর _src/front_of_house.rs_ ফাইল তৈরি না
করা পর্যন্ত এটি compile হবে না।

<Listing number="7-21" file-name="src/lib.rs" caption="`front_of_house` module ঘোষণা করা, যার body থাকবে *src/front_of_house.rs*-এ">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/lib.rs}}
```

</Listing>

এরপর কার্লি ব্র্যাকেটের ভেতরে যে কোড ছিল, তা _src/front_of_house.rs_ নামের একটি নতুন ফাইলে রাখুন,
Listing 7-22-এ যেমন দেখানো হয়েছে। compiler জানে এই ফাইলে খুঁজতে হবে, কারণ crate root-এ সে
`front_of_house` নামের module ঘোষণাটি পেয়েছে।

<Listing number="7-22" file-name="src/front_of_house.rs" caption="*src/front_of_house.rs*-এ `front_of_house` module-এর ভেতরের সংজ্ঞাগুলো">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/front_of_house.rs}}
```

</Listing>

লক্ষ করুন, আপনার module tree-তে একটি `mod` ঘোষণা দিয়ে একটি ফাইল শুধু _একবারই_ load করতে হয়। একবার
compiler জেনে গেলে যে ফাইলটি প্রজেক্টের অংশ (আর আপনি `mod` statement যেখানে রেখেছেন তার কারণে কোডটি
module tree-র কোথায় আছে তা-ও জেনে গেলে), আপনার প্রজেক্টের অন্য ফাইলগুলোর উচিত load হওয়া ফাইলের কোড
উল্লেখ করতে, যেখানে এটি ঘোষণা করা হয়েছে, সেখানকার একটি path ব্যবহার করা, যা [“module tree-র কোনো item
উল্লেখ করার path”][paths]<!-- ignore --> অংশে আলোচনা করা হয়েছে। অন্যভাবে বললে, `mod` এমন কোনো “include”
operation _নয়_, যা হয়তো আপনি অন্য প্রোগ্রামিং ভাষায় দেখেছেন।

এরপর আমরা `hosting` module-টিকে তার নিজস্ব ফাইলে বের করে আনব। প্রক্রিয়াটি একটু আলাদা, কারণ `hosting`
হলো `front_of_house`-এর child module, root module-এর নয়। `hosting`-এর ফাইলটি আমরা রাখব একটি নতুন
ডিরেক্টরিতে, যার নাম হবে module tree-তে এর ancestor-দের নাম অনুযায়ী, এই ক্ষেত্রে _src/front_of_house_।

`hosting` সরানো শুরু করতে আমরা _src/front_of_house.rs_ বদলে এমন করি, যাতে এতে শুধু `hosting` module-এর
ঘোষণাটি থাকে:

<Listing file-name="src/front_of_house.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house.rs}}
```

</Listing>

তারপর `hosting` module-এ করা সংজ্ঞাগুলো রাখতে আমরা একটি _src/front_of_house_ ডিরেক্টরি আর একটি
_hosting.rs_ ফাইল তৈরি করি:

<Listing file-name="src/front_of_house/hosting.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house/hosting.rs}}
```

</Listing>

এর বদলে _hosting.rs_-কে যদি _src_ ডিরেক্টরিতে রাখতাম, তাহলে compiler আশা করত _hosting.rs_-এর কোড
crate root-এ ঘোষিত একটি `hosting` module-এর কোড, `front_of_house` module-এর child হিসেবে ঘোষিত নয়।
কোন module-এর কোডের জন্য কোন ফাইল যাচাই করা হবে, compiler-এর সেই নিয়মগুলোর ফলে ডিরেক্টরি আর ফাইলগুলো
module tree-র সঙ্গে আরও ঘনিষ্ঠভাবে মেলে।

> ### বিকল্প ফাইল path {#alternate-file-paths}
>
> এ পর্যন্ত আমরা Rust compiler-এর ব্যবহৃত সবচেয়ে Rust-সুলভ ফাইল path-গুলো দেখেছি, তবে Rust ফাইল path-এর
> একটি পুরোনো ধরনও সমর্থন করে। crate root-এ ঘোষিত `front_of_house` নামের একটি module-এর জন্য compiler
> module-টির কোড খুঁজবে:
>
> - _src/front_of_house.rs_-এ (যা আমরা দেখেছি)
> - _src/front_of_house/mod.rs_-এ (পুরোনো ধরন, এখনো সমর্থিত path)
>
> `front_of_house`-এর submodule `hosting` নামের একটি module-এর জন্য compiler module-টির কোড
> খুঁজবে:
>
> - _src/front_of_house/hosting.rs_-এ (যা আমরা দেখেছি)
> - _src/front_of_house/hosting/mod.rs_-এ (পুরোনো ধরন, এখনো সমর্থিত path)
>
> একই module-এর জন্য দুটি ধরনই ব্যবহার করলে compiler error পাবেন। একই প্রজেক্টের আলাদা module-এর জন্য
> দুটি ধরন মিশিয়ে ব্যবহারের অনুমতি আছে, তবে আপনার প্রজেক্টে ঘোরাফেরা করা মানুষের কাছে তা বিভ্রান্তিকর
> হতে পারে।
>
> _mod.rs_ নামের ফাইল ব্যবহার করা ধরনের প্রধান অসুবিধা হলো, আপনার প্রজেক্টে _mod.rs_ নামের অনেক ফাইল
> জমে যেতে পারে, যা এডিটরে একসঙ্গে খোলা থাকলে বিভ্রান্তিকর হয়ে যায়।

আমরা প্রতিটি module-এর কোড একটি আলাদা ফাইলে সরিয়েছি, আর module tree একই আছে। `eat_at_restaurant`-এর
function call-গুলো কোনো পরিবর্তন ছাড়াই কাজ করবে, যদিও সংজ্ঞাগুলো এখন আলাদা ফাইলে থাকে। এই কৌশলে
module আকারে বড় হতে থাকলে সেগুলোকে নতুন ফাইলে সরিয়ে নিতে পারেন।

লক্ষ করুন, _src/lib.rs_-এর `pub use crate::front_of_house::hosting` statement-ও বদলায়নি, আর crate-এর অংশ
হিসেবে কোন ফাইলগুলো compile হবে, তাতে `use`-এর কোনো প্রভাবও নেই। `mod` keyword module ঘোষণা করে, আর
Rust সেই module-এ যাওয়া কোড খোঁজে module-এর নামের সমান নামের একটি ফাইলে।

{{#quiz ../quizzes/ch07-05-files.toml}}

## সারসংক্ষেপ {#summary}

Rust আপনাকে একটি package-কে একাধিক crate-এ আর একটি crate-কে module-এ ভাগ করতে দেয়, যাতে এক module-এ
সংজ্ঞায়িত item অন্য module থেকে উল্লেখ করা যায়। absolute বা relative path লিখে আপনি তা করতে পারেন। এই
path-গুলো একটি `use` statement দিয়ে scope-এ আনা যায়, যাতে ওই scope-এ item-টির একাধিক ব্যবহারের জন্য
একটি ছোট path ব্যবহার করতে পারেন। module-এর কোড ডিফল্টভাবে private, তবে `pub` keyword যোগ করে সংজ্ঞাগুলো
public করতে পারেন।

পরের অধ্যায়ে আমরা standard library-র কিছু collection data structure দেখব, যা আপনার সুন্দরভাবে গোছানো
কোডে ব্যবহার করতে পারবেন।

[paths]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
