# smart pointer {#smart-pointers}

pointer হলো এমন variable-এর একটি সাধারণ ধারণা, যাতে মেমোরির একটি ঠিকানা থাকে। এই ঠিকানা অন্য কোনো ডেটাকে নির্দেশ করে, বা সেটির দিকে “point করে”। Rust-এ
সবচেয়ে প্রচলিত pointer হলো reference, যা আপনি অধ্যায় 4-এ শিখেছেন। reference বোঝানো হয় `&` চিহ্ন দিয়ে, আর এরা যে মানকে নির্দেশ করে তা borrow করে।
ডেটাকে নির্দেশ করা ছাড়া এদের আর কোনো বিশেষ ক্ষমতা নেই, এবং এদের কোনো বাড়তি খরচও নেই।

অন্যদিকে _Smart pointers_ (smart pointer) হলো এমন data structure, যা pointer-এর মতো আচরণ করে, কিন্তু সঙ্গে বাড়তি মেটাডেটা ও ক্ষমতাও রাখে। smart
pointer-এর ধারণা শুধু Rust-এর নয়: smart pointer-এর উৎপত্তি C++-এ, আর অন্য ভাষাতেও এটি আছে। Rust-এর standard library-তে নানা রকম smart pointer সংজ্ঞায়িত
আছে, যা reference-এর দেওয়া কার্যক্ষমতার চেয়েও বেশি কিছু দেয়। সাধারণ ধারণাটি বুঝতে আমরা smart pointer-এর কয়েকটি ভিন্ন উদাহরণ দেখব, যার মধ্যে আছে একটি
_reference counting_ (কতগুলো reference আছে গুনে মেমোরি ফেরতের ব্যবস্থা) smart pointer type। এই pointer owner-এর সংখ্যা হিসাব রেখে ডেটার একাধিক owner
থাকতে দেয়, আর কোনো owner বাকি না থাকলে ডেটা পরিষ্কার করে।

ownership আর borrowing-এর ধারণার কারণে Rust-এ reference আর smart pointer-এর মধ্যে আরেকটি পার্থক্য আছে: reference শুধু ডেটা borrow করে, কিন্তু অনেক
ক্ষেত্রে smart pointer যে ডেটাকে নির্দেশ করে, তার _মালিক_ হয়।

smart pointer সাধারণত struct দিয়ে implement করা হয়। সাধারণ struct-এর মতো নয়, smart pointer `Deref` আর `Drop` trait implement করে। `Deref` trait smart
pointer struct-এর একটি instance-কে reference-এর মতো আচরণ করতে দেয়, যাতে আপনি এমন কোড লিখতে পারেন যা reference বা smart pointer, দুটির সঙ্গেই কাজ করে।
`Drop` trait দিয়ে আপনি নিজের মতো করে ঠিক করতে পারেন smart pointer-এর একটি instance scope-এর বাইরে গেলে কোন কোড চলবে। এই অধ্যায়ে আমরা এই দুটি trait নিয়েই
আলোচনা করব এবং দেখাব smart pointer-এর জন্য এরা কেন গুরুত্বপূর্ণ।

smart pointer pattern যেহেতু Rust-এ প্রায়ই ব্যবহৃত একটি সাধারণ design pattern, তাই এই অধ্যায়ে বিদ্যমান সব smart pointer আলোচনা করা হবে না। অনেক library-র
নিজস্ব smart pointer আছে, আর আপনি নিজেও লিখতে পারেন। আমরা standard library-র সবচেয়ে প্রচলিত smart pointer-গুলো দেখব:

- `Box<T>`, heap-এ মান allocate করার জন্য
- `Rc<T>`, একটি reference counting type, যা একাধিক ownership সম্ভব করে
- `Ref<T>` আর `RefMut<T>`, যা পাওয়া যায় `RefCell<T>`-এর মাধ্যমে, যা এমন একটি type, যেটি borrowing-এর নিয়ম compile time-এর বদলে runtime-এ
  প্রয়োগ করে

এছাড়া আমরা _interior mutability_ (ভেতরের মান পরিবর্তনযোগ্যতা) pattern দেখব, যেখানে একটি immutable type ভেতরের একটি মান পরিবর্তন করার জন্য একটি API
প্রকাশ করে। reference cycle (একে অপরকে নির্দেশ করা reference-এর চক্র) নিয়েও আলোচনা করব: এগুলো কীভাবে মেমোরি leak করতে পারে এবং কীভাবে এগুলো ঠেকানো
যায়।

চলুন শুরু করা যাক!
