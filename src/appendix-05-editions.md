## পরিশিষ্ট E: edition {#appendix-e-editions}

অধ্যায় 1-এ দেখেছেন, `cargo new` আপনার _Cargo.toml_ ফাইলে edition সম্পর্কে একটু metadata যোগ করে। এর মানে কী, তা নিয়েই এই পরিশিষ্ট!

Rust ভাষা ও compiler-এর release চক্র ছয় সপ্তাহের, অর্থাৎ ব্যবহারকারীরা নতুন feature-এর একটি অবিরাম ধারা পান। অন্য programming language-গুলো বড় পরিবর্তন কম ঘন ঘন release করে; Rust ছোট হালনাগাদ আরও ঘন ঘন release করে। কিছুদিন পর এই সব ছোট ছোট পরিবর্তন জমে বড় হয়ে ওঠে। কিন্তু এক release থেকে আরেক release-এ পেছন ফিরে তাকিয়ে বলা কঠিন হতে পারে, “বাহ, Rust 1.10 থেকে Rust 1.31-এর মধ্যে Rust অনেক বদলে গেছে!”

মোটামুটি প্রতি তিন বছর পরপর Rust team একটি নতুন Rust _edition_ তৈরি করে। প্রতিটি edition এর মধ্যে যুক্ত হওয়া feature-গুলোকে পুরোপুরি হালনাগাদ documentation ও tooling-সহ একটি স্পষ্ট প্যাকেজে একত্র করে। নতুন edition স্বাভাবিক ছয় সপ্তাহের release প্রক্রিয়ার অংশ হিসেবেই আসে।

edition বিভিন্ন মানুষের কাছে বিভিন্ন উদ্দেশ্য পূরণ করে:

- সক্রিয় Rust ব্যবহারকারীদের জন্য নতুন edition ধাপে ধাপে আসা পরিবর্তনগুলোকে সহজে বোঝা যায় এমন একটি প্যাকেজে একত্র করে।
- যারা Rust ব্যবহার করেন না, তাদের জন্য নতুন edition জানান দেয় যে কিছু বড় অগ্রগতি যুক্ত হয়েছে, যার ফলে Rust-কে আরেকবার দেখে নেওয়া সার্থক হতে পারে।
- যারা Rust তৈরি করছেন, তাদের জন্য নতুন edition পুরো project-এর একটি মিলনবিন্দু হয়ে ওঠে।

এই লেখার সময় চারটি Rust edition পাওয়া যায়: Rust 2015, Rust 2018, Rust 2021 আর Rust 2024। এই বই Rust 2024 edition-এর idiom ব্যবহার করে লেখা।

_Cargo.toml_-এর `edition` key বোঝায় compiler আপনার কোডের জন্য কোন edition ব্যবহার করবে। key-টি না থাকলে backward compatibility-র কারণে Rust edition-এর মান হিসেবে `2015` ব্যবহার করে।

প্রতিটি project default 2015 edition ছাড়া অন্য কোনো edition বেছে নিতে পারে। edition-এ অসামঞ্জস্যপূর্ণ পরিবর্তন থাকতে পারে, যেমন এমন একটি নতুন keyword যোগ করা, যা কোডের identifier-এর সঙ্গে সংঘাত করে। তবে আপনি সেই পরিবর্তনগুলো বেছে না নিলে, ব্যবহৃত Rust compiler-এর version upgrade করলেও আপনার কোড compile হতে থাকবে।

সব Rust compiler version সেই compiler release-এর আগে থাকা যেকোনো edition support করে, এবং যেকোনো support করা edition-এর crate একসঙ্গে link করতে পারে। edition-এর পরিবর্তন শুধু compiler শুরুতে কোড কীভাবে parse করে তা প্রভাবিত করে। তাই আপনি যদি Rust 2015 ব্যবহার করেন আর আপনার কোনো dependency Rust 2018 ব্যবহার করে, তাহলে আপনার project compile হবে এবং সেই dependency ব্যবহার করতে পারবে। বিপরীত পরিস্থিতি, যেখানে আপনার project Rust 2018 আর একটি dependency Rust 2015 ব্যবহার করে, সেটিও কাজ করে।

স্পষ্ট করে বলি: বেশিরভাগ feature সব edition-এ পাওয়া যাবে। যেকোনো Rust edition ব্যবহারকারী developer-রা নতুন stable release হলে উন্নতিগুলো পেতে থাকবেন। তবে কিছু ক্ষেত্রে, মূলত নতুন keyword যোগ হলে, কিছু নতুন feature শুধু পরের edition-এ পাওয়া যেতে পারে। এমন feature-এর সুবিধা নিতে চাইলে আপনাকে edition বদলাতে হবে।

আরও বিস্তারিত জানতে [_The Rust Edition Guide_][edition-guide] দেখুন। এটি একটি পূর্ণাঙ্গ বই, যা edition-গুলোর পার্থক্য তালিকাভুক্ত করে এবং `cargo fix` দিয়ে কীভাবে স্বয়ংক্রিয়ভাবে আপনার কোড নতুন edition-এ upgrade করতে হয় তা ব্যাখ্যা করে।

[edition-guide]: https://doc.rust-lang.org/stable/edition-guide
